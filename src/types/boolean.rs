// jkcoxson

use log::trace;

use crate::error::PlistError;
use crate::unsafe_bindings;
use crate::Plist;

impl Plist {
    /// Returns a plist with bool type
    pub fn new_bool(bool: bool) -> Plist {
        trace!("Generating new bool plist");
        unsafe {
            unsafe_bindings::plist_new_bool(match bool {
                true => 1,
                false => 0,
            })
        }
        .into()
    }
    /// Returns the value of the bool
    pub fn get_bool_val(&self) -> Result<bool, PlistError> {
        if self.plist_type != self.get_node_type() {
            return Err(PlistError::InvalidArg);
        }
        let mut val = unsafe { std::mem::zeroed() };
        Ok(unsafe {
            unsafe_bindings::plist_get_bool_val(self.plist_t, &mut val);
            !matches!(val, 0)
        })
    }
    /// Sets a plist to type bool with the given value
    pub fn set_bool_val(&self, val: bool) {
        let val = if val { 1 } else { 0 };
        trace!("Setting bool value");
        unsafe { unsafe_bindings::plist_set_bool_val(self.plist_t, val) }
    }
}

impl TryFrom<Plist> for bool {
    type Error = PlistError;
    fn try_from(plist: Plist) -> Result<Self, Self::Error> {
        plist.get_bool_val()
    }
}

impl From<bool> for Plist {
    fn from(val: bool) -> Self {
        Plist::new_bool(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool() {
        let p = Plist::new_bool(false);
        p.set_bool_val(true);
        assert_eq!(p.get_bool_val().unwrap(), true);
    }
}
