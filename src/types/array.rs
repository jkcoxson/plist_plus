// jkcoxson

use log::trace;

use crate::{error::PlistError, unsafe_bindings, Plist, PlistType};

impl Plist {
    /// Creates a new plist with an empty array
    pub fn new_array() -> Plist {
        trace!("Generating new array plist");
        unsafe { unsafe_bindings::plist_new_array() }.into()
    }
    /// Returns the number of elements in the array
    pub fn array_get_size(&self) -> Result<u32, PlistError> {
        if self.plist_type != PlistType::Array {
            return Err(PlistError::InvalidArg);
        }
        trace!("Getting array size");
        Ok(unsafe { unsafe_bindings::plist_array_get_size(self.plist_t) })
    }
    /// Returns the element at the given index
    pub fn array_get_item(&self, index: u32) -> Result<Plist, PlistError> {
        if self.plist_type != PlistType::Array {
            return Err(PlistError::InvalidArg);
        }
        trace!("Getting array item");
        let mut plist: Plist = unsafe { unsafe_bindings::plist_array_get_item(self.plist_t, index) }.into();
        plist.false_drop = true;

        Ok(plist)
    }
    /// Gets the index of an array item
    pub fn array_get_item_index(&self) -> Result<u32, PlistError> {
        if self.plist_type != PlistType::Array {
            return Err(PlistError::InvalidArg);
        }
        trace!("Getting array item index");
        Ok(unsafe {
            unsafe_bindings::plist_array_get_item_index(self.plist_t) // ???
        })
    }
    /// Sets an array item at the given index
    pub fn array_set_item(&mut self, item: Plist, index: u32) -> Result<(), PlistError> {
        if self.plist_type != PlistType::Array {
            return Err(PlistError::InvalidArg);
        }
        trace!("Setting array item");
        unsafe { unsafe_bindings::plist_array_set_item(self.plist_t, item.plist_t, index) };
        item.false_drop();
        Ok(())
    }
    /// Adds an item to the array
    pub fn array_append_item(&mut self, item: Plist) -> Result<(), PlistError> {
        if self.plist_type != PlistType::Array {
            return Err(PlistError::InvalidArg);
        }
        trace!("Appending array item");
        unsafe { unsafe_bindings::plist_array_append_item(self.plist_t, item.plist_t) };
        item.false_drop();
        Ok(())
    }
    /// Inserts an item into the array at a given index
    pub fn array_insert_item(&mut self, item: Plist, index: u32) -> Result<(), PlistError> {
        if self.plist_type != PlistType::Array {
            return Err(PlistError::InvalidArg);
        }
        trace!("Inserting array item");
        unsafe { unsafe_bindings::plist_array_insert_item(self.plist_t, item.plist_t, index) }
        item.false_drop();
        Ok(())
    }
    /// Removes an item from the array at a given index
    pub fn array_remove_item(&self, index: u32) -> Result<(), PlistError> {
        if self.plist_type != PlistType::Array {
            return Err(PlistError::InvalidArg);
        }
        trace!("Removing array item");
        unsafe { unsafe_bindings::plist_array_remove_item(self.plist_t, index) };
        Ok(())
    }
    /// Removes one self from an array
    pub fn array_item_remove(&self) -> Result<(), PlistError> {
        trace!("Removing array item");
        unsafe { unsafe_bindings::plist_array_item_remove(self.plist_t) }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_test() {
        let b = Plist::new_bool(true);
        let mut p = Plist::new_array();
        p.array_append_item(b).unwrap();
        assert!(p.array_get_item(0).unwrap().get_bool_val().unwrap());
    }
}
