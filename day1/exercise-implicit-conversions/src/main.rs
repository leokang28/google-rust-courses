fn main() {
    let x = 10i8;
    let y = 5i16;
    // x type is i8 which is not expected for function add's arguments type
    // use x.into() will converse i8 to i16 because i16 has been implemented From<i8> in the standard lib
    // check in https://doc.rust-lang.org/std/convert/trait.From.html
    add(x.into(), y);
}

fn add(lhs: i16, rhs: i16) -> i16 {
    lhs + rhs
}
