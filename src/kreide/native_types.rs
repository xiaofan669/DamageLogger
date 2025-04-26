use std::{fmt::Display, slice, string::FromUtf16Error};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeObject {
    pub klass: *const std::ffi::c_void,
    pub monitor: *const std::ffi::c_void, // *const MonitorData
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeString {
    pub obj: NativeObject,
    pub m_stringLength: u32,
    pub m_firstChar: u16,
}

impl NativeString {
    pub fn to_string(&self) -> Result<String, FromUtf16Error> {
        unsafe {
            let ptr = &self.m_firstChar;
            let array = std::slice::from_raw_parts(ptr, self.m_stringLength as usize);
            String::from_utf16(&array)
        }
    }
}

impl Display for NativeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_string() {
            Ok(string) => write!(f, "{}", string),
            Err(e) => write!(f, "{}", e),
        }
    }
}

#[repr(C, align(8))]
#[derive(Debug, Clone, Copy)]
pub struct NativeArray<T> {
    pub obj: NativeObject,
    pub bounds: *const std::ffi::c_void,
    pub max_length: u32,
    // This is the first item of some pointer
    vector: *const T,
}

impl<T> NativeArray<T> {
    pub fn to_slice(&self) -> &[*const T] {
        unsafe {
            let ptr = &self.vector;
            slice::from_raw_parts(ptr, self.max_length as usize)
        }
    }
}
