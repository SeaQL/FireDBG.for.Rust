use super::*;

cpp_class!(pub unsafe struct SBTypeEnumMember as "SBTypeEnumMember");

unsafe impl Send for SBTypeEnumMember {}

impl SBTypeEnumMember {
    pub fn name(&self) -> &str {
        let ptr = cpp!(unsafe [self as "SBTypeEnumMember*"] -> *const c_char as "const char*" {
            return self->GetName();
        });
        unsafe { get_str(ptr) }
    }
    pub fn type_(&self) -> SBType {
        cpp!(unsafe [self as "SBTypeEnumMember*"] -> SBType as "SBType" {
            return self->GetType();
        })
    }
    pub fn r#type(&self) -> SBType {
        self.type_()
    }
}

impl IsValid for SBTypeEnumMember {
    fn is_valid(&self) -> bool {
        cpp!(unsafe [self as "SBTypeEnumMember*"] -> bool as "bool" {
            return self->IsValid();
        })
    }
}

impl fmt::Debug for SBTypeEnumMember {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_descr(f, |descr| {
            cpp!(unsafe [self as "SBTypeEnumMember*", descr as "SBStream*"] -> bool as "bool" {
                return self->GetDescription(*descr, eDescriptionLevelFull);
            })
        })
    }
}

cpp_class!(pub unsafe struct SBTypeEnumMemberList as "SBTypeEnumMemberList");

unsafe impl Send for SBTypeEnumMemberList {}

impl SBTypeEnumMemberList {
    pub fn number_of_members(&self) -> u32 {
        cpp!(unsafe [self as "SBTypeEnumMemberList*"] -> u32 as "uint32_t" {
            return self->GetSize();
        })
    }
    pub fn member_at_index(&self, index: u32) -> SBTypeEnumMember {
        cpp!(unsafe [self as "SBTypeEnumMemberList*", index as "uint32_t"] -> SBTypeEnumMember as "SBTypeEnumMember" {
            return self->GetTypeEnumMemberAtIndex(index);
        })
    }
    pub fn members<'a>(&'a self) -> impl Iterator<Item = SBTypeEnumMember> + 'a {
        SBIterator::new(self.number_of_members(), move |index| {
            self.member_at_index(index)
        })
    }
}
