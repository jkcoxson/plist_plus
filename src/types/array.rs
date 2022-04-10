// jkcoxson

use crate::{debug, Plist, PlistType, unsafe_bindings};

impl Plist {
    pub fn new_array() -> Plist {
        debug!("Generating new array plist");
        unsafe { unsafe_bindings::plist_new_array() }.into()
    }
    pub fn array_get_size(&self) -> Result<u32, ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        debug!("Getting array size");
        Ok(unsafe { unsafe_bindings::plist_array_get_size(self.plist_t) })
    }
    pub fn array_get_item(&self, index: u32) -> Result<Plist, ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        debug!("Getting array item");
        Ok(unsafe { unsafe_bindings::plist_array_get_item(self.plist_t, index) }.into())
    }
    pub fn array_get_item_index(&self) -> Result<u32, ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        debug!("Getting array item index");
        Ok(unsafe {
            unsafe_bindings::plist_array_get_item_index(self.plist_t) // ???
        })
    }

    pub fn array_set_item(&mut self, item: Plist, index: u32) -> Result<(), ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        debug!("Setting array item");
        unsafe { unsafe_bindings::plist_array_set_item(self.plist_t, item.plist_t, index) };
        self.dependent_plists.push(item.plist_t);
        item.false_drop();
        Ok(())
    }
    pub fn array_append_item(&mut self, item: Plist) -> Result<(), ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        debug!("Appending array item");
        unsafe { unsafe_bindings::plist_array_append_item(self.plist_t, item.plist_t) };
        self.dependent_plists.push(item.plist_t);
        item.false_drop();
        Ok(())
    }
    pub fn array_insert_item(&mut self, item: Plist, index: u32) -> Result<(), ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        debug!("Inserting array item");
        unsafe { unsafe_bindings::plist_array_insert_item(self.plist_t, item.plist_t, index) }
        self.dependent_plists.push(item.plist_t);
        item.false_drop();
        Ok(())
    }
    pub fn array_remove_item(&self, index: u32) -> Result<(), ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        debug!("Removing array item");
        unsafe { unsafe_bindings::plist_array_remove_item(self.plist_t, index) };
        Ok(())
    }
    pub fn array_item_remove(&self) -> Result<(), ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        debug!("Removing array item");
        unsafe { unsafe_bindings::plist_array_item_remove(self.plist_t) }
        Ok(())
    }
}