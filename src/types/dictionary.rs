// jkcoxson

use std::{ffi::CString, os::raw::c_char};

use log::trace;

use crate::{error::PlistError, unsafe_bindings, Plist, PlistType};

impl Plist {
    /// Returns a plist with type dictionary
    /// This plist is empty
    pub fn new_dict() -> Plist {
        trace!("Generating new dictionary plist");
        unsafe { unsafe_bindings::plist_new_dict() }.into()
    }
    /// Returns the number of items contained in the plist dictionary
    pub fn dict_get_size(&self) -> Result<u32, PlistError> {
        if self.plist_type != PlistType::Dictionary {
            return Err(PlistError::InvalidArg);
        }
        trace!("Getting dict size");
        Ok(unsafe { unsafe_bindings::plist_dict_get_size(self.plist_t) })
    }
    /// Get the key associated with the item
    pub fn dict_get_item_key(&self) -> Result<String, PlistError> {
        if self.plist_type != PlistType::Dictionary {
            return Err(PlistError::InvalidArg);
        }
        let mut key = std::ptr::null_mut();
        trace!("Getting dict item key for {}", self.id);
        unsafe { unsafe_bindings::plist_dict_get_item_key(self.plist_t, &mut key) };
        trace!("Converting key to string");
        let key = unsafe { std::ffi::CStr::from_ptr(key).to_string_lossy().into_owned() };
        Ok(key)
    }
    /// Get the item associated with the key
    pub fn dict_get_item(&self, key: &str) -> Result<Plist, PlistError> {
        if self.plist_type != PlistType::Dictionary {
            return Err(PlistError::InvalidArg);
        }
        let key_c_string = CString::new(key).unwrap();
        trace!("Getting dict item for {}", self.id);
        let mut item: Plist =
            unsafe { unsafe_bindings::plist_dict_get_item(self.plist_t, key_c_string.as_ptr()) }
                .into();
        item.false_drop = true;
        Ok(item)
    }
    /// Get the key associated with self within a dictionary
    pub fn dict_item_get_key(&self) -> Result<Plist, PlistError> {
        trace!("Getting dict item key");
        Ok(unsafe { unsafe_bindings::plist_dict_item_get_key(self.plist_t) }.into())
    }
    pub fn dict_set_item(&mut self, key: &str, item: Plist) -> Result<(), PlistError> {
        let key = CString::new(key).unwrap();
        if self.plist_type != PlistType::Dictionary {
            return Err(PlistError::InvalidArg);
        }
        trace!("Setting dict item");
        unsafe { unsafe_bindings::plist_dict_set_item(self.plist_t, key.as_ptr(), item.plist_t) }
        item.false_drop();
        Ok(())
    }
    /// Inserts a new item into the dictionary
    /// The item must also be a plist
    pub fn dict_insert_item(&mut self, key: &str, item: Plist) -> Result<(), PlistError> {
        let key = CString::new(key).unwrap();
        if self.plist_type != PlistType::Dictionary {
            return Err(PlistError::InvalidArg);
        }
        trace!("Inserting dict item");
        unsafe {
            unsafe_bindings::plist_dict_set_item(
                self.plist_t,
                key.as_ptr() as *const c_char,
                item.plist_t,
            )
        }
        item.false_drop();
        Ok(())
    }
    /// Removes an item from the dictionary with a given key
    pub fn dict_remove_item(&self, key: &str) -> Result<(), PlistError> {
        let key = CString::new(key).unwrap();
        if self.plist_type != PlistType::Dictionary {
            return Err(PlistError::InvalidArg);
        }
        trace!("Removing dict item");
        unsafe {
            unsafe_bindings::plist_dict_remove_item(self.plist_t, key.as_ptr() as *const c_char)
        }
        Ok(())
    }
    /// Merges a dictionary into the current dictionary
    pub fn dict_merge(&mut self, dict: Plist) -> Result<(), PlistError> {
        if self.plist_type != PlistType::Dictionary {
            return Err(PlistError::InvalidArg);
        }
        trace!("Merging dict");
        unsafe { unsafe_bindings::plist_dict_merge(&mut self.plist_t, dict.plist_t) }
        dict.false_drop();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dict_test() {
        let b = Plist::new_bool(false);
        let mut p = Plist::new_dict();
        p.dict_set_item("b", b).unwrap();
        let b = p.dict_get_item("b").unwrap();
        assert_eq!(b.get_bool_val().unwrap(), false);
    }
}
