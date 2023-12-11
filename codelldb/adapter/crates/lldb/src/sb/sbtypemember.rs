use super::*;

cpp_class!(pub unsafe struct SBTypeMember as "SBTypeMember");

unsafe impl Send for SBTypeMember {}

impl SBTypeMember {
    pub fn name(&self) -> &str {
        let ptr = cpp!(unsafe [self as "SBTypeMember*"] -> *const c_char as "const char*" {
            return self->GetName();
        });
        unsafe { get_str(ptr) }
    }
    pub fn type_(&self) -> SBType {
        cpp!(unsafe [self as "SBTypeMember*"] -> SBType as "SBType" {
            return self->GetType();
        })
    }
    pub fn r#type(&self) -> SBType {
        self.type_()
    }
}

impl IsValid for SBTypeMember {
    fn is_valid(&self) -> bool {
        cpp!(unsafe [self as "SBTypeMember*"] -> bool as "bool" {
            return self->IsValid();
        })
    }
}

impl fmt::Debug for SBTypeMember {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_descr(f, |descr| {
            cpp!(unsafe [self as "SBTypeMember*", descr as "SBStream*"] -> bool as "bool" {
                return self->GetDescription(*descr, eDescriptionLevelFull);
            })
        })
    }
}
