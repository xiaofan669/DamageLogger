use std::{borrow::Cow, ffi::CString, ops::Add as _};

use crate::{cs_class, cs_method, cs_property};

use super::{
    api::{Il2CppClass, Il2CppField, Il2CppType},
    get_cached_class,
};

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Il2CppString(pub usize);

impl Il2CppString {
    cs_class!("System.Runtime.InteropServices.Marshal");

    cs_method!(ptr_to_string_ansi, "PtrToStringAnsi", &["System.IntPtr"], Il2CppString, (ptr: *const u8));

    #[inline(always)]
    pub fn as_str(&self) -> Cow<'static, str> {
        unsafe {
            let str_length = *(self.0.wrapping_add(16) as *const u32);
            let str_ptr = self.0.wrapping_add(20) as *const u16;
            let slice = std::slice::from_raw_parts(str_ptr, str_length as usize);
            String::from_utf16(slice).unwrap().into()
        }
    }
}

impl From<&str> for Il2CppString {
    #[inline(always)]
    fn from(s: &str) -> Self {
        let cs = CString::new(s).unwrap();
        Self::ptr_to_string_ansi(cs.as_c_str().to_bytes_with_nul().as_ptr())
            .expect("failed to allocate il2cpp string")
    }
}

impl From<String> for Il2CppString {
    #[inline(always)]
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<usize> for Il2CppString {
    #[inline(always)]
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Il2CppArray(pub usize);

impl Il2CppArray {
    #[inline(always)]
    pub fn class(&self) -> Il2CppClass {
        unsafe { Il2CppClass(*(self.0 as *const usize)) }
    }

    #[inline(always)]
    pub fn monitor(&self) -> usize {
        unsafe { *((self.0 + 8) as *const usize) }
    }

    #[inline(always)]
    pub fn bounds(&self) -> usize {
        unsafe { *((self.0 + 16) as *const usize) }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        unsafe { *((self.0 + 24) as *const usize) }
    }

    #[inline(always)]
    fn first_item_ptr(&self) -> usize {
        self.0 + 32
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    pub fn get<T>(&self, i: usize) -> &T {
        let size = std::mem::size_of::<T>();
        unsafe { &*((self.first_item_ptr().add(i * size)) as *const T) }
    }

    #[inline(always)]
    pub fn get_mut<T>(&mut self, i: usize) -> &mut T {
        let size = std::mem::size_of::<T>();
        unsafe { &mut *((self.first_item_ptr().add(i * size)) as *mut T) }
    }

    #[inline(always)]
    pub fn to_vec<T: Clone>(self) -> Vec<T> {
        unsafe {
            std::slice::from_raw_parts(self.first_item_ptr() as *const T, self.len()).to_vec()
        }
    }

    #[inline(always)]
    pub fn to_vec_sized<T: Clone>(self, size: i32) -> Vec<T> {
        unsafe {
            std::slice::from_raw_parts(self.first_item_ptr() as *const T, size as usize).to_vec()
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Il2CppObject(pub usize);

#[allow(unused)]
impl Il2CppObject {
    pub const NULL: Il2CppObject = Il2CppObject(0);

    #[inline(always)]
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    pub fn get_class(&self) -> Il2CppClass {
        unsafe { Il2CppClass(*(self.0 as *const usize)) }
    }

    #[inline(always)]
    pub fn unbox<T: Copy>(&self) -> T {
        unsafe { *((self.0 + 16) as *const T) }
    }
}

impl From<usize> for Il2CppObject {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct List(pub usize);

impl List {
    #[inline(always)]
    pub fn class(&self) -> Il2CppClass {
        unsafe { Il2CppClass(*(self.0 as *const usize)) }
    }

    #[inline(always)]
    pub fn monitor(&self) -> usize {
        unsafe { *((self.0 + 8) as *const usize) }
    }

    #[inline(always)]
    pub fn items(&self) -> Il2CppArray {
        unsafe { Il2CppArray(*((self.0 + 16) as *const usize)) }
    }

    #[inline(always)]
    pub fn size(&self) -> i32 {
        unsafe { *((self.0 + 24) as *const i32) }
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
struct Type(pub usize);

impl Type {
    cs_class!("System.Type");
    cs_method!(get_type_from_handle, "GetTypeFromHandle", &["System.RuntimeTypeHandle"], Self, (ty: crate::kreide::il2cpp::api::Il2CppType));
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct RuntimeType(pub usize);

impl RuntimeType {
    cs_class!("System.RuntimeType");

    cs_property!(pub base_type, "get_BaseType", RuntimeType, self);
    cs_method!(_get_field, "GetField", &["string", "System.Reflection.BindingFlags"], RuntimeField, (name: Il2CppString, binding_flags: i32), self);

    #[inline(always)]
    pub fn get_field(&self, name: &str) -> anyhow::Result<Il2CppField> {
        match self._get_field(name.into(), 60) {
            Ok(field) => {
                if field.0 != 0 {
                    return Ok(field.get_il2cpp_field());
                } else {
                    let base_type = self.get_base_type()?;
                    let field = base_type._get_field(name.into(), 60)?;
                    if field.0 != 0 {
                        return Ok(field.get_il2cpp_field());
                    }
                }
            }
            Err(_) => {
                let base_type = self.get_base_type()?;
                let field = base_type._get_field(name.into(), 60)?;
                if field.0 != 0 {
                    return Ok(field.get_il2cpp_field());
                }
            }
        }

        Err(anyhow::format_err!(
            "no such field {} in {}",
            name,
            self.get_il2cpp_type().name()
        ))
    }

    #[inline(always)]
    pub fn from_class(class: Il2CppClass) -> Self {
        Self(Type::get_type_from_handle(class.byval_arg()).unwrap().0)
    }

    #[inline(always)]
    pub fn from_name(name: &str) -> Self {
        Self::from_class(get_cached_class(name).unwrap())
    }

    #[inline(always)]
    pub fn get_il2cpp_type(&self) -> Il2CppType {
        unsafe { Il2CppType(*((self.0 + 16) as *const usize)) }
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct RuntimeField(pub usize);

impl RuntimeField {
    #[inline(always)]
    pub fn get_il2cpp_field(&self) -> Il2CppField {
        unsafe { Il2CppField(*((self.0 + 24) as *const usize)) }
    }
}
