use std::borrow::Cow;

use super::{native::Il2CppObject, util, GA_BASE};

macro_rules! il2cpp_api {
    ($index:expr, $name:ident($($arg_name:ident: $arg_type:ty),*) -> $ret_type:ty) => {
        #[allow(warnings)]
        #[inline(always)]
        pub fn $name($($arg_name: $arg_type,)*) -> $ret_type {
            unsafe {
                type FuncType = unsafe extern "fastcall" fn($($arg_type,)*) -> $ret_type;
                ::std::mem::transmute::<usize, FuncType>(
                    *((super::UP_BASE + super::API_BASE_PTR + 8 * $index) as *const usize)
                )($($arg_name,)*)
           }
        }
    };
}

il2cpp_api!(22, il2cpp_assembly_get_image(assembly: Il2CppAssembly) -> Il2CppImage);
il2cpp_api!(35, il2cpp_class_get_methods(klass: Il2CppClass, iter: *const *const usize) -> Il2CppMethod);
il2cpp_api!(37, il2cpp_class_get_name(klass: Il2CppClass) -> *const i8);
il2cpp_api!(49, il2cpp_class_from_type(r#type: Il2CppType) -> Il2CppClass);
il2cpp_api!(63, il2cpp_domain_get() -> Il2CppDomain);
il2cpp_api!(65, il2cpp_domain_get_assemblies(domain: Il2CppDomain, size: *mut usize) -> *mut Il2CppAssembly);
il2cpp_api!(73, il2cpp_field_get_name(field: Il2CppField) -> *const i8);
il2cpp_api!(77, il2cpp_field_get_value_object(field: Il2CppField, obj: Il2CppObject) -> Il2CppObject);
il2cpp_api!(117, il2cpp_method_get_name(method: Il2CppMethod) -> *const i8);
il2cpp_api!(123, il2cpp_method_get_param_count(method: Il2CppMethod) -> u32);
il2cpp_api!(124, il2cpp_method_get_param(method: Il2CppMethod, index: u32) -> Il2CppType);
il2cpp_api!(154, il2cpp_thread_attach(domain: Il2CppDomain) -> usize);
il2cpp_api!(161, il2cpp_type_get_name(r#type: Il2CppType) -> *const i8);
il2cpp_api!(169, il2cpp_image_get_class_count(image: Il2CppImage) -> usize);
il2cpp_api!(170, il2cpp_image_get_class(image: Il2CppImage, index: usize) -> Il2CppClass);

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Il2CppDomain(pub usize);

impl Il2CppDomain {
    #[inline(always)]
    pub fn assemblies(&self) -> Vec<Il2CppAssembly> {
        let mut count = 0;
        let assemblies = il2cpp_domain_get_assemblies(*self, &mut count);
        unsafe { std::slice::from_raw_parts(assemblies, count).to_vec() }
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Il2CppImage(pub usize);

impl Il2CppImage {
    #[inline(always)]
    pub fn class_count(&self) -> usize {
        il2cpp_image_get_class_count(*self)
    }

    #[inline(always)]
    pub fn classes(&self) -> Vec<Il2CppClass> {
        (0..self.class_count())
            .map(|index| il2cpp_image_get_class(*self, index))
            .collect()
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Il2CppAssembly(pub usize);

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Il2CppClass(pub usize);

impl Il2CppClass {
    #[inline(always)]
    pub fn name(&self) -> Cow<'static, str> {
        unsafe { util::cstr_to_str(il2cpp_class_get_name(*self)) }
    }

    #[inline(always)]
    pub fn byval_arg(&self) -> Il2CppType {
        Il2CppType(self.0 + 128)
    }

    #[inline(always)]
    pub fn methods(&self) -> Vec<Il2CppMethod> {
        let iter = std::ptr::null();
        let mut result = Vec::new();
        loop {
            let method = il2cpp_class_get_methods(*self, &iter);
            if method.0 == 0 {
                break;
            }
            result.push(method)
        }
        result
    }

    #[inline(always)]
    pub fn find_method_by_name(&self, name: &str) -> Option<Il2CppMethod> {
        self.methods()
            .into_iter()
            .find(|&method| method.name() == name)
    }

    #[inline(always)]
    pub fn find_method(&self, name: &str, arg_types: &[&str]) -> Option<Il2CppMethod> {
        for method in self.methods() {
            if method.name() == name {
                let count = method.args_cnt() as usize;
                if count == arg_types.len() {
                    let mut fail = false;
                    for (i, arg_type) in arg_types.iter().enumerate() {
                        if *arg_type != method.arg_type_formatted(i as u32) {
                            fail = true;
                            break;
                        }
                    }

                    if !fail {
                        return Some(method);
                    }
                }
            }
        }
        None
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Il2CppType(pub usize);

impl Il2CppType {
    #[inline(always)]
    pub fn name(&self) -> Cow<'static, str> {
        unsafe { util::cstr_to_str(il2cpp_type_get_name(*self)) }
    }

    #[inline(always)]
    pub fn formatted_name(&self) -> String {
        let name = self.name();

        (match name.as_ref() {
            "System.Int32" => "int",
            "System.UInt32" => "uint",
            "System.Int16" => "short",
            "System.UInt16" => "ushort",
            "System.Int64" => "long",
            "System.UInt64" => "ulong",
            "System.Byte" => "byte",
            "System.SByte" => "sbyte",
            "System.Boolean" => "bool",
            "System.Single" => "float",
            "System.Double" => "double",
            "System.String" => "string",
            "System.Char" => "char",
            "System.Object" => "object",
            "System.Void" => "void",
            "System.Decimal" => "decimal",
            "System.DateTime" => "DateTime",
            other => other,
        })
        .to_string()
    }

    #[inline(always)]
    pub fn class(&self) -> Il2CppClass {
        il2cpp_class_from_type(*self)
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Il2CppMethod(pub usize);

impl Il2CppMethod {
    #[inline(always)]
    pub fn name(&self) -> Cow<'static, str> {
        unsafe { util::cstr_to_str(il2cpp_method_get_name(*self)) }
    }

    #[inline(always)]
    pub fn class(&self) -> Il2CppClass {
        unsafe { *((self.0) as *const Il2CppClass) }
    }

    #[inline(always)]
    pub fn va(&self) -> usize {
        unsafe { *((self.0 + 8) as *const usize) }
    }

    #[inline(always)]
    pub fn rva(&self) -> usize {
        let va = self.va();
        if va == 0 {
            return 0;
        }
        unsafe { self.va() - GA_BASE }
    }

    #[inline(always)]
    pub fn args_cnt(&self) -> u32 {
        il2cpp_method_get_param_count(*self)
    }

    #[inline(always)]
    pub fn arg(&self, i: u32) -> Il2CppType {
        il2cpp_method_get_param(*self, i)
    }

    #[inline(always)]
    pub fn arg_type_formatted(&self, i: u32) -> String {
        self.arg(i).formatted_name()
    }

    #[inline(always)]
    pub fn format_params(&self) -> String {
        use std::fmt::Write;
        let param_count = il2cpp_method_get_param_count(*self);
        let name = self.name();
        let mut out = String::with_capacity(0);

        let _ = write!(out, "{name}(");
        for param_index in 0..param_count {
            let param = il2cpp_method_get_param(*self, param_index);
            let _ = write!(out, "{}", param.class().byval_arg().formatted_name());

            if param_index + 1 < param_count {
                let _ = write!(out, ",");
            }
        }
        let _ = write!(out, ")");

        out
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Il2CppField(pub usize);

impl Il2CppField {
    #[inline(always)]
    pub fn name(&self) -> Cow<'static, str> {
        unsafe { util::cstr_to_str(il2cpp_field_get_name(*self)) }
    }

    #[inline(always)]
    pub fn get_value(&self, instance: Il2CppObject) -> Option<Il2CppObject> {
        let value = il2cpp_field_get_value_object(*self, instance);
        if value.0 == 0 {
            None
        } else {
            Some(value)
        }
    }
}
