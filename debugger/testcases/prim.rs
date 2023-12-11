fn main() {
    let is_true: bool = true;
    let not_true: bool = false;
    let unicode: char = 'A';
    let byte: u8 = 1;
    let signed_byte: i8 = -1;
    let short: u16 = 2;
    let signed_short: i16 = -2;
    let int: u32 = 3;
    let signed_int: i32 = -3;
    let long: u64 = 4;
    let signed_long: i64 = -4;
    let index: usize = 5;
    let signed_index: isize = -5;
    let long_long: u128 = 6;
    let signed_long_long: i128 = -6;
    let float: f32 = 3.1415927410125732421875; // exact
    let double: f64 = 3.14159265359000006156975359772332012653350830078125; // exact
    let unit: () = ();
    let array: [u8; 5] = [1, 2, 3, 4, 5];
    let slice: &[u8] = array.as_slice();
    let bytes: Vec<u8> = vec![5, 6, 7, 8, 9];
    let str_lit: &str = "hello world";
    let string: String = format!("{str_lit}!");
    let bytes_empty: Vec<u8> = vec![];
    let string_vec_empty: Vec<String> = vec![];
    let bytes_empty_slice = bytes_empty.as_slice();
    let string_vec_empty_slice = string_vec_empty.as_slice();
    let string_empty = String::new();
    let str_lit_empty: &str = &string_empty;

    dbg!(is_true);
    dbg!(not_true);
    dbg!(unicode);
    dbg!(byte);
    dbg!(signed_byte);
    dbg!(short);
    dbg!(signed_short);
    dbg!(int);
    dbg!(signed_int);
    dbg!(long);
    dbg!(signed_long);
    dbg!(long_long);
    dbg!(signed_long_long);
    dbg!(index);
    dbg!(signed_index);
    dbg!(float);
    dbg!(double);
    dbg!(unit);
    dbg!(array);
    dbg!(slice);
    dbg!(&bytes); // I observed that when the vec is dropped, the variable is still in scope but it reads garbage
    dbg!(str_lit);
    dbg!(&string);
    dbg!(&bytes_empty);
    dbg!(&string_vec_empty);
    dbg!(&bytes_empty_slice);
    dbg!(&string_vec_empty_slice);
    dbg!(&string_empty);
    dbg!(&str_lit_empty);

    println!();
}
