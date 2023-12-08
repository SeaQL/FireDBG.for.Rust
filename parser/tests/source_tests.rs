mod common;

pub use firedbg_rust_parser::*;
pub use pretty_assertions::assert_eq;

// TODO: we need a better test on parsing the whole workspace

/*

#[test]
fn parse_test_directory() {
    let directory = concat!(env!("CARGO_MANIFEST_DIR"), "/tests");
    let mut files = parse_directory(directory);
    assert_eq!(files.len(), 12);
    let mut i = I { i: 0 };

    files[i.cur()].functions.pop(); // Pop the last `fn get_breakpoints` breakpoint
    assert_file(
        &files[i.next()],
        "/parser/tests/common/free_fn.rs",
        common::free_fn::get_breakpoints(),
    );

    files[i.cur()].functions.pop(); // Pop the last `fn get_breakpoints` breakpoint
    assert_file(
        &files[i.next()],
        "/parser/tests/common/impl_fn.rs",
        common::impl_fn::get_breakpoints(),
    );

    files[i.cur()].functions.pop(); // Pop the last `fn get_breakpoints` breakpoint
    assert_file(
        &files[i.next()],
        "/parser/tests/common/impl_trait.rs",
        common::impl_trait::get_breakpoints(),
    );

    assert_file(&files[i.next()], "/parser/tests/common/mod.rs", []);

    files[i.cur()].functions.pop(); // Pop the last `fn get_breakpoints` breakpoint
    assert_file(
        &files[i.next()],
        "/parser/tests/common/nested_fn.rs",
        common::nested_fn::get_breakpoints(),
    );

    files[i.cur()].functions.pop(); // Pop the last `fn get_breakpoints` breakpoint
    assert_file(
        &files[i.next()],
        "/parser/tests/common/result_fn.rs",
        common::result_fn::get_breakpoints(),
    );

    assert_file(
        &files[i.next()],
        "/parser/tests/free_fn_tests.rs",
        [FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "parse_free_fn".into(),
                is_async: false,
                return_type: false,
            },
            loc: LineColumn {
                line: 7,
                column: 21,
            },
        }],
    );

    assert_file(
        &files[i.next()],
        "/parser/tests/impl_fn_tests.rs",
        [FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "parse_impl_fn".into(),
                is_async: false,
                return_type: false,
            },
            loc: LineColumn {
                line: 7,
                column: 21,
            },
        }],
    );

    assert_file(
        &files[i.next()],
        "/parser/tests/impl_trait_tests.rs",
        [FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "parse_impl_trait".into(),
                is_async: false,
                return_type: false,
            },
            loc: LineColumn {
                line: 7,
                column: 24,
            },
        }],
    );

    assert_file(
        &files[i.next()],
        "/parser/tests/nested_fn_tests.rs",
        [FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "parse_nested_fn".into(),
                is_async: false,
                return_type: false,
            },
            loc: LineColumn {
                line: 7,
                column: 23,
            },
        }],
    );

    assert_file(
        &files[i.next()],
        "/parser/tests/result_fn_tests.rs",
        [FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "parse_result_fn".into(),
                is_async: false,
                return_type: false,
            },
            loc: LineColumn {
                line: 7,
                column: 23,
            },
        }],
    );
}

struct I {
    i: usize,
}

impl I {
    fn cur(&self) -> usize {
        self.i
    }

    fn next(&mut self) -> usize {
        let i = self.i;
        self.i += 1;
        i
    }
}

fn assert_file<I>(file: &File, path: &str, breakpoints: I)
where
    I: IntoIterator<Item = FunctionDef> + std::fmt::Debug,
    Vec<FunctionDef>: PartialEq<I>,
{
    assert!(file.path.ends_with(path));
    assert_eq!(file.functions, breakpoints);
}

*/
