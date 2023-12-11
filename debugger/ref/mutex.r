std::sync::mutex::Mutex<u64> {
    inner: std::sys::unix::locks::futex_mutex::Mutex {
        futex: core::sync::atomic::AtomicU32 { value: 0u32 }
    },
    poison: std::sync::poison::Flag {
        failed: core::sync::atomic::AtomicBool { value: 0u8 }
    },
    data: core::cell::UnsafeCell<u64> {
        value: 1u64
    },
}
std::sync::rwlock::RwLock<u64> {
    inner: std::sys::unix::locks::futex_rwlock::RwLock {
        state: core::sync::atomic::AtomicU32 {
            value: 0u32
        },
        writer_notify: core::sync::atomic::AtomicU32 {
            value: 0u32
        }
    },
    poison: std::sync::poison::Flag {
        failed: core::sync::atomic::AtomicBool {
            value: 0u8
        }
    },
    data: core::cell::UnsafeCell<u64> { 
        value: 1u64
    }
}