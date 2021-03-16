// MIT/Apache2 License

#![cfg(feature = "xkb")]

use super::auto::{
    xkb::{
        SaActionMessage, SaDeviceBtn, SaDeviceValuator, SaIsoLock, SaLatchGroup, SaLatchMods,
        SaLockControls, SaLockDeviceBtn, SaLockGroup, SaLockMods, SaLockPtrBtn, SaMovePtr,
        SaNoAction, SaPtrBtn, SaRedirectKey, SaSetControls, SaSetGroup, SaSetMods, SaSetPtrDflt,
        SaSwitchScreen, SaTerminate, SaType,
    },
    AsByteSequence,
};

/// An action generated by XKB.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Action {
    NoAction(SaNoAction),
    SetMods(SaSetMods),
    LatchMods(SaLatchMods),
    LockMods(SaLockMods),
    SetGroup(SaSetGroup),
    LatchGroup(SaLatchGroup),
    LockGroup(SaLockGroup),
    MovePtr(SaMovePtr),
    PtrBtn(SaPtrBtn),
    LockPtrBtn(SaLockPtrBtn),
    SetPtrDflt(SaSetPtrDflt),
    IsoLock(SaIsoLock),
    Terminate(SaTerminate),
    SwitchScreen(SaSwitchScreen),
    SetControls(SaSetControls),
    LockControls(SaLockControls),
    ActionMessage(SaActionMessage),
    RedirectKey(SaRedirectKey),
    DeviceBtn(SaDeviceBtn),
    LockDeviceBtn(SaLockDeviceBtn),
    DeviceValuator(SaDeviceValuator),
}

impl Default for Action {
    #[inline]
    fn default() -> Self {
        Self::NoAction(Default::default())
    }
}

const ACTION_SIZE: usize = 8;

impl AsByteSequence for Action {
    #[inline]
    fn size(&self) -> usize {
        ACTION_SIZE
    }

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        match self {
            Self::NoAction(na) => na.as_bytes(bytes),
            Self::SetMods(sm) => sm.as_bytes(bytes),
            Self::LatchMods(lm) => lm.as_bytes(bytes),
            Self::LockMods(lm) => lm.as_bytes(bytes),
            Self::SetGroup(sg) => sg.as_bytes(bytes),
            Self::LatchGroup(lg) => lg.as_bytes(bytes),
            Self::LockGroup(lg) => lg.as_bytes(bytes),
            Self::MovePtr(mp) => mp.as_bytes(bytes),
            Self::PtrBtn(pn) => pn.as_bytes(bytes),
            Self::LockPtrBtn(lpb) => lpb.as_bytes(bytes),
            Self::SetPtrDflt(spd) => spd.as_bytes(bytes),
            Self::IsoLock(il) => il.as_bytes(bytes),
            Self::Terminate(t) => t.as_bytes(bytes),
            Self::SwitchScreen(ss) => ss.as_bytes(bytes),
            Self::SetControls(sc) => sc.as_bytes(bytes),
            Self::LockControls(lc) => lc.as_bytes(bytes),
            Self::ActionMessage(am) => am.as_bytes(bytes),
            Self::RedirectKey(rk) => rk.as_bytes(bytes),
            Self::DeviceBtn(db) => db.as_bytes(bytes),
            Self::LockDeviceBtn(ldb) => ldb.as_bytes(bytes),
            Self::DeviceValuator(dv) => dv.as_bytes(bytes),
        }
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let ty = SaType::from_bytes(bytes)?.0;

        let this = match ty {
            SaType::NoAction => Self::NoAction(SaNoAction::from_bytes(bytes)?.0),
            SaType::SetMods => Self::SetMods(SaSetMods::from_bytes(bytes)?.0),
            SaType::LatchMods => Self::LatchMods(SaLatchMods::from_bytes(bytes)?.0),
            SaType::LockMods => Self::LockMods(SaLockMods::from_bytes(bytes)?.0),
            SaType::SetGroup => Self::SetGroup(SaSetGroup::from_bytes(bytes)?.0),
            SaType::LatchGroup => Self::LatchGroup(SaLatchGroup::from_bytes(bytes)?.0),
            SaType::LockGroup => Self::LockGroup(SaLockGroup::from_bytes(bytes)?.0),
            SaType::MovePtr => Self::MovePtr(SaMovePtr::from_bytes(bytes)?.0),
            SaType::PtrBtn => Self::PtrBtn(SaPtrBtn::from_bytes(bytes)?.0),
            SaType::LockPtrBtn => Self::LockPtrBtn(SaLockPtrBtn::from_bytes(bytes)?.0),
            SaType::SetPtrDflt => Self::SetPtrDflt(SaSetPtrDflt::from_bytes(bytes)?.0),
            SaType::IsoLock => Self::IsoLock(SaIsoLock::from_bytes(bytes)?.0),
            SaType::Terminate => Self::Terminate(SaTerminate::from_bytes(bytes)?.0),
            SaType::SwitchScreen => Self::SwitchScreen(SaSwitchScreen::from_bytes(bytes)?.0),
            SaType::SetControls => Self::SetControls(SaSetControls::from_bytes(bytes)?.0),
            SaType::LockControls => Self::LockControls(SaLockControls::from_bytes(bytes)?.0),
            SaType::ActionMessage => Self::ActionMessage(SaActionMessage::from_bytes(bytes)?.0),
            SaType::RedirectKey => Self::RedirectKey(SaRedirectKey::from_bytes(bytes)?.0),
            SaType::DeviceBtn => Self::DeviceBtn(SaDeviceBtn::from_bytes(bytes)?.0),
            SaType::LockDeviceBtn => Self::LockDeviceBtn(SaLockDeviceBtn::from_bytes(bytes)?.0),
            SaType::DeviceValuator => Self::DeviceValuator(SaDeviceValuator::from_bytes(bytes)?.0),
        };

        Some((this, ACTION_SIZE))
    }
}
