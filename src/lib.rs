#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unaligned_references)]

extern crate core;

use std::ffi::{c_void, CStr, CString};
use std::fmt::{Debug, Formatter};
use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::plist_error::PlistError;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod plist_error;
pub mod array;
pub mod dict;
pub mod value;

#[derive(Debug, Eq, PartialEq)]
pub enum PlistType {
    Boolean,
    UInt,
    Real,
    String,
    Array,
    Dictionary,
    Date,
    Data,
    Key,
    UID,
    Null,
    None
}

impl PlistType {
    fn from(plist_type: plist_type) -> Self {
        match plist_type {
            plist_type_PLIST_BOOLEAN => Self::Boolean,
            plist_type_PLIST_UINT => Self::UInt,
            plist_type_PLIST_REAL => Self::Real,
            plist_type_PLIST_STRING => Self::String,
            plist_type_PLIST_ARRAY => Self::Array,
            plist_type_PLIST_DICT => Self::Dictionary,
            plist_type_PLIST_DATE => Self::Date,
            plist_type_PLIST_DATA => Self::Data,
            plist_type_PLIST_KEY => Self::Key,
            plist_type_PLIST_UID => Self::UID,
            plist_type_PLIST_NULL => Self::Null,
            plist_type_PLIST_NONE => Self::None,
            _ => panic!("Unexpected plist_type")
        }
    }
}

pub struct Plist {
    pub(crate) p: Option<plist_t>,
    pub(crate) rawP: Option<plist_t>
}

impl Drop for Plist {
    fn drop(&mut self) {
        if let Some(p) = self.p {
            unsafe { plist_free(p) }
        }
    }
}

impl Debug for Plist {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.xml().unwrap_or_else(|x| x.to_string()))
    }
}

impl Plist {
    pub fn new(plist: plist_t) -> Self {
        Plist {
            p: Some(plist),
            rawP: None
        }
    }

    pub fn new_with_weak(plist: plist_t) -> Self {
        Plist {
            p: None,
            rawP: Some(plist)
        }
    }

    pub fn from_xml(xml: String) -> Result<Self, PlistError>  {
        let mut p: plist_t = std::ptr::null_mut();
        let xml = CString::new(xml).unwrap();
        let length = xml.as_bytes().len();
        let xml = xml.as_ptr();
        unsafe {
            PlistError::try_from(plist_from_xml(xml, length as u32, &mut p))
        }?;

        Ok(Plist::new(p))
    }

    pub fn from_bin(bin: &[u8]) -> Result<Self, PlistError> {
        let mut p: plist_t = std::ptr::null_mut();
        let length = bin.len();
        let bin = CString::new(bin).unwrap();
        unsafe {
            PlistError::try_from(plist_from_bin(bin.as_ptr(), length as u32, &mut p))
        }?;

        Ok(Plist::new(p))
    }

    pub fn from_json(json: String) -> Result<Self, PlistError> {
        let length = json.len();
        let json = CString::new(json).unwrap();
        let json = json.as_ptr();
        let mut plist: plist_t = null_mut();

        unsafe {
            PlistError::try_from(plist_from_json(json, length as u32, &mut plist))
        }?;

        Ok(Plist::new(plist))
    }

    pub fn from_memory(data: &[i8]) -> Result<Self, PlistError> {
        let length = data.len();
        let data = data.as_ptr();
        let mut plist: plist_t = null_mut();

        unsafe {
            PlistError::try_from(plist_from_memory(data, length as u32, &mut plist))
        }?;

        Ok(Plist::new(plist))
    }

    pub fn is_binary(data: &[i8]) -> bool {
        let length = data.len();
        let data = data.as_ptr();

        let is_binary = unsafe { plist_is_binary(data, length as u32) };

        is_binary == 1
    }

    pub fn copy(&self) -> Self {
        let p = unsafe { plist_copy(self.as_ptr().unwrap()) };

        Plist::new(p)
    }

    pub fn xml(&self) -> Result<String, PlistError> {
        let mut xml: *mut c_char = std::ptr::null_mut();
        let mut length = 0;
        unsafe {
            PlistError::try_from(plist_to_xml(self.as_ptr()?, &mut xml, &mut length))
        }?;

        let result = unsafe { CStr::from_ptr(xml) };
        let result = result.to_str().unwrap().to_owned();
        unsafe {
            plist_mem_free(xml as *mut c_void)
        };

        Ok(result)
    }

    pub fn bin(&self) -> Result<Vec<i8>, PlistError> {
        let mut raw: *mut c_char = null_mut();
        let mut length: u32 = 0;
        unsafe {
            PlistError::try_from(plist_to_bin(self.as_ptr()?, &mut raw, &mut length))
        }?;

        let bin = unsafe { std::slice::from_raw_parts(raw as *const i8, length as usize) };

        unsafe {
            plist_mem_free(raw as *mut c_void);
        }

        Ok(bin.to_vec())
    }

    pub fn json(&self, prettify: bool) -> Result<String, PlistError> {
        let prettify = if prettify { 1 } else { 0 };
        let mut raw: *mut c_char = null_mut();
        let mut length: u32 = 0;

        unsafe {
            PlistError::try_from(plist_to_json(self.as_ptr()?, &mut raw, &mut length, prettify))
        }?;

        let json = unsafe { CStr::from_ptr(raw) };
        let json = json.to_str().unwrap();

        unsafe {
            plist_mem_free(raw as *mut c_void);
        }

        Ok(json.to_owned())
    }

    pub fn plist_type(&self) -> PlistType {
        if let Ok(p) = self.as_ptr() {
            let t = unsafe { plist_get_node_type(p) };
            PlistType::from(t)
        } else {
            PlistType::None
        }
    }

    pub fn as_ptr(&self) -> Result<plist_t, PlistError> {
        match self.p {
            Some(p) => Ok(p),
            None => match self.rawP {
                Some(p) => Ok(p),
                None => Err(PlistError::Dealloc)
            }
        }
    }

    pub(crate) fn replace_weak(&mut self) {
        if let Some(p) = self.p {
            self.rawP = Some(p);
            self.p = None;
        }
    }
}

trait Getter<T> {
    fn get(&self, index: T) -> Option<Self> where Self: Sized;
}

#[cfg(test)]
mod tests {
    use crate::Plist;
    use crate::dict::DictGetter;

    #[test]
    fn xml() {
        let xml = r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
            <plist version="1.0">
            <dict>
                <key>test1</key>
                <string>foo</string>
                <key>test2</key>
                <integer>10000</integer>
                <key>tests</key>
                <array>
                    <true/>
                    <false/>
                    <true/>
                </array>
                <key>dict</key>
                <dict>
                    <key>array</key>
                    <array>
                        <dict>
                            <key>hoge</key>
                            <string>hoge</string>
                        </dict>
                    </array>
                </dict>
            </dict>
            </plist>
        "#;
        let plist = Plist::from_xml(xml.to_string()).unwrap();
        assert_eq!(plist.dict().unwrap().get("test1").unwrap().as_str().unwrap(), "foo");
    }
}
