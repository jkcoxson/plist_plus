// jkcoxson

use log::info;

use crate::{unsafe_bindings, Plist, PlistType};

impl Plist {
    /// Creates a new plist with an empty array
    pub fn new_array() -> Plist {
        info!("Generating new array plist");
        unsafe { unsafe_bindings::plist_new_array() }.into()
    }
    /// Returns the number of elements in the array
    pub fn array_get_size(&self) -> Result<u32, ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        info!("Getting array size");
        Ok(unsafe { unsafe_bindings::plist_array_get_size(self.plist_t) })
    }
    /// Returns the element at the given index
    pub fn array_get_item(&self, index: u32) -> Result<Plist, ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        info!("Getting array item");
        Ok(unsafe { unsafe_bindings::plist_array_get_item(self.plist_t, index) }.into())
    }
    /// Gets the index of an array item
    pub fn array_get_item_index(&self) -> Result<u32, ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        info!("Getting array item index");
        Ok(unsafe {
            unsafe_bindings::plist_array_get_item_index(self.plist_t) // ???
        })
    }
    /// Sets an array item at the given index
    pub fn array_set_item(&mut self, item: Plist, index: u32) -> Result<(), ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        info!("Setting array item");
        unsafe { unsafe_bindings::plist_array_set_item(self.plist_t, item.plist_t, index) };
        self.dependent_plists.push(item.plist_t);
        item.false_drop();
        Ok(())
    }
    /// Adds an item to the array
    pub fn array_append_item(&mut self, item: Plist) -> Result<(), ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        info!("Appending array item");
        unsafe { unsafe_bindings::plist_array_append_item(self.plist_t, item.plist_t) };
        self.dependent_plists.push(item.plist_t);
        item.false_drop();
        Ok(())
    }
    /// Inserts an item into the array at a given index
    pub fn array_insert_item(&mut self, item: Plist, index: u32) -> Result<(), ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        info!("Inserting array item");
        unsafe { unsafe_bindings::plist_array_insert_item(self.plist_t, item.plist_t, index) }
        self.dependent_plists.push(item.plist_t);
        item.false_drop();
        Ok(())
    }
    /// Removes an item from the array at a given index
    pub fn array_remove_item(&self, index: u32) -> Result<(), ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        info!("Removing array item");
        unsafe { unsafe_bindings::plist_array_remove_item(self.plist_t, index) };
        Ok(())
    }
    /// Removes one self from an array
    pub fn array_item_remove(&self) -> Result<(), ()> {
        info!("Removing array item");
        unsafe { unsafe_bindings::plist_array_item_remove(self.plist_t) }
        Ok(())
    }
}
