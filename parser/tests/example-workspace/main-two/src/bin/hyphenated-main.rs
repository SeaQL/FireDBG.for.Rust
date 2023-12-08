use std::fmt::{Display, Formatter};

mod pet {
    use super::*;

    #[derive(Clone)]
    pub struct Cat {
        pub name: String,
    }

    impl Cat {
        fn display(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} is a cat", self.name)
        }
    }

    impl Display for Cat {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            self.display(f)
        }
    }

    #[derive(Clone)]
    pub struct Dog {
        pub name: String,
    }

    impl Dog {
        fn display(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} is a dog", self.name)
        }
    }

    impl Display for Dog {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            self.display(f)
        }
    }
}

use pet::*;

fn inlined_display<T: Display>(animal: &T) {
    println!("{}", animal);
}

fn hyphenated_main() -> i32 {
    1
}

fn main() {
    let memo = Cat {
        name: "Memo".into(),
    };
    println!("{}", memo);
    inlined_display(&memo);

    hyphenated_main();
}
