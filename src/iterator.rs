// jkcoxson

use libc::c_void;
use log::trace;

use crate::{unsafe_bindings, Plist, PlistType};

pub struct PlistIterator {
    iter_pointer: *mut c_void,
    plist: Plist,
}

pub struct PlistItem {
    pub plist: Plist,
    pub key: Option<String>,
}

impl IntoIterator for Plist {
    type Item = PlistItem;
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
    type Item = PlistItem;

    fn next(&mut self) -> Option<Self::Item> {
        match self.plist.plist_type {
            PlistType::Array => {
                let to_fill = unsafe { std::mem::zeroed() };
                trace!("Getting next item in array");
                unsafe {
                    unsafe_bindings::plist_array_next_item(
                        self.plist.plist_t,
                        self.iter_pointer,
                        to_fill,
                    )
                };
                if to_fill.is_null() {
                    trace!("No more items in array");
                    None
                } else {
                    trace!("Getting type of next item in array");
                    Some(PlistItem {
                        plist: unsafe { *to_fill }.into(),
                        key: None,
                    })
                }
            }
            PlistType::Dictionary => {
                let mut key = unsafe { std::mem::zeroed() };
                let mut to_fill = unsafe { std::mem::zeroed() };
                trace!("Getting next item in dictionary");
                unsafe {
                    unsafe_bindings::plist_dict_next_item(
                        self.plist.plist_t,
                        self.iter_pointer,
                        &mut key,
                        &mut to_fill,
                    )
                };
                if to_fill.is_null() {
                    trace!("No more items in dictionary");
                    None
                } else {
                    let key_str = unsafe {
                        std::ffi::CString::from_raw(key)
                            .to_str()
                            .unwrap()
                            .to_string()
                    };
                    trace!("Getting type of next item in dictionary");
                    Some(PlistItem {
                        plist: to_fill.into(),
                        key: Some(key_str),
                    })
                }
            }
            _ => panic!("Cannot iterate over non-array or non-dictionary plist"),
        }
    }
}

impl Drop for PlistIterator {
    fn drop(&mut self) {
        // Free the pointer
        unsafe {
            libc::free(self.iter_pointer);
        }
    }
}
