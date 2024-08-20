use std::fmt::Display;

use super::object_map::ObjectIndex;

#[derive(Clone, Copy)]
pub union RawVMData {
    as_unit: (),
    as_i64: i64,
    as_u64: u64,
    as_f64: f64,
    as_bool: bool,
    as_object: ObjectIndex,
}

#[derive(Clone, Copy)]
pub struct VMData {
    pub tag: u64,
    data: RawVMData,
}

macro_rules! def_new_vmdata_func {
    ($ident: ident, $field: ident, $ty: ty, $const: ident) => {
        #[inline(always)]
        pub fn $ident(val: $ty) -> Self {
            Self::new(Self::$const, RawVMData { $field: val })
        }
    };
}

impl VMData {
    pub const TAG_UNIT: u64 = 0;
    pub const TAG_U64: u64 = 4;
    pub const TAG_I64: u64 = 8;
    pub const TAG_FLOAT: u64 = 9;
    pub const TAG_BOOL: u64 = 10;
    pub const TAG_STR: u64 = 11;

    pub fn new(tag: u64, data: RawVMData) -> Self {
        Self { tag, data }
    }

    pub fn new_unit() -> Self {
        Self {
            tag: Self::TAG_UNIT,
            data: RawVMData { as_unit: () },
        }
    }

    pub fn new_object(tag: u64, val: ObjectIndex) -> Self {
        assert!(tag > 256, "object typeid is within the reserved area");
        Self {
            tag,
            data: RawVMData { as_object: val },
        }
    }

    pub fn new_string(val: ObjectIndex) -> Self {
        Self::new(Self::TAG_STR, RawVMData { as_object: val })
    }

    def_new_vmdata_func!(new_i64, as_i64, i64, TAG_I64);
    def_new_vmdata_func!(new_u64, as_u64, u64, TAG_U64);
    def_new_vmdata_func!(new_f64, as_f64, f64, TAG_FLOAT);
    def_new_vmdata_func!(new_bool, as_bool, bool, TAG_BOOL);
}

impl PartialEq for VMData {
    fn eq(&self, other: &Self) -> bool {
        if self.tag != other.tag {
            return false;
        }

        match self.tag {
            Self::TAG_BOOL => self.as_bool() == other.as_bool(),
            Self::TAG_FLOAT => self.as_f64() == other.as_f64(),
            Self::TAG_I64 => self.as_i64() == other.as_i64(),
            Self::TAG_U64 => self.as_u64() == other.as_u64(),
            Self::TAG_UNIT => true,
            _ if self.tag > 256 => self.as_object() == other.as_object(),
            _ => panic!("Illegal comparison"),
        }
    }
}

impl PartialOrd for VMData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.tag != other.tag {
            return None;
        }

        match self.tag {
            Self::TAG_FLOAT => self.as_f64().partial_cmp(&other.as_f64()),
            Self::TAG_U64 => self.as_u64().partial_cmp(&other.as_u64()),
            Self::TAG_I64 => self.as_i64().partial_cmp(&other.as_i64()),
            _ => panic!("Illegal comparison"),
        }
    }
}

impl Display for VMData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.tag {
                Self::TAG_UNIT => "()".to_string(),
                Self::TAG_I64 => self.as_i64().to_string(),
                Self::TAG_U64 => self.as_u64().to_string(),
                Self::TAG_FLOAT => self.as_f64().to_string(),
                Self::TAG_BOOL => self.as_bool().to_string(),

                _ if self.is_object() => self.as_object().to_string(),
                _ => "reserved".to_string(),
            }
        )
    }
}

impl std::fmt::Debug for VMData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VMData {{ tag: {}({}), data: {}}}",
            self.tag,
            match self.tag {
                Self::TAG_BOOL => "bool",
                Self::TAG_UNIT => "unit",
                Self::TAG_FLOAT => "f64",
                Self::TAG_I64 => "i64",
                Self::TAG_U64 => "u64",
                _ if self.is_object() => "obj",
                _ => "res",
            },
            match self.tag {
                Self::TAG_UNIT => "()".to_string(),
                Self::TAG_I64 => self.as_i64().to_string(),
                Self::TAG_U64 => self.as_u64().to_string(),
                Self::TAG_FLOAT => self.as_f64().to_string(),
                Self::TAG_BOOL => self.as_bool().to_string(),
                _ if self.is_object() => self.as_object().to_string(),
                _ => "reserved".to_string(),
            }
        )
    }
}

macro_rules! enum_variant_function {
    ($getter: ident, $is: ident, $variant: ident, $ty: ty) => {
        #[inline(always)]
        #[must_use]
        pub fn $getter(self) -> $ty {
            unsafe { self.data.$getter }
        }

        #[inline(always)]
        #[must_use]
        pub fn $is(self) -> bool {
            self.tag == Self::$variant
        }
    };
}

impl VMData {
    enum_variant_function!(as_i64, is_i64, TAG_I64, i64);
    enum_variant_function!(as_f64, is_f64, TAG_FLOAT, f64);
    enum_variant_function!(as_u64, is_u64, TAG_U64, u64);
    enum_variant_function!(as_bool, is_bool, TAG_BOOL, bool);
    enum_variant_function!(as_unit, is_unit, TAG_UNIT, ());

    #[inline(always)]
    #[must_use]
    pub fn is_object(self) -> bool {
        self.tag > 256 || self.tag == Self::TAG_STR
    }

    #[inline(always)]
    #[must_use]
    pub fn as_object(self) -> ObjectIndex {
        if !self.is_object() {
            unreachable!()
        }

        unsafe { self.data.as_object }
    }
}
