use std::path::PathBuf;

fn func(v: &PathBuf) {
    println!("{v:?}");
}

fn main() {
    let mut path = PathBuf::new();
    path.push(r"/");
    path.push("home");
    path.push("hello");
    func(&path);
}
