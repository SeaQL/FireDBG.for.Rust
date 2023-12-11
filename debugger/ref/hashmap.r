std::collections::HashMap<&str, u32, RandomState> {
    base: hashbrown::map::HashMap<&str, u32, RandomState> {
        hash_builder: RandomState {
            k0: 16630950511546250859u64,
            k1: 6925738688493646258u64,
        },
        table: hashbrown::raw::RawTable<(&str, u32)> {
            table: hashbrown::raw::RawTableInner<alloc::alloc::Global> {
                bucket_mask: 7usize,
                ctrl: core::ptr::non_null::NonNull<u8> {
                    pointer: &62u8,
                }
                growth_left: 3usize,
                items: 4usize,
                alloc: alloc::alloc::Global {}
            },
            marker: core::marker::PhantomData<(&str, u32)> {}
        }
    }
}