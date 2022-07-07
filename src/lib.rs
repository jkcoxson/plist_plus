#![allow(clippy::not_unsafe_ptr_arg_deref)]
use error::PlistError;
#[doc = include_str!("../README.md")]
use log::{trace, warn};
use rand::Rng;
use std::{ffi::CString, fmt::Formatter, os::raw::c_char};

pub mod error;
mod iterator;
mod types;
mod unsafe_bindings;

/// The main struct for the plist library
/// This struct contains a pointer to the C compatible structure
pub struct Plist {
    pub(crate) plist_t: unsafe_bindings::plist_t,
    pub plist_type: PlistType,
    pub(crate) dependent_plists: Vec<unsafe_bindings::plist_t>,
    pub(crate) id: u32,
}

unsafe impl Send for Plist {}
unsafe impl Sync for Plist {}

/// The type of a given plist
#[derive(PartialEq, Debug)]
pub enum PlistType {
    Boolean,
    Integer,
    Real,
    Date,
    Data,
    String,
    Array,
    Dictionary,
    Unknown,
    Key,
    Uid,
    None,
}

impl Plist {
    /// Returns a pointer to the underlying C compatible structure
    /// This is compatible with libraries such as libimobiledevice
    pub fn get_pointer(&self) -> *mut std::ffi::c_void {
        self.plist_t as *mut std::ffi::c_void
    }
    /// This takes a string in the form of XML and returns a Plist struct
    pub fn from_xml(xml: String) -> Result<Plist, PlistError> {
        let xml = match CString::new(xml) {
            Ok(s) => s,
            Err(_) => {
                warn!("Could not convert string to CString");
                return Err(PlistError::InvalidArg);
            }
        };
        let xml_len = std::convert::TryInto::try_into(xml.as_bytes().len()).unwrap();
        let mut plist_t = unsafe { std::mem::zeroed() };
        trace!("Parsing xml");
        unsafe {
            unsafe_bindings::plist_from_xml(xml.as_ptr() as *const c_char, xml_len, &mut plist_t)
        };
        Ok(plist_t.into())
    }
    /// This takes a string in the form of binary and returns a Plist struct
    pub fn from_bin(bin: Vec<u8>) -> Result<Plist, PlistError> {
        let mut plist_t = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::plist_from_bin(
                bin.as_ptr() as *const c_char,
                bin.len() as u32,
                &mut plist_t,
            )
        };
        if result != 0 {
            return Err(result.into());
        }
        Ok(plist_t.into())
    }
    pub fn from_memory(bin: Vec<u8>) -> Result<Plist, PlistError> {
        let mut plist_t = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::plist_from_memory(
                bin.as_ptr() as *const c_char,
                bin.len() as u32,
                &mut plist_t,
            )
        };
        if result != 0 {
            return Err(result.into());
        }
        Ok(plist_t.into())
    }
    /// This will back the plist to the plist it came from
    /// This is unsafe due to how the underlying C library works
    /// It will return a second copy of the plist, and should be false dropped if used
    /// # Safety
    /// Don't be stupid
    pub unsafe fn get_parent(self) -> Plist {
        trace!("Getting parent");
        unsafe_bindings::plist_get_parent(self.plist_t).into()
    }
    /// Gets the type of the plist from the C library
    pub fn get_node_type(&self) -> PlistType {
        trace!("Getting node type");
        unsafe { unsafe_bindings::plist_get_node_type(self.plist_t) }.into() // puts on sunglasses
    }
    /// Queries if the plist has a binary structure
    pub fn is_binary(&self) -> bool {
        let plist_data = unsafe { std::mem::zeroed() };
        let plist_len = unsafe { std::mem::zeroed() };
        trace!("Getting plist data");
        unsafe {
            unsafe_bindings::plist_get_data_val(self.plist_t, plist_data, plist_len);
        }
        trace!("Checking if plist is binary");
        !matches!(
            unsafe {
                unsafe_bindings::plist_is_binary(*plist_data, (*plist_len).try_into().unwrap())
            },
            0
        )
    }
    /// Traverses a list of plists
    /// Reimplimented from the C function because function overloading is evil
    pub fn access_path(self, plists: Vec<String>) -> Result<Plist, PlistError> {
        let mut current = self;
        let mut i = 0;
        while i < plists.len() {
            match current.plist_type {
                PlistType::Array => {
                    current = match current.array_get_item(i as u32) {
                        Ok(item) => item,
                        Err(_) => return Err(PlistError::InvalidArg),
                    };
                }
                PlistType::Dictionary => {
                    current = match current.dict_get_item(&plists[i]) {
                        Ok(item) => item,
                        Err(_) => return Err(PlistError::InvalidArg),
                    };
                }
                _ => {
                    return Err(PlistError::InvalidArg);
                }
            }
            i += 1;
        }
        Ok(current.plist_t.into())
    }

    /// Disposes of the Rust structure without calling the destructor of the C structure
    /// This is necessary when a function absorbs another plist.
    /// That way, the rest of the plist struct is dropped, but the data at the pointer is not.
    /// This prevents many segfaults, but may cause unknown memory leaks.
    /// Needs more research...
    pub fn false_drop(mut self) {
        trace!("False dropping {}", self.id);
        let replacement = unsafe { unsafe_bindings::plist_new_bool(0) };
        self.plist_t = replacement;
    }

    /// Compares two structs and determines if they are equal
    pub fn compare_node_values(node_l: Plist, node_r: Plist) -> bool {
        trace!("Comparing node values");
        matches!(
            unsafe { unsafe_bindings::plist_compare_node_value(node_l.plist_t, node_r.plist_t) }
                .to_string()
                .as_str(),
            "TRUE"
        )
    }

    pub fn get_display_value(&self) -> Result<String, PlistError> {
        let mut to_return;
        match self.plist_type {
            PlistType::Boolean => {
                to_return = self.get_bool_val()?.to_string();
            }
            PlistType::Integer => {
                to_return = self.get_uint_val()?.to_string();
            }
            PlistType::Real => {
                to_return = self.get_real_val()?.to_string();
            }
            PlistType::Data => {
                to_return = format!("{:?}", self.get_data_val()?);
            }
            PlistType::Date => {
                todo!();
            }
            PlistType::String => {
                to_return = self.get_string_val()?;
            }
            PlistType::Array => {
                to_return = "[".to_string();
                for item in self.clone().into_iter() {
                    to_return = format!("{}{}", to_return, item.plist.get_display_value()?);
                }
                to_return = format!("{}]", to_return);
            }
            PlistType::Dictionary => {
                to_return = "{ ".to_string();
                for line in self.clone().into_iter() {
                    to_return = format!(
                        "{}{}: {}, ",
                        to_return,
                        line.key.unwrap(),
                        line.plist.get_display_value()?
                    );
                }
                // Chop off the last comma and space
                to_return = format!(
                    "{} }}",
                    to_return
                        .chars()
                        .take(to_return.len() - 2)
                        .collect::<String>()
                );
            }
            PlistType::Uid => {
                todo!();
            }
            PlistType::Key => {
                todo!();
            }
            PlistType::Unknown => {
                to_return = "Unknown".to_string();
            }
            PlistType::None => {
                to_return = "None".to_string();
            }
        }

        Ok(to_return)
    }
}

impl From<unsafe_bindings::plist_t> for Plist {
    fn from(plist_t: unsafe_bindings::plist_t) -> Self {
        let mut rng = rand::thread_rng();
        let id = rng.gen::<u32>();
        trace!("Creating plist from plist_t with id {}", id);
        Plist {
            plist_t,
            plist_type: unsafe { unsafe_bindings::plist_get_node_type(plist_t) }.into(),
            dependent_plists: Vec::new(),
            id,
        }
    }
}

impl From<Plist> for String {
    fn from(plist: Plist) -> Self {
        let plist_t = plist.plist_t;
        let mut plist_data = std::ptr::null_mut();
        let mut plist_size = 0;
        trace!("Converting plist to XML data");
        unsafe {
            unsafe_bindings::plist_to_xml(plist_t, &mut plist_data, &mut plist_size);
        }
        trace!("Assembling XML data");
        let plist_data = unsafe {
            std::slice::from_raw_parts(plist_data as *const u8, plist_size.try_into().unwrap())
        };
        let plist_data = std::str::from_utf8(plist_data).unwrap();

        String::from(plist_data)
    }
}

impl ToString for Plist {
    fn to_string(&self) -> String {
        let mut plist_data = std::ptr::null_mut();
        let mut plist_size = 0;
        trace!("Converting plist to XML data");
        unsafe {
            unsafe_bindings::plist_to_xml(self.plist_t, &mut plist_data, &mut plist_size);
        }
        trace!("Assembling XML data");
        let plist_data = unsafe {
            std::slice::from_raw_parts(plist_data as *const u8, plist_size.try_into().unwrap())
        };
        let plist_data = std::str::from_utf8(plist_data).unwrap();

        String::from(plist_data)
    }
}

impl From<Plist> for Vec<u8> {
    fn from(plist: Plist) -> Self {
        let plist_t = plist.plist_t;
        let mut plist_data = std::ptr::null_mut();
        let mut plist_size = 0;
        trace!("Converting plist to binary data");
        unsafe {
            unsafe_bindings::plist_to_bin(plist_t, &mut plist_data, &mut plist_size);
        }
        trace!("Assembling binary data");
        let plist_data = unsafe {
            std::slice::from_raw_parts(plist_data as *const u8, plist_size.try_into().unwrap())
        };

        plist_data.to_vec()
    }
}

impl Clone for Plist {
    fn clone(&self) -> Self {
        trace!("Cloning plist");
        let plist_t = unsafe { unsafe_bindings::plist_copy(self.plist_t) };
        trace!("Getting type of cloned plist");
        plist_t.into()
    }
}

impl Clone for PlistType {
    fn clone(&self) -> Self {
        match self {
            PlistType::Array => PlistType::Array,
            PlistType::Boolean => PlistType::Boolean,
            PlistType::Data => PlistType::Data,
            PlistType::Date => PlistType::Date,
            PlistType::Dictionary => PlistType::Dictionary,
            PlistType::Integer => PlistType::Integer,
            PlistType::Real => PlistType::Real,
            PlistType::String => PlistType::String,
            PlistType::Uid => PlistType::Uid,
            PlistType::Unknown => PlistType::Unknown,
            PlistType::Key => PlistType::Key,
            PlistType::None => PlistType::None,
        }
    }
}

impl std::fmt::Debug for Plist {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let plist_data = self.to_string();
        write!(f, "{:?}: {}", self.plist_type, plist_data)
    }
}

impl Drop for Plist {
    fn drop(&mut self) {
        trace!("Dropping plist {}", self.id);
        // Dependent plists should be freed automatically because this object is being dropped, right?
        if self.plist_t as u8 == 0 {
            warn!("Plist has already been freed");
            return;
        }
        unsafe { unsafe_bindings::plist_free(self.plist_t) }
        trace!("Plist dropped");
    }
}

impl From<u32> for PlistType {
    fn from(i: u32) -> Self {
        match i {
            0 => PlistType::Boolean,
            1 => PlistType::Integer,
            2 => PlistType::Real,
            3 => PlistType::String,
            4 => PlistType::Array,
            5 => PlistType::Dictionary,
            6 => PlistType::Date,
            7 => PlistType::Data,
            8 => PlistType::Key,
            9 => PlistType::Uid,
            10 => PlistType::None,
            _ => PlistType::Unknown,
        }
    }
}

impl From<PlistType> for String {
    fn from(plist_type: PlistType) -> String {
        match plist_type {
            PlistType::Boolean => "Boolean".to_string(),
            PlistType::Integer => "Integer".to_string(),
            PlistType::Real => "Real".to_string(),
            PlistType::Date => "Date".to_string(),
            PlistType::Data => "Data".to_string(),
            PlistType::String => "String".to_string(),
            PlistType::Array => "Array".to_string(),
            PlistType::Dictionary => "Dictionary".to_string(),
            PlistType::Unknown => "Unknown".to_string(),
            PlistType::Key => "Key".to_string(),
            PlistType::Uid => "Uid".to_string(),
            PlistType::None => "None".to_string(),
        }
    }
}
