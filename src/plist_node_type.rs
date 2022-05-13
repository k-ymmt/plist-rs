use crate::{plist_type, plist_type_PLIST_ARRAY, plist_type_PLIST_BOOLEAN, plist_type_PLIST_DATA, plist_type_PLIST_DATE, plist_type_PLIST_DICT, plist_type_PLIST_KEY, plist_type_PLIST_NONE, plist_type_PLIST_NULL, plist_type_PLIST_REAL, plist_type_PLIST_STRING, plist_type_PLIST_UID, plist_type_PLIST_UINT};

#[derive(Debug, Eq, PartialEq)]
pub enum PlistNodeType {
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

impl PlistNodeType {
    pub fn from(plist_type: plist_type) -> Self {
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
