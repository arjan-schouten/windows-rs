use crate::*;

pub unsafe trait Handle: Sized + PartialEq {
    fn is_invalid(&self) -> bool {
        *self == unsafe { std::mem::zeroed() }
    }

    fn ok(self) -> Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(Error::from_win32())
        }
    }
}