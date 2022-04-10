// jkcoxson

use libc::c_void;

use crate::{debug, unsafe_bindings, Plist, PlistType};

pub struct PlistIterator {
    iter_pointer: *mut c_void,
    plist: Plist,
}

impl IntoIterator for Plist {
    type Item = Plist;
    type IntoIter = PlistIterator;

    fn into_iter(self) -> Self::IntoIter {
        let mut pointer = unsafe { std::mem::zeroed() };
        match self.plist_type.clone() {
            PlistType::Array => unsafe {
                unsafe_bindings::plist_array_new_iter(self.plist_t, &mut pointer)
            },
            PlistType::Dictionary => unsafe {
                unsafe_bindings::plist_dict_new_iter(self.plist_t, &mut pointer)
            },
            _ => panic!("Cannot iterate over non-array or non-dictionary plist"),
        };

        PlistIterator {
            iter_pointer: pointer,
            plist: self,
        }
    }
}

impl Iterator for PlistIterator {
    type Item = Plist;

    fn next(&mut self) -> Option<Self::Item> {
        match self.plist.plist_type {
            PlistType::Array => {
                let to_fill = unsafe { std::mem::zeroed() };
                debug!("Getting next item in array");
                unsafe {
                    unsafe_bindings::plist_array_next_item(
                        self.plist.plist_t,
                        self.iter_pointer,
                        to_fill,
                    )
                };
                if to_fill.is_null() {
                    debug!("No more items in array");
                    None
                } else {
                    debug!("Getting type of next item in array");
                    Some(unsafe { *to_fill }.into())
                }
            }
            PlistType::Dictionary => {
                let mut key = unsafe { std::mem::zeroed() };
                let mut to_fill = unsafe { std::mem::zeroed() };
                debug!("Getting next item in dictionary");
                unsafe {
                    unsafe_bindings::plist_dict_next_item(
                        self.plist.plist_t,
                        self.iter_pointer,
                        &mut key,
                        &mut to_fill,
                    )
                };
                if to_fill.is_null() {
                    debug!("No more items in dictionary");
                    None
                } else {
                    let _key_str =
                        unsafe { std::ffi::CStr::from_ptr(key).to_string_lossy().into_owned() };
                    debug!("Getting type of next item in dictionary");
                    Some(to_fill.into()) // yeet
                }
            }
            _ => panic!("Cannot iterate over non-array or non-dictionary plist"),
        }
    }
}
