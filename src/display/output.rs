// MIT/Apache2 License

use super::{Connection, PendingRequestFlags, RequestCookie, RequestWorkaround, EXT_KEY_SIZE};
use crate::{util::cycled_zeroes, Fd, Request};
use alloc::{string::ToString, vec, vec::Vec};
use core::iter;
use tinyvec::TinyVec;

#[inline]
fn string_as_array_bytes(s: &str) -> [u8; EXT_KEY_SIZE] {
    let mut bytes: [u8; EXT_KEY_SIZE] = [0; EXT_KEY_SIZE];
    if s.len() > EXT_KEY_SIZE {
        bytes.copy_from_slice(&s.as_bytes()[..24]);
    } else {
        (&mut bytes[..s.len()]).copy_from_slice(s.as_bytes());
    }
    bytes
}

impl<Conn: Connection> super::Display<Conn> {
    #[allow(clippy::single_match_else)]
    #[inline]
    fn get_ext_opcode(&mut self, extname: &'static str) -> crate::Result<u8> {
        let sarr = string_as_array_bytes(extname);
        match self.extensions.get(&sarr) {
            Some(code) => Ok(*code),
            None => {
                let code = self
                    .query_extension_immediate(extname.to_string())
                    .map_err(|_| crate::BreadError::ExtensionNotPresent(extname.into()))?
                    .major_opcode;
                self.extensions.insert(sarr, code);
                Ok(code)
            }
        }
    }

    #[cfg(feature = "async")]
    #[inline]
    async fn get_ext_opcode_async(&mut self, extname: &'static str) -> crate::Result<u8> {
        let sarr = string_as_array_bytes(extname);
        match self.extensions.get(&sarr) {
            Some(code) => Ok(*code),
            None => {
                let code = self
                    .query_extension_immediate_async(extname.to_string())
                    .await
                    .map_err(|_| crate::BreadError::ExtensionNotPresent(extname.into()))?
                    .major_opcode;
                self.extensions.insert(sarr, code);
                Ok(code)
            }
        }
    }

    #[inline]
    fn encode_request<R: Request>(
        &mut self,
        req: &R,
        ext_opcode: Option<u8>,
    ) -> (u64, TinyVec<[u8; 32]>) {
        let sequence = self.request_number;
        self.request_number += 1;

        // write to bytes
        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(req.size());

        let mut len = req.as_bytes(&mut bytes);
        log::trace!("len is {} bytes long", len);

        // pad to a multiple of four bytes if we can
        let remainder = len % 4;
        if remainder != 0 {
            let extend_by = 4 - remainder;
            bytes.extend(iter::once(0).cycle().take(extend_by));
            len += extend_by;
            debug_assert_eq!(len % 4, 0);
            log::trace!("Extended length is now {}", len);
        }

        match ext_opcode {
            None => {
                // First byte is opcode
                // Second byte is minor opcode (ignored for now)
                log::debug!("Request has opcode {}", R::OPCODE);
                bytes[0] = R::OPCODE;
            }
            Some(extension) => {
                // First byte is extension opcode
                // Second byte is regular opcode
                bytes[0] = extension;
                bytes[1] = R::OPCODE;
            }
        }

        // Third and fourth are length
        let x_len = len / 4;
        log::trace!("xlen is {}", x_len);
        let len_bytes = x_len.to_ne_bytes();
        bytes[2] = len_bytes[0];
        bytes[3] = len_bytes[1];

        bytes.truncate(len);

        log::trace!("Request has bytes {:?}", &bytes);

        let mut flags = PendingRequestFlags {
            expects_fds: R::REPLY_EXPECTS_FDS,
            ..Default::default()
        };

        // there exists a very enraging bug in the X server, where certain GLX requests have the wrong size
        // attached to them. this bug has become so widespread that we have to assume that it exists in all
        // versions of the X server.
        //
        // to summarize, the X server makes an arithmatic error when calculating the length of the reply of
        // requests GetFBConfigs and VendorPrivate. in these replies, they forget to multiply the length value
        // by two. therefore, on the input end, we have to multiply it by two ourselves.
        //
        // the reason why this is enraging is because i just came out of combing through the codebase of both
        // breadglx and breadx for why this would happen, when it turns out the answer is just "X server broke,
        // multiply value by two, lol". the rage i feel that this bug is now baked into the X protocol is
        // immeasurable, but not immeasurable enough for me to switch to Wayland
        match (
            R::EXTENSION,
            R::OPCODE,
            bytes.get(32..36).map(|a| {
                let mut arr: [u8; 4] = [0; 4];
                arr.copy_from_slice(a);
                u32::from_ne_bytes(arr)
            }),
        ) {
            (Some("GLX"), 17, Some(0x10004)) | (Some("GLX"), 21, _) => {
                log::debug!("Applying GLX FbConfig workaround to request");
                flags.workaround = RequestWorkaround::GlxFbconfigBug;
            }
            _ => (),
        }

        self.expect_reply(sequence, flags);

        (sequence, bytes)
    }

    #[inline]
    pub fn send_request_internal<R: Request>(
        &mut self,
        mut req: R,
    ) -> crate::Result<RequestCookie<R>> {
        let ext_opcode = match R::EXTENSION {
            None => None,
            Some(ext) => Some(self.get_ext_opcode(ext)?),
        };
        let (sequence, bytes): (u64, TinyVec<[u8; 32]>) = self.encode_request(&req, ext_opcode);

        let mut _dummy: Vec<Fd> = vec![];
        let fds = match req.file_descriptors() {
            Some(fds) => fds,
            None => &mut _dummy,
        };

        self.connection.send_packet(&bytes, fds)?;
        Ok(RequestCookie::from_sequence(sequence))
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn send_request_internal_async<R: Request>(
        &mut self,
        mut req: R,
    ) -> crate::Result<RequestCookie<R>> {
        let ext_opcode = match R::EXTENSION {
            None => None,
            Some(ext) => Some(self.get_ext_opcode_async(ext).await?),
        };
        let (sequence, bytes) = self.encode_request(&req, ext_opcode);

        let mut _dummy: Vec<Fd> = vec![];
        let fds = match req.file_descriptors() {
            Some(fds) => fds,
            None => &mut _dummy,
        };

        self.connection.send_packet_async(&bytes, fds).await?;
        Ok(RequestCookie::from_sequence(sequence))
    }
}
