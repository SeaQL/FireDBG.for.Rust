pub use std::rc::Rc;

fn main() {
    let mut integers = vec![];
    map_append(&mut integers);
}

fn map_append(vec: &mut Vec<Rc<i32>>) -> &mut Vec<Rc<i32>> {
    let mut tmp = [1, 2, 3].iter().map(|c| Rc::new(c + 1)).collect();
    vec.append(&mut tmp);
    vec
}
