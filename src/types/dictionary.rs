// jkcoxson

use std::ffi::CString;

use crate::{debug, Plist, PlistType, unsafe_bindings};

impl Plist {
    pub fn new_dict() -> Plist {
        debug!("Generating new dictionary plist");
        unsafe { unsafe_bindings::plist_new_dict() }.into()
    }
    pub fn dict_get_size(&self) -> Result<u32, ()> {
        if self.plist_type != PlistType::Dictionary {
            return Err(());
        }
        debug!("Getting dict size");
        Ok(unsafe { unsafe_bindings::plist_dict_get_size(self.plist_t) })
    }
    pub fn dict_get_item_key(&self) -> Result<String, ()> {
        if self.plist_type != PlistType::Dictionary {
            return Err(());
        }
        let mut key = std::ptr::null_mut();
        debug!("Getting dict item key");
        unsafe { unsafe_bindings::plist_dict_get_item_key(self.plist_t, &mut key) };
        debug!("Converting key to string");
        let key = unsafe { std::ffi::CStr::from_ptr(key).to_string_lossy().into_owned() };
        Ok(key)
    }
    pub fn dict_get_item(&self, key: &str) -> Result<Plist, ()> {
        if self.plist_type != PlistType::Dictionary {
            return Err(());
        }
        let key_c_string = CString::new(key).unwrap();
        debug!("Getting dict item");
        Ok(
            unsafe { unsafe_bindings::plist_dict_get_item(self.plist_t, key_c_string.as_ptr()) }
                .into(),
        )
    }
    pub fn dict_item_get_key(&self) -> Result<Plist, ()> {
        if self.plist_type != PlistType::Dictionary {
            return Err(());
        }
        debug!("Getting dict item key");
        Ok(unsafe { unsafe_bindings::plist_dict_item_get_key(self.plist_t) }.into())
    }
    pub fn dict_set_item(&mut self, key: &str, item: Plist) -> Result<(), ()> {
        let key = CString::new(key).unwrap();
        if self.plist_type != PlistType::Dictionary {
            return Err(());
        }
        debug!("Setting dict item");
        unsafe { unsafe_bindings::plist_dict_set_item(self.plist_t, key.as_ptr(), item.plist_t) }
        self.dependent_plists.push(item.plist_t);
        item.false_drop();
        Ok(())
    }
    pub fn dict_insert_item(&mut self, key: &str, item: Plist) -> Result<(), ()> {
        let key = CString::new(key).unwrap();
        if self.plist_type != PlistType::Dictionary {
            return Err(());
        }
        debug!("Inserting dict item");
        unsafe {
            unsafe_bindings::plist_dict_insert_item(
                self.plist_t,
                key.as_ptr() as *const i8,
                item.plist_t,
            )
        }
        self.dependent_plists.push(item.plist_t);
        item.false_drop();
        Ok(())
    }
    pub fn dict_remove_item(&self, key: &str) -> Result<(), ()> {
        let key = CString::new(key).unwrap();
        if self.plist_type != PlistType::Dictionary {
            return Err(());
        }
        debug!("Removing dict item");
        unsafe { unsafe_bindings::plist_dict_remove_item(self.plist_t, key.as_ptr() as *const i8) }
        Ok(())
    }
    pub fn dict_merge(&mut self, dict: Plist) -> Result<(), ()> {
        if self.plist_type != PlistType::Dictionary {
            return Err(());
        }
        debug!("Merging dict");
        unsafe { unsafe_bindings::plist_dict_merge(&mut self.plist_t, dict.plist_t) }
        self.dependent_plists.push(dict.plist_t);
        dict.false_drop();
        Ok(())
    }
}