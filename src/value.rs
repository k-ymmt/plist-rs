use std::ffi::{c_void, CStr, CString};
use std::os::raw::c_char;
use std::ptr::{null_mut, slice_from_raw_parts};
use crate::{Plist, plist_get_bool_val, plist_get_data_val, plist_get_key_val, plist_get_real_val, plist_get_string_val, plist_get_uid_val, plist_get_uint_val, plist_mem_free, plist_new_bool, plist_new_data, plist_new_date, plist_new_null, plist_new_real, plist_new_string, plist_new_uid, plist_new_uint, plist_set_bool_val, plist_set_data_val, plist_set_date_val, plist_set_key_val, plist_set_real_val, plist_set_string_val, plist_set_uid_val, plist_set_uint_val, PlistType};

// Getters
impl Plist {
    pub fn as_str(&self) -> Option<String> {
        let mut val: *mut c_char = null_mut();
        unsafe {
            plist_get_string_val(self.get_ptr().ok()?, &mut val)
        };

        if val.is_null() {
            return None;
        }

        let string = unsafe { CStr::from_ptr(val) };
        let string = string.to_str().unwrap().to_owned();
        unsafe {
            plist_mem_free(val as *mut c_void);
        }

        Some(string)
    }

    pub fn as_bool(&self) -> Option<bool> {
        let mut bool: u8 = u8::MAX;
        unsafe { plist_get_bool_val(self.get_ptr().ok()?, &mut bool) };

        if bool == u8::MAX {
            return None
        }

        Some(bool == 1)
    }

    pub fn as_key(&self) -> Option<String> {
        let mut raw: *mut c_char = null_mut();
        unsafe { plist_get_key_val(self.get_ptr().ok()?, &mut raw) };

        if raw.is_null() {
            return None
        }

        let key = unsafe { CStr::from_ptr(raw) };
        let key = key.to_str().unwrap().to_owned();

        unsafe {
            plist_mem_free(raw as *mut c_void)
        };

        Some(key)
    }

    pub fn as_uint(&self) -> Option<u64> {
        if self.plist_type() != PlistType::UInt {
            return None
        }

        let mut uint: u64  = 0;
        unsafe { plist_get_uint_val(self.get_ptr().ok()?,  &mut uint) };

        Some(uint)
    }

    pub fn as_real(&self) -> Option<f64> {
        if self.plist_type() != PlistType::Real {
            return None
        }

        let mut real: f64 = 0.0;
        unsafe { plist_get_real_val(self.get_ptr().ok()?, &mut real) };

        Some(real)
    }

    pub fn as_data(&self) -> Option<Vec<i8>> {
        let mut raw: *mut c_char = null_mut();
        let mut length: u64 = 0;
        unsafe { plist_get_data_val(self.get_ptr().ok()?, &mut raw, &mut length) };

        if raw.is_null() {
            return None
        }

        let data= slice_from_raw_parts(raw, length as usize);
        let data = unsafe { data.as_ref().unwrap().to_vec() };
        unsafe { plist_mem_free(raw as *mut c_void) };
        Some(data)
    }

    pub fn as_uid(&self) -> Option<u64> {
        if self.plist_type() != PlistType::UID {
            return None
        }

        let mut uid: u64 = 0;
        unsafe { plist_get_uid_val(self.get_ptr().ok()?, &mut uid) };

        Some(uid)
    }
}

// Setters
pub trait Setter<T> {
    fn set(&self, value: T);
}

impl Setter<String> for Plist {
    fn set(&self, string: String) {
        let string = CString::new(string).unwrap();
        let string = string.as_ptr();

        unsafe { plist_set_string_val(self.get_ptr().unwrap(), string) }
    }
}

impl Setter<&str> for Plist {
    fn set(&self, string: &str) {
        let string = CString::new(string).unwrap();
        let string = string.as_ptr();

        unsafe { plist_set_string_val(self.get_ptr().unwrap(), string) }
    }
}

impl Setter<bool> for Plist {
    fn set(&self, bool: bool) {
        let bool = if bool { 1 } else { 0 };
        unsafe { plist_set_bool_val(self.get_ptr().unwrap(), bool) }
    }
}

impl Setter<u64> for Plist {
    fn set(&self, uint: u64) {
        unsafe { plist_set_uint_val(self.get_ptr().unwrap(), uint) }
    }
}

impl Setter<f64> for Plist {
    fn set(&self, real: f64) {
        unsafe { plist_set_real_val(self.get_ptr().unwrap(), real) }
    }
}

impl Setter<&[i8]> for Plist {
    fn set(&self, data: &[i8]) {
        let length = data.len() as u64;
        let data = data.as_ptr();
        unsafe { plist_set_data_val(self.get_ptr().unwrap(), data, length) }
    }
}

impl Plist {
    pub fn set_key(&self, string: String) {
        let string = CString::new(string).unwrap();
        let string = string.as_ptr();

        unsafe { plist_set_key_val(self.get_ptr().unwrap(), string) }
    }

    pub fn set_date(&self, sec: i32, usec: i32) {
        unsafe { plist_set_date_val(self.get_ptr().unwrap(), sec, usec) }
    }

    pub fn set_uid(&self, uid: u64) {
        unsafe { plist_set_uid_val(self.get_ptr().unwrap(), uid) }
    }
}

impl From<&str> for Plist {
    fn from(string: &str) -> Self {
        let string = CString::new(string).unwrap();
        let string = string.as_ptr();
        let p = unsafe { plist_new_string(string) };

        Plist::new(p)
    }
}

impl From<String> for Plist {
    fn from(string: String) -> Self {
        let string = CString::new(string).unwrap();
        let string = string.as_ptr();
        let p = unsafe { plist_new_string(string) };

        Plist::new(p)
    }
}

impl From<bool> for Plist {
    fn from(boolean: bool) -> Self {
        let p = unsafe { plist_new_bool(if boolean { 1 } else { 0 }) };

        Plist::new(p)
    }
}

impl From<u64> for Plist {
    fn from(uint: u64) -> Self {
        let p = unsafe { plist_new_uint(uint) };

        Plist::new(p)
    }
}

impl From<f64> for Plist {
    fn from(real: f64) -> Self {
        let p = unsafe { plist_new_real(real) };

        Plist::new(p)
    }
}

impl From<&[i8]> for Plist {
    fn from(data: &[i8]) -> Self {
        let length = data.len();
        let p = unsafe { plist_new_data(data.as_ptr(), length as u64) };

        Plist::new(p)
    }
}

impl Plist {
    pub fn from_date(sec: i32, usec: i32) -> Self {
        let p = unsafe { plist_new_date(sec, usec) };

        Plist::new(p)
    }
}

impl Plist {
    pub fn from_uid(uid: u64) -> Self {
        let p = unsafe { plist_new_uid(uid) };

        Plist::new(p)
    }
}

impl Plist {
    pub fn null() -> Self {
        let p = unsafe { plist_new_null() };

        Plist::new(p)
    }
}

#[cfg(test)]
mod tests {
    use crate::Plist;
    use crate::value::Setter;

    #[test]
    fn test() {
        let plist = Plist::from("string");
        assert_eq!(plist.as_str().unwrap(), "string");
        plist.set("new string");
        assert_eq!(plist.as_str().unwrap(), "new string");

        let plist = Plist::from(true);
        assert_eq!(plist.as_bool().unwrap(), true);
        plist.set(false);
        assert_eq!(plist.as_bool().unwrap(), false);

        let plist = Plist::from(1 as u64);
        assert_eq!(plist.as_uint().unwrap(), 1);
        plist.set(u64::MAX);
        assert_eq!(plist.as_uint().unwrap(), u64::MAX);

        let plist = Plist::from(1 as f64);
        assert_eq!(plist.as_real().unwrap(), 1.0);
        plist.set(f64::MAX);
        assert_eq!(plist.as_real().unwrap(), f64::MAX);

        let data: &[i8] = [1, 2, 3, 4, 5].as_slice();
        let plist = Plist::from(data);
        assert_eq!(plist.as_data().unwrap(), data);
        let data: &[i8] = [0].as_slice();
        plist.set(data);
        assert_eq!(plist.as_data().unwrap(), data);

        let plist = Plist::from_uid(1);
        assert_eq!(plist.as_uid().unwrap(), 1);
        plist.set_uid(0);
        assert_eq!(plist.as_uid().unwrap(), 0);
    }
}