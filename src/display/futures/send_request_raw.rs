// MIT/Apache2 License

use crate::display::{AsyncDisplay, PendingReply, PendingRequest, RequestInfo};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// The future returned by `send_request_raw_async`. This polls the `AsyncDisplay` instance until it
/// returns properly.
#[derive(Debug)]
#[must_use = "futures do nothing unless you poll or .await them"]
pub struct SendRequestRawFuture<'a, D: ?Sized> {
    display: &'a mut D,
    is_finished: bool,
}

impl<'a, D: ?Sized> Unpin for SendRequestRawFuture<'a, D> {}

impl<'a, D: AsyncDisplay + ?Sized> SendRequestRawFuture<'a, D> {
    #[inline]
    pub(crate) fn run(display: &'a mut D, request: RequestInfo) -> Self {
        // begin the send request process
        display.begin_send_request_raw(request);
        Self {
            display,
            is_finished: false,
        }
    }

    /// Consumes this future and returns the display we are currently sending a request to.
    #[inline]
    pub(crate) fn cannibalize(self) -> &'a mut D {
        self.display
    }
}

impl<'a, D: AsyncDisplay + ?Sized> Future for SendRequestRawFuture<'a, D> {
    type Output = crate::Result<u16>;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<crate::Result<u16>> {
        if self.is_finished {
            panic!("Attempted to poll future after completion");
        }
        let res = self.display.poll_send_request_raw(cx);
        if res.is_ready() {
            self.is_finished = true;
        }
        res
    }
}
