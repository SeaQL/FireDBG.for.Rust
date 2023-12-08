fn func() -> i32 {
    let a;
    a = "HELLO";

    let mut b;
    b = 1;
    b += 1; // addition
    b &= 1; // bitwise AND
    b |= 1; // bitwise OR
    b ^= 1; // bitwise XOR
    b /= 1; // division
    b *= 1; // multiplication
    b %= 1; // remainder
    b <<= 1; // left shift
    b >>= 1; // right shift
    b -= 1; // subtraction

    let c = "WORLD";

    let mut d = 0.14;
    d = d + 3.0;
    d += 3.0;

    let e = if true {
        println!("A `StatementType::Other`");
        1
    } else {
        2
    };

    const PI_CONST: f64 = 3.14;

    static PI_STATIC: f64 = 3.14;

    struct Object {
        name: String,
    }

    let mut obj = Object { name: "NAME".into() };
    obj.name = "NEW".into();

    struct Color(usize, usize, usize);

    let mut magic_color = Color(2023, 8, 31);
    magic_color.0 = 2023;
    magic_color.1 = 9;
    magic_color.2 = 1;

    #[derive(Default)]
    struct NestedA(usize, usize, usize, usize, NestedB);
    #[derive(Default)]
    struct NestedB(usize, usize, usize, NestedC);
    #[derive(Default)]
    struct NestedC(usize, usize, NestedD);
    #[derive(Default)]
    struct NestedD(usize, NestedE);
    #[derive(Default)]
    struct NestedE(usize);

    #[derive(Default)]
    struct NestedOf2(NestedE);
    #[derive(Default)]
    struct NestedOf3(NestedD);
    #[derive(Default)]
    struct NestedOf4(NestedC);
    #[derive(Default)]
    struct NestedOf5(NestedB);
    #[derive(Default)]
    struct NestedOf6(NestedA);

    let mut nested_of_2 = NestedOf2::default();
    nested_of_2.0.0 = 2;

    let mut nested_of_3 = NestedOf3::default();
    nested_of_3.0.1.0 = 3;

    let mut nested_of_4 = NestedOf4::default();
    nested_of_4.0.2.1.0 = 4;

    let mut nested_of_5 = NestedOf5::default();
    nested_of_5.0.3.2.1.0 = 5;

    let mut nested_of_6 = NestedOf6::default();
    nested_of_6.0.4.3.2.1.0 = 6;

    impl Object {
        const OBJ_E_CONST: f64 = 2.71;

        fn object_func(&self) -> f64 {
            let obj_e = if true {
                Self::OBJ_E_CONST
            } else {
                2.71
            };
            Self::OBJ_E_CONST
        }
    }

    trait CalculateE {
        const E_CONST: f64 = 2.71;

        fn cal_e() -> f64 {
            let cal_e = if true {
                Self::E_CONST
            } else {
                Object::OBJ_E_CONST
            };
            Self::E_CONST
        }
    }

    impl CalculateE for Object {
        fn cal_e() -> f64 {
            let obj_e = if true {
                Object::OBJ_E_CONST
            } else {
                <Object as CalculateE>::E_CONST
            };
            Object::OBJ_E_CONST
        }
    }

    let a = Some(1);

    let Some(_) = a else {
        unreachable!()
    };

    println!();

    123;

    pub fn assign_to_stmt() -> Vec<NestedE> {
        let mut vec = vec![NestedE::default()];
        vec.last_mut().unwrap().0 = 2;
        vec
    }

    1
}

use firedbg_protocol::source::*;
use firedbg_rust_parser::*;

pub fn get_breakpoints() -> Vec<FunctionDef> {
    vec![
        FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "func".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 1,
                    column: Some(19),
                },
                end: LineColumn {
                    line: 2,
                    column: Some(5),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "object_func".into(),
                parent_func: "func".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 89,
                    column: Some(39),
                },
                end: LineColumn {
                    line: 90,
                    column: Some(13),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "cal_e".into(),
                parent_func: "func".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 102,
                    column: Some(28),
                },
                end: LineColumn {
                    line: 103,
                    column: Some(13),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "cal_e".into(),
                parent_func: "func".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 113,
                    column: Some(28),
                },
                end: LineColumn {
                    line: 114,
                    column: Some(13),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "assign_to_stmt".into(),
                parent_func: "func".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 133,
                    column: Some(46),
                },
                end: LineColumn {
                    line: 134,
                    column: Some(9),
                },
            },
        },
    ]
}
