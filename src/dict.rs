use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr::null_mut;
use std::rc::Rc;
use crate::{Getter, Plist, plist_copy, plist_dict_get_item, plist_dict_get_size, plist_dict_iter, plist_dict_merge, plist_dict_new_iter, plist_dict_next_item, plist_dict_remove_item, plist_dict_set_item, plist_new_dict, plist_t, PlistType};

impl From<HashMap<&str, Plist>> for Plist {
    fn from(dict: HashMap<&str, Plist>) -> Self {
        let p = unsafe { plist_new_dict() };
        for (key, value) in dict {
            let key = CString::new(key).unwrap();
            let key = key.as_ptr();
            unsafe { plist_dict_set_item(p, key, plist_copy(value.as_ptr().unwrap())) }
        }

        Plist::new(p)
    }
}

impl Getter<&str> for Plist {
    fn get(&self, index: &str) -> Option<Self> {
        let key = CString::new(index).unwrap();
        let key = key.as_ptr();
        let p = unsafe {
            plist_dict_get_item(self.as_ptr().ok()?, key)
        };

        if p.is_null() {
            return None
        }

        Some(Plist::new_with_weak(p))
    }
}

impl Getter<String> for Plist {
    fn get(&self, index: String) -> Option<Self> where Self: Sized {
        let key = CString::new(index).unwrap();
        let key = key.as_ptr();
        let p = unsafe {
            plist_dict_get_item(self.as_ptr().ok()?, key)
        };

        if p.is_null() {
            return None
        }

        Some(Plist::new_with_weak(p))
    }
}

pub struct PlistDict {
    inner: Rc<Plist>
}

impl PlistDict {
    pub fn len(&self) -> usize {
        unsafe { plist_dict_get_size(self.inner.as_ptr().unwrap()) as usize }
    }

    pub fn merge(&self, source: &PlistDict) {
        let mut p = self.inner.as_ptr().unwrap();
        let s = source.inner.as_ptr().unwrap();
        unsafe { plist_dict_merge(&mut p, s) }
    }
}

pub trait DictSetter<T> {
    fn set(&self, key: T, value: Plist);
}

impl DictSetter<String> for PlistDict {
    fn set(&self, key: String, mut value: Plist) {
        let key = CString::new(key).unwrap();
        let key = key.as_ptr();
        unsafe { plist_dict_set_item(self.inner.as_ptr().unwrap(), key, value.as_ptr().unwrap()) }
        value.replace_weak();
    }
}

impl DictSetter<&str> for PlistDict {
    fn set(&self, key: &str, mut value: Plist) {
        let key = CString::new(key).unwrap();
        let key = key.as_ptr();
        unsafe { plist_dict_set_item(self.inner.as_ptr().unwrap(), key, value.as_ptr().unwrap()) }
        value.replace_weak();
    }
}

pub trait DictGetter<T> {
    fn get(&self, key: T) -> Option<Plist>;
}

impl DictGetter<String> for PlistDict {
    fn get(&self, key: String) -> Option<Plist> {
        self.inner.get(key)
    }
}

impl DictGetter<&str> for PlistDict {
    fn get(&self, key: &str) -> Option<Plist> {
        self.inner.get(key)
    }
}

pub trait DictRemove<T> {
    fn remove(&self, key: T);
}

impl DictRemove<String> for PlistDict {
    fn remove(&self, key: String) {
        let key = CString::new(key).unwrap();
        let key = key.as_ptr();
        unsafe { plist_dict_remove_item(self.inner.as_ptr().unwrap(), key) }
    }
}

impl DictRemove<&str> for PlistDict {
    fn remove(&self, key: &str) {
        let key = CString::new(key).unwrap();
        let key = key.as_ptr();
        unsafe { plist_dict_remove_item(self.inner.as_ptr().unwrap(), key) }
    }
}

pub struct PlistDictIter {
    p: Rc<Plist>,
    iter: plist_dict_iter
}

impl Iterator for PlistDictIter {
    type Item = (String, Plist);

    fn next(&mut self) -> Option<Self::Item> {
        let mut key: *mut c_char = null_mut();
        let mut value: plist_t = null_mut();
        unsafe { plist_dict_next_item(self.p.as_ptr().ok()?, self.iter, &mut key, &mut value) };

        if key.is_null() || value.is_null() {
            return None
        }

        let key = unsafe { CStr::from_ptr(key).to_str().unwrap() };
        let key = key.to_owned();
        let value = Plist::new_with_weak(value);

        Some((key, value))
    }
}

impl IntoIterator for PlistDict {
    type Item = (String, Plist);
    type IntoIter = PlistDictIter;

    fn into_iter(self) -> Self::IntoIter {
        let mut iter: plist_dict_iter = null_mut();
        unsafe { plist_dict_new_iter(self.inner.as_ptr().unwrap(), &mut iter) };

        PlistDictIter {
            p: Rc::clone(&self.inner.clone()),
            iter
        }
    }
}

impl Into<Rc<Plist>> for PlistDict {
    fn into(self) -> Rc<Plist> {
        Rc::clone(&self.inner)
    }
}

impl Plist {
    pub fn dict(self) -> Option<PlistDict> {
        if self.plist_type() != PlistType::Dictionary {
            return None
        }

        Some(PlistDict {
            inner: Rc::new(self)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{Getter, Plist};
    use crate::dict::{DictGetter, DictRemove, DictSetter};

    #[test]
    fn new_test() {
        let dict = HashMap::from([
            ("key1", Plist::from(0)),
            ("key2", Plist::from(1)),
        ]);
        let dict = Plist::from(dict);
        assert_eq!(dict.get("key1").unwrap().as_uint().unwrap(), 0);
        assert_eq!(dict.get("key2").unwrap().as_uint().unwrap(), 1);
    }

    #[test]
    fn len() {
        let dict = HashMap::from([
            ("key1", Plist::from(0)),
            ("key2", Plist::from(1)),
            ("key3", Plist::from(2)),
        ]);
        let dict = Plist::from(dict);

        assert_eq!(dict.dict().unwrap().len(), 3);
    }

    #[test]
    fn set() {
        let dict = HashMap::from([
            ("key1", Plist::from(0)),
            ("key2", Plist::from(1))
        ]);
        let dict = Plist::from(dict);
        let dict = dict.dict().unwrap();
        dict.set("key2", Plist::from(2));
        assert_eq!(dict.get("key2").unwrap().as_uint().unwrap(), 2)
    }

    #[test]
    fn remove() {
        let dict = HashMap::from([
            ("key1", Plist::from(0)),
            ("key2", Plist::from(1)),
            ("key3", Plist::from(2)),
        ]);
        let dict = Plist::from(dict);
        let dict = dict.dict().unwrap();
        dict.remove("key2");
        assert_eq!(dict.len(), 2);
    }

    #[test]
    fn iter() {
        let keyValues = HashMap::from([
            ("key1", 0),
            ("key2", 1),
            ("key3", 2),
        ]);
        let keys = keyValues.keys().cloned().collect::<Vec<&str>>();
        let dict = Plist::from(HashMap::new());
        let dict = dict.dict().unwrap();
        for (key, value) in keyValues.clone() {
            dict.set(key, Plist::from(value));
        }

        for (index, (key, value)) in dict.into_iter().enumerate() {
            assert_eq!(key, keys[index]);
            assert_eq!(value.as_uint().unwrap(), keyValues[keys[index]]);
        }
    }
}