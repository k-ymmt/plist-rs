use std::ptr::null_mut;
use std::rc::Rc;
use crate::{Getter, Plist, plist_array_append_item, plist_array_get_item, plist_array_get_size, plist_array_insert_item, plist_array_iter, plist_array_new_iter, plist_array_next_item, plist_array_remove_item, plist_array_set_item, plist_copy, plist_new_array, plist_t, PlistType};

impl From<Vec<Plist>> for Plist {
    fn from(array: Vec<Plist>) -> Self {
        let p = unsafe { plist_new_array() };
        for value in array {
            unsafe {
                let value = plist_copy(value.as_ptr().unwrap());
                plist_array_append_item(p, value)
            }
        }

        Plist::new(p)
    }
}

impl From<&[Plist]> for Plist {
    fn from(array: &[Plist]) -> Self {
        let p = unsafe { plist_new_array() };
        for value in array {
            unsafe {
                let value = plist_copy(value.as_ptr().unwrap());
                plist_array_append_item(p, value);
            }
        }

        Plist::new(p)
    }
}

impl Getter<usize> for Plist {
    fn get(&self, index: usize) -> Option<Self> {
        let p = unsafe {
            plist_array_get_item(self.as_ptr().ok()?, index as u32)
        };
        if p.is_null() {
            return None;
        }

        Some(Plist::new_with_weak(p))
    }
}

pub struct PlistArray {
    pub(crate) inner: Rc<Plist>,
}

impl Into<Rc<Plist>> for PlistArray {
    fn into(self) -> Rc<Plist> {
        self.inner
    }
}

impl PlistArray {
    pub fn len(&self) -> usize {
        unsafe {
            plist_array_get_size(self.inner.as_ptr().unwrap()) as usize
        }
    }

    pub fn get(&self, index: usize) -> Option<Plist> {
        self.inner.get(index)
    }

    pub fn set(&self, mut item: Plist, index: u32) {
        unsafe { plist_array_set_item(self.inner.as_ptr().unwrap(), item.as_ptr().unwrap(), index) }
        item.replace_weak();
    }

    pub fn append(&self, mut item: Plist) {
        unsafe { plist_array_append_item(self.inner.as_ptr().unwrap(), item.as_ptr().unwrap()) }
        item.replace_weak();
    }

    pub fn insert(&self, mut item: Plist, index: u32) {
        unsafe { plist_array_insert_item(self.inner.as_ptr().unwrap(), item.as_ptr().unwrap(), index) }
        item.replace_weak();
    }

    pub fn remove(&self, index: u32) {
        unsafe { plist_array_remove_item(self.inner.as_ptr().unwrap(), index) }
    }
}

pub struct PlistArrayIter {
    p: Rc<Plist>,
    iter: plist_array_iter,
}

impl Iterator for PlistArrayIter {
    type Item = Plist;

    fn next(&mut self) -> Option<Self::Item> {
        let mut p: plist_t = null_mut();
        unsafe { plist_array_next_item(self.p.as_ptr().unwrap(), self.iter, &mut p) }

        if p.is_null() {
            return None;
        }

        Some(Plist::new_with_weak(p))
    }
}

impl IntoIterator for PlistArray {
    type Item = Plist;
    type IntoIter = PlistArrayIter;

    fn into_iter(self) -> Self::IntoIter {
        let mut iter: plist_array_iter = null_mut();
        unsafe { plist_array_new_iter(self.inner.as_ptr().unwrap(), &mut iter) };
        let p = Rc::clone(&self.inner);
        PlistArrayIter {
            p,
            iter,
        }
    }
}

impl Plist {
    pub fn array(self) -> Option<PlistArray> {
        if self.plist_type() != PlistType::Array {
            return None;
        }

        Some(PlistArray {
            inner: Rc::new(self)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Getter, Plist};

    #[test]
    fn new_test() {
        let array = [
            Plist::from("test"),
            Plist::from(true)
        ];
        let array = Plist::from(array.as_slice());

        assert_eq!(array.get(0).unwrap().as_str().unwrap(), "test");
        assert_eq!(array.get(1).unwrap().as_bool().unwrap(), true);
    }

    #[test]
    fn len() {
        let array = [
            Plist::from(1),
            Plist::from(1),
            Plist::from(1),
        ];
        let array = Plist::from(array.as_slice());

        assert_eq!(array.array().unwrap().len(), 3);
    }

    #[test]
    fn set() {
        let array = [
            Plist::from(0),
            Plist::from(1),
        ];
        let array = Plist::from(array.as_slice());
        let array = array.array().unwrap();
        array.set(Plist::from(2), 1);
        assert_eq!(array.get(1).unwrap().as_uint().unwrap(), 2)
    }

    #[test]
    fn append() {
        let array = [
            Plist::from(0)
        ];
        let array = Plist::from(array.as_slice());
        let array = array.array().unwrap();
        array.append(Plist::from(1));
        assert_eq!(array.get(0).unwrap().as_uint().unwrap(), 0);
        assert_eq!(array.get(1).unwrap().as_uint().unwrap(), 1);
    }

    #[test]
    fn insert() {
        let array = [
            Plist::from(0),
            Plist::from(2),
        ];
        let array = Plist::from(array.as_slice());
        let array = array.array().unwrap();
        array.insert(Plist::from(1), 1);
        assert_eq!(array.len(), 3);
        assert_eq!(array.get(1).unwrap().as_uint().unwrap(), 1);
    }

    #[test]
    fn remove() {
        let array = [
            Plist::from(0),
            Plist::from(1),
            Plist::from(2),
        ];
        let array = Plist::from(array.as_slice());
        let array = array.array().unwrap();
        array.remove(1);
        assert_eq!(array.len(), 2);
        assert_eq!(array.get(1).unwrap().as_uint().unwrap(), 2);
    }

    #[test]
    fn iter() {
        let values = [0, 2, 3];
        let array = values.map(Plist::from);
        let array = Plist::from(array.as_slice());

        for (index, value) in array.array().unwrap().into_iter().enumerate() {
            assert_eq!(value.as_uint().unwrap(), values[index]);
        }
    }
}