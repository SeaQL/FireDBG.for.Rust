use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SBFunctionId(pub usize);

cpp_class!(pub unsafe struct SBFunction as "SBFunction");

unsafe impl Send for SBFunction {}

impl SBFunction {
    /// The address of mangled name
    pub fn id(&self) -> SBFunctionId {
        let ptr = cpp!(unsafe [self as "SBFunction*"] -> *const c_char as "const void*" {
            return self->GetMangledName();
        });
        SBFunctionId(ptr as usize)
    }
    pub fn name(&self) -> &str {
        let ptr = cpp!(unsafe [self as "SBFunction*"] -> *const c_char as "const char*" {
            return self->GetName();
        });
        unsafe { get_str(ptr) }
    }
    pub fn display_name(&self) -> &str {
        let ptr = cpp!(unsafe [self as "SBFunction*"] -> *const c_char as "const char*" {
            return self->GetDisplayName();
        });
        unsafe { get_str(ptr) }
    }
    pub fn mangled_name(&self) -> &str {
        let ptr = cpp!(unsafe [self as "SBFunction*"] -> *const c_char as "const char*" {
            return self->GetMangledName();
        });
        unsafe { get_str(ptr) }
    }
    pub fn argument_name(&self, arg_idx: u32) -> &str {
        let ptr = cpp!(unsafe [self as "SBFunction*", arg_idx as "uint32_t"] -> *const c_char as "const char*" {
            return self->GetArgumentName(arg_idx);
        });
        unsafe { get_str(ptr) }
    }
    pub fn start_address(&self) -> SBAddress {
        cpp!(unsafe [self as "SBFunction*"] -> SBAddress as "SBAddress" {
            return self->GetStartAddress();
        })
    }
    pub fn end_address(&self) -> SBAddress {
        cpp!(unsafe [self as "SBFunction*"] -> SBAddress as "SBAddress" {
            return self->GetEndAddress();
        })
    }
    pub fn prologue_byte_size(&self) -> u32 {
        cpp!(unsafe [self as "SBFunction*"] -> u32 as "uint32_t" {
            return self->GetPrologueByteSize();
        })
    }
    pub fn type_(&self) -> SBType {
        cpp!(unsafe [self as "SBFunction*"] -> SBType as "SBType" {
            return self->GetType();
        })
    }
    pub fn r#type(&self) -> SBType {
        self.type_()
    }
}

impl fmt::Debug for SBFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_descr(f, |descr| {
            cpp!(unsafe [self as "SBFunction*", descr as "SBStream*"] -> bool as "bool" {
                return self->GetDescription(*descr);
            })
        })
    }
}
