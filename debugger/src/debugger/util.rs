// The `line` look like this
// 1.1: where = vec_map.o`alloc::alloc::Global::grow_impl + 1119 at vec_map.rs:1:1, address = vec_map.o[0x000000010000624f], unresolved, hit count = 0
// 1.2: where = vec_map.o`alloc::raw_vec::finish_grow + 415 at vec_map.rs:1:1, address = vec_map.o[0x00000001000064cf], unresolved, hit count = 0
// 1.3: where = vec_map.o`alloc::raw_vec::finish_grow + 643 at vec_map.rs:1:1, address = vec_map.o[0x00000001000065b3], unresolved, hit count = 0
// 1.4: where = vec_map.o`alloc::raw_vec::RawVec<T,A>::grow_amortized + 258 at vec_map.rs:1:1, address = vec_map.o[0x0000000100006d12], unresolved, hit count = 0
//                       ^                                           ^^^
// 2.1: where = vec_map.o`vec_map::main + 11 at vec_map.rs:6:27, address = vec_map.o[0x0000000100007bbb], unresolved, hit count = 0
//                       ^             ^^^
// 3.1: where = hyphenated-main`<hyphenated_main::pet::Cat as core::fmt::Display>::fmt + 16 at hyphenated-main.rs:19:13, address = hyphenated-main[0x0000000100003360], unresolved, hit count = 0
//                             ^                                                      ^^^
// We want to split by ` and +, and get the fully quantify function name without any < prefix character
pub(super) fn get_bp_fn_full_name(line: &str) -> Option<&str> {
    if let Some((_, after)) = line.split_once('`') {
        if let Some((between, _)) = after.split_once(" + ") {
            let full_name = between
                .trim_start_matches('<') // Trim generic: `<hyphenated_main::pet::Cat as core::fmt::Display>::fmt`
                .trim_end_matches("::{{closure}}"); // Trim closure: `graph::test_directed_graph::test_add_edge::{{closure}}`
            return Some(full_name);
        }
    }
    None
}

// The `line` look like this
// 2.1: where = vec_map.o`vec_map::main + 11 at vec_map.rs:6:27, address = vec_map.o[0x0000000100007bbb], unresolved, hit count = 0
//           ^^^         ^
// 1.1: where = tail_call.o`tail_call::head + 11 at tail_call.rs:2:11, address = tail_call.o[0x000000010000424b], unresolved, hit count = 0
//           ^^^           ^
// We want to split by = and `, then get the executable name
pub(super) fn get_executable_name(line: &str) -> Option<&str> {
    if let Some((_, after)) = line.split_once(" = ") {
        if let Some((between, _)) = after.split_once("`") {
            return Some(between);
        }
    }
    None
}

// The `line` look like this
// 1.1: where = tail_call.o`tail_call::head + 11 at tail_call.rs:2:11, address = tail_call.o[0x000000010000424b], unresolved, hit count = 0
//              ^^^^^^^^^^^ ^^^^^^^^^
// 14.1: where = simple_tests-dcf6ffbfe2a6ebdb`simple_tests::main_one_test + 11 at simple_tests.rs:5:5, address = simple_tests-dcf6ffbfe2a6ebdb[0x0000000100003a4b], unresolved, hit count = 0
//               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ^^^^^^^^^^^^
// 15.1: where = simple_tests-dcf6ffbfe2a6ebdb`quick_sort::conquer::partition + 39 at conquer.rs:2:17, address = simple_tests-dcf6ffbfe2a6ebdb[0x0000000100002557], unresolved, hit count = 0
//               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ^^^^^^^^^^
// 3.1: where = hyphenated-main`<hyphenated_main::pet::Cat as core::fmt::Display>::fmt + 16 at hyphenated-main.rs:19:13, address = hyphenated-main[0x0000000100003360], unresolved, hit count = 0
//              ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^
// We want to know if it's a function defined in the executable module
pub(super) fn is_executable_fn(line: &str) -> bool {
    if let Some(fn_full_name) = get_bp_fn_full_name(line) {
        if let Some(executable_name) = get_executable_name(line) {
            // exec_prefix =
            // 1.1: where = hyphenated-main`hyphenated_main::pet::Cat::display
            //                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            // `hyphenated_main::pet::Cat::display` => `hyphenated_main`
            if let Some((exec_prefix, _)) = fn_full_name.split_once(':') {
                // executable_name.replace('-', "_") =
                // 1.1: where = hyphenated-main`hyphenated_main::pet::Cat::display
                //              ^^^^^^^^^^^^^^^
                // `hyphenated-main` => `hyphenated_main`
                if executable_name.replace('-', "_").starts_with(exec_prefix) {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_bp_fn_full_name() {
        assert_eq!(get_bp_fn_full_name("1.1: where = tail_call.o`tail_call::head + 11 at tail_call.rs:2:11, address = tail_call.o[0x000000010000424b], unresolved, hit count = 0").unwrap(), "tail_call::head");
        assert_eq!(get_bp_fn_full_name("2.1: where = tail_call.o`tail_call::inter + 11 at tail_call.rs:6:5, address = tail_call.o[0x000000010000428b], unresolved, hit count = 0").unwrap(), "tail_call::inter");
        assert_eq!(get_bp_fn_full_name("3.1: where = tail_call.o`tail_call::tail + 11 at tail_call.rs:10:5, address = tail_call.o[0x00000001000042ab], unresolved, hit count = 0").unwrap(), "tail_call::tail");
        assert_eq!(get_bp_fn_full_name("4.1: where = tail_call.o`tail_call::end + 9 at tail_call.rs:15:2, address = tail_call.o[0x00000001000042e9], unresolved, hit count = 0").unwrap(), "tail_call::end");
        assert_eq!(get_bp_fn_full_name("5.1: where = tail_call.o`tail_call::main + 11 at tail_call.rs:18:15, address = tail_call.o[0x00000001000042fb], unresolved, hit count = 0").unwrap(), "tail_call::main");
        assert_eq!(get_bp_fn_full_name("6.1: where = tail_call.o`tail_call::main + 442 at tail_call.rs:21:2, address = 0x00000001000044aa, resolved, hit count = 0").unwrap(), "tail_call::main");
        assert_eq!(get_bp_fn_full_name("7.1: where = tail_call.o`tail_call::head + 36 at tail_call.rs:3:2, address = 0x0000000100004264, resolved, hit count = 0").unwrap(), "tail_call::head");
        assert_eq!(get_bp_fn_full_name("8.1: where = tail_call.o`tail_call::inter + 21 at tail_call.rs:7:2, address = 0x0000000100004295, resolved, hit count = 0").unwrap(), "tail_call::inter");
        assert_eq!(get_bp_fn_full_name("9.1: where = tail_call.o`tail_call::tail + 36 at tail_call.rs:11:2, address = 0x00000001000042c4, resolved, hit count = 0").unwrap(), "tail_call::tail");
        assert_eq!(get_bp_fn_full_name("10.1: where = tail_call.o`tail_call::end + 10 at tail_call.rs:15:2, address = 0x00000001000042ea, resolved, hit count = 0").unwrap(), "tail_call::end");
        assert_eq!(get_bp_fn_full_name("3.1: where = hyphenated-main`<hyphenated_main::pet::Cat as core::fmt::Display>::fmt + 16 at hyphenated-main.rs:19:13, address = hyphenated-main[0x0000000100003360], unresolved, hit count = 0").unwrap(), "hyphenated_main::pet::Cat as core::fmt::Display>::fmt");
        assert_eq!(get_bp_fn_full_name("20.1: where = graph-4fa5fcab6738bc6e`graph::test_directed_graph::test_add_edge + 25 at lib.rs:179:25, address = graph-4fa5fcab6738bc6e[0x0000000100003a09], unresolved, hit count = 0").unwrap(), "graph::test_directed_graph::test_add_edge");
        assert_eq!(get_bp_fn_full_name("20.2: where = graph-4fa5fcab6738bc6e`graph::test_directed_graph::test_add_edge::{{closure}} + 20 at lib.rs:178:24, address = graph-4fa5fcab6738bc6e[0x0000000100014304], unresolved, hit count = 0").unwrap(), "graph::test_directed_graph::test_add_edge");
    }

    #[test]
    fn test_get_executable_name() {
        assert_eq!(get_executable_name("1.1: where = tail_call.o`tail_call::head + 11 at tail_call.rs:2:11, address = tail_call.o[0x000000010000424b], unresolved, hit count = 0").unwrap(), "tail_call.o");
        assert_eq!(get_executable_name("2.1: where = tail_call.o`tail_call::inter + 11 at tail_call.rs:6:5, address = tail_call.o[0x000000010000428b], unresolved, hit count = 0").unwrap(), "tail_call.o");
        assert_eq!(get_executable_name("3.1: where = tail_call.o`tail_call::tail + 11 at tail_call.rs:10:5, address = tail_call.o[0x00000001000042ab], unresolved, hit count = 0").unwrap(), "tail_call.o");
        assert_eq!(get_executable_name("13.1: where = simple_tests-dcf6ffbfe2a6ebdb`simple_tests::main_one_test::{{closure}} + 20 at simple_tests.rs:4:20, address = simple_tests-dcf6ffbfe2a6ebdb[0x00000001000037d4], unresolved, hit count = 0").unwrap(), "simple_tests-dcf6ffbfe2a6ebdb");
        assert_eq!(get_executable_name("14.1: where = simple_tests-dcf6ffbfe2a6ebdb`simple_tests::main_one_test + 11 at simple_tests.rs:5:5, address = simple_tests-dcf6ffbfe2a6ebdb[0x0000000100003a4b], unresolved, hit count = 0").unwrap(), "simple_tests-dcf6ffbfe2a6ebdb");
        assert_eq!(get_executable_name("15.1: where = simple_tests-dcf6ffbfe2a6ebdb`quick_sort::conquer::partition + 39 at conquer.rs:2:17, address = simple_tests-dcf6ffbfe2a6ebdb[0x0000000100002557], unresolved, hit count = 0").unwrap(), "simple_tests-dcf6ffbfe2a6ebdb");
        assert_eq!(get_executable_name("1.1: where = hyphenated-main`hyphenated_main::pet::Cat::display + 27 [inlined] core::fmt::rt::Argument::new_display at rt.rs:97:22, address = hyphenated-main[0x00000001000032eb], unresolved, hit count = 0").unwrap(), "hyphenated-main");
        assert_eq!(get_executable_name("2.1: where = hyphenated-main`hyphenated_main::pet::Cat::display + 54 at hyphenated-main.rs:13:13, address = hyphenated-main[0x0000000100003306], unresolved, hit count = 0").unwrap(), "hyphenated-main");
        assert_eq!(get_executable_name("3.1: where = hyphenated-main`<hyphenated_main::pet::Cat as core::fmt::Display>::fmt + 16 at hyphenated-main.rs:19:13, address = hyphenated-main[0x0000000100003360], unresolved, hit count = 0").unwrap(), "hyphenated-main");
        assert_eq!(get_executable_name("5.1: where = another_main`another_main::main + 11 at another_main.rs:5:5, address = another_main[0x00000001000031eb], unresolved, hit count = 0").unwrap(), "another_main");
        assert_eq!(get_executable_name("6.1: where = another_main`another_main::main_one_another_main + 4 at another_main.rs:18:2, address = another_main[0x0000000100003384], unresolved, hit count = 0").unwrap(), "another_main");
        assert_eq!(get_executable_name("7.1: where = another_main`main_one::main_one_common_fn + 4 at lib.rs:3:2, address = another_main[0x0000000100003d94], unresolved, hit count = 0").unwrap(), "another_main");
        assert_eq!(get_executable_name("40.1: where = another_main`quick_sort::conquer::partition + 39 at conquer.rs:2:17, address = another_main[0x0000000100003967], unresolved, hit count = 0").unwrap(), "another_main");
        assert_eq!(get_executable_name("41.1: where = another_main`quick_sort::divide::quick_sort + 47 at divide.rs:5:8, address = another_main[0x0000000100002d9f], unresolved, hit count = 0").unwrap(), "another_main");
        assert_eq!(get_executable_name("42.1: where = another_main`quick_sort::run_quick_sort + 24 at lib.rs:8:15, address = another_main[0x00000001000033d8], unresolved, hit count = 0").unwrap(), "another_main");
    }

    #[test]
    fn test_is_executable_fn() {
        assert!(is_executable_fn("1.1: where = tail_call.o`tail_call::head + 11 at tail_call.rs:2:11, address = tail_call.o[0x000000010000424b], unresolved, hit count = 0"));
        assert!(is_executable_fn("2.1: where = tail_call.o`tail_call::inter + 11 at tail_call.rs:6:5, address = tail_call.o[0x000000010000428b], unresolved, hit count = 0"));
        assert!(is_executable_fn("3.1: where = tail_call.o`tail_call::tail + 11 at tail_call.rs:10:5, address = tail_call.o[0x00000001000042ab], unresolved, hit count = 0"));
        assert!(is_executable_fn("13.1: where = simple_tests-dcf6ffbfe2a6ebdb`simple_tests::main_one_test::{{closure}} + 20 at simple_tests.rs:4:20, address = simple_tests-dcf6ffbfe2a6ebdb[0x00000001000037d4], unresolved, hit count = 0"));
        assert!(is_executable_fn("14.1: where = simple_tests-dcf6ffbfe2a6ebdb`simple_tests::main_one_test + 11 at simple_tests.rs:5:5, address = simple_tests-dcf6ffbfe2a6ebdb[0x0000000100003a4b], unresolved, hit count = 0"));
        assert!(!is_executable_fn("15.1: where = simple_tests-dcf6ffbfe2a6ebdb`quick_sort::conquer::partition + 39 at conquer.rs:2:17, address = simple_tests-dcf6ffbfe2a6ebdb[0x0000000100002557], unresolved, hit count = 0"));
        assert!(is_executable_fn("1.1: where = hyphenated-main`hyphenated_main::pet::Cat::display + 27 [inlined] core::fmt::rt::Argument::new_display at rt.rs:97:22, address = hyphenated-main[0x00000001000032eb], unresolved, hit count = 0"));
        assert!(is_executable_fn("2.1: where = hyphenated-main`hyphenated_main::pet::Cat::display + 54 at hyphenated-main.rs:13:13, address = hyphenated-main[0x0000000100003306], unresolved, hit count = 0"));
        assert!(is_executable_fn("3.1: where = hyphenated-main`<hyphenated_main::pet::Cat as core::fmt::Display>::fmt + 16 at hyphenated-main.rs:19:13, address = hyphenated-main[0x0000000100003360], unresolved, hit count = 0"));
        assert!(is_executable_fn("5.1: where = another_main`another_main::main + 11 at another_main.rs:5:5, address = another_main[0x00000001000031eb], unresolved, hit count = 0"));
        assert!(is_executable_fn("6.1: where = another_main`another_main::main_one_another_main + 4 at another_main.rs:18:2, address = another_main[0x0000000100003384], unresolved, hit count = 0"));
        assert!(!is_executable_fn("7.1: where = another_main`main_one::main_one_common_fn + 4 at lib.rs:3:2, address = another_main[0x0000000100003d94], unresolved, hit count = 0"));
        assert!(!is_executable_fn("40.1: where = another_main`quick_sort::conquer::partition + 39 at conquer.rs:2:17, address = another_main[0x0000000100003967], unresolved, hit count = 0"));
        assert!(!is_executable_fn("41.1: where = another_main`quick_sort::divide::quick_sort + 47 at divide.rs:5:8, address = another_main[0x0000000100002d9f], unresolved, hit count = 0"));
        assert!(!is_executable_fn("42.1: where = another_main`quick_sort::run_quick_sort + 24 at lib.rs:8:15, address = another_main[0x00000001000033d8], unresolved, hit count = 0"));
    }
}
