use std::{ffi::CString, fmt::Formatter};

use rand::Rng;

#[doc = include_str!("../README.md")]

mod unsafe_bindings;
mod debug;
mod types;

pub struct Plist {
    pub(crate) plist_t: unsafe_bindings::plist_t,
    pub plist_type: PlistType,
    pub(crate) dependent_plists: Vec<unsafe_bindings::plist_t>,
    pub(crate) id: u32,
}

unsafe impl Send for Plist {}
unsafe impl Sync for Plist {}

pub struct PlistArrayIter {
    plist_array_iter: unsafe_bindings::plist_array_iter,
    plist: Plist,
}

unsafe impl Send for PlistDictIter {}
unsafe impl Sync for PlistDictIter {}

pub struct PlistDictIter {
    plist_dict_iter: unsafe_bindings::plist_dict_iter,
    plist: Plist,
}

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

impl Plist {
    pub fn from_xml(xml: String) -> Result<Plist, ()> {
        let xml = match CString::new(xml) {
            Ok(s) => s,
            Err(_) => {
                debug!("Could not convert string to CString");
                return Err(());
            }
        };
        let xml_len = std::convert::TryInto::try_into(xml.as_bytes().len()).unwrap();
        let mut plist_t = unsafe { std::mem::zeroed() };
        debug!("Parsing xml");
        unsafe {
            unsafe_bindings::plist_from_xml(xml.as_ptr() as *const i8, xml_len, &mut plist_t)
        };
        Ok(plist_t.into())
    }
    pub fn get_parent(&self) -> Plist {
        debug!("Getting parent");
        unsafe { unsafe_bindings::plist_get_parent(self.plist_t) }.into()
    }
    pub fn get_node_type(&self) -> PlistType {
        debug!("Getting node type");
        unsafe { unsafe_bindings::plist_get_node_type(self.plist_t) }.into() // puts on sunglasses
    }
    pub fn is_binary(&self) -> bool {
        let plist_data = unsafe { std::mem::zeroed() };
        let plist_len = unsafe { std::mem::zeroed() };
        debug!("Getting plist data");
        unsafe {
            unsafe_bindings::plist_get_data_val(self.plist_t, plist_data, plist_len);
        }
        debug!("Checking if plist is binary");
        match unsafe {
            unsafe_bindings::plist_is_binary(*plist_data, (*plist_len).try_into().unwrap())
        } {
            0 => false,
            _ => true,
        }
    }
    /// Reimplimented from the C function because function overloading is evil
    pub fn access_path(self, plists: Vec<String>) -> Result<Plist, ()> {
        let mut current = self;
        let mut i = 0;
        while i < plists.len() {
            match current.plist_type {
                PlistType::Array => {
                    current = match current.array_get_item(i as u32) {
                        Ok(item) => item,
                        Err(_) => return Err(()),
                    };
                }
                PlistType::Dictionary => {
                    current = match current.dict_get_item(&plists[i]) {
                        Ok(item) => item,
                        Err(_) => return Err(()),
                    };
                }
                _ => {
                    return Err(());
                }
            }
            i += 1;
        }
        Ok(current.plist_t.into()) // Probably really stupid
    }

    /// This is necessary when a function absorbs another plist.
    /// That way, the rest of the plist struct is dropped, but the data at the pointer is not.
    /// This prevents many segfaults, but may cause unknown memory leaks.
    /// Needs more research...
    pub fn false_drop(mut self) {
        debug!("False dropping {}", self.id);
        let replacement = unsafe { unsafe_bindings::plist_new_bool(0) };
        self.plist_t = replacement;
    }
}

impl From<unsafe_bindings::plist_t> for Plist {
    fn from(plist_t: unsafe_bindings::plist_t) -> Self {
        let mut rng = rand::thread_rng();
        let id = rng.gen::<u32>();
        debug!("Creating plist from plist_t with id {}", id);
        Plist {
            plist_t,
            plist_type: unsafe { unsafe_bindings::plist_get_node_type(plist_t) }.into(),
            dependent_plists: Vec::new(),
            id: id,
        }
    }
}

impl From<Plist> for String {
    fn from(plist: Plist) -> Self {
        let plist_t = plist.plist_t;
        let mut plist_data = std::ptr::null_mut();
        let mut plist_size = 0;
        debug!("Converting plist to XML data");
        unsafe {
            unsafe_bindings::plist_to_xml(plist_t, &mut plist_data, &mut plist_size);
        }
        debug!("Assembling XML data");
        let plist_data = unsafe {
            std::slice::from_raw_parts(plist_data as *const u8, plist_size.try_into().unwrap())
        };
        let plist_data = std::str::from_utf8(plist_data).unwrap();
        let plist_data = String::from(plist_data);
        plist_data
    }
}

impl ToString for Plist {
    fn to_string(&self) -> String {
        let plist_t = self.plist_t;
        let mut plist_data = std::ptr::null_mut();
        let mut plist_size = 0;
        debug!("Converting plist to XML data");
        unsafe {
            unsafe_bindings::plist_to_xml(plist_t, &mut plist_data, &mut plist_size);
        }
        debug!("Assembling XML data");
        let plist_data = unsafe {
            std::slice::from_raw_parts(plist_data as *const u8, plist_size.try_into().unwrap())
        };
        let plist_data = std::str::from_utf8(plist_data).unwrap();
        let plist_data = String::from(plist_data);
        plist_data
    }
}

impl From<Plist> for Vec<u8> {
    fn from(plist: Plist) -> Self {
        let plist_t = plist.plist_t;
        let mut plist_data = std::ptr::null_mut();
        let mut plist_size = 0;
        debug!("Converting plist to binary data");
        unsafe {
            unsafe_bindings::plist_to_bin(plist_t, &mut plist_data, &mut plist_size);
        }
        debug!("Assembling binary data");
        let plist_data = unsafe {
            std::slice::from_raw_parts(plist_data as *const u8, plist_size.try_into().unwrap())
        };
        let plist_data = plist_data.to_vec();
        plist_data
    }
}



impl Clone for Plist {
    fn clone(&self) -> Self {
        debug!("Cloning plist");
        let plist_t = unsafe { unsafe_bindings::plist_copy(self.plist_t) };
        debug!("Getting type of cloned plist");
        plist_t.into()
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
        debug!("Dropping plist {}", self.id);
        // Dependent plists should be freed automatically because this object is being dropped, right?
        if self.plist_t as u8 == 0 {
            debug!("Plist has already been freed");
            return;
        }
        unsafe { unsafe_bindings::plist_free(self.plist_t) }
        debug!("Plist dropped");
    }
}

impl PlistArrayIter {
    pub fn next_item(&mut self) -> Option<Plist> {
        let to_fill = unsafe { std::mem::zeroed() };
        debug!("Getting next item in array");
        unsafe {
            unsafe_bindings::plist_array_next_item(
                self.plist.plist_t,
                self.plist_array_iter,
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
}

impl From<Plist> for PlistArrayIter {
    fn from(plist: Plist) -> Self {
        let mut plist_array_iter = unsafe { std::mem::zeroed() };
        debug!("Getting iterator for array");
        unsafe { unsafe_bindings::plist_array_new_iter(plist.plist_t, &mut plist_array_iter) };
        PlistArrayIter {
            plist_array_iter,
            plist,
        }
    }
}

impl PlistDictIter {
    pub fn next_item(&mut self) -> Option<(String, Plist)> {
        let mut key = unsafe { std::mem::zeroed() };
        let mut to_fill = unsafe { std::mem::zeroed() };
        debug!("Getting next item in dictionary");
        unsafe {
            unsafe_bindings::plist_dict_next_item(
                self.plist.plist_t,
                self.plist_dict_iter,
                &mut key,
                &mut to_fill,
            )
        };
        if to_fill.is_null() {
            debug!("No more items in dictionary");
            None
        } else {
            let key_str = unsafe { std::ffi::CStr::from_ptr(key).to_string_lossy().into_owned() };
            debug!("Getting type of next item in dictionary");
            Some((key_str, to_fill.into())) // yeet
        }
    }
}

impl From<Plist> for PlistDictIter {
    fn from(plist: Plist) -> Self {
        let mut plist_dict_iter = unsafe { std::mem::zeroed() };
        debug!("Getting iterator for dictionary");
        unsafe { unsafe_bindings::plist_dict_new_iter(plist.plist_t, &mut plist_dict_iter) };
        PlistDictIter {
            plist_dict_iter,
            plist,
        }
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

pub fn compare_node_values(node_l: Plist, node_r: Plist) -> bool {
    debug!("Comparing node values");
    match unsafe { unsafe_bindings::plist_compare_node_value(node_l.plist_t, node_r.plist_t) }
        .to_string()
        .as_str()
    {
        "TRUE" => true,
        _ => false,
    }
}