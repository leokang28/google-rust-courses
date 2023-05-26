use std::mem::{align_of, size_of};

macro_rules! dbg_size {
    ($t:ty) => {
        println!(
            "{}: size {} bytes, align: {} bytes",
            stringify!($t),
            size_of::<$t>(),
            align_of::<$t>()
        );
    };
}

enum Foo {
    A,
    B,
}
enum WebEvent {
    PageLoad,                 // Variant without payload
    KeyPress(char),           // Tuple struct variant
    Click { x: i64, y: i64 }, // Full struct variant
}

// #[rustfmt::skip]
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::KeyPress(c) => println!("pressed '{c}'"),
        WebEvent::Click { x, y } => println!("clicked at x={x}, y={y}"),
    }
}

fn main() {
    let load = WebEvent::PageLoad;
    let press = WebEvent::KeyPress('x');
    let click = WebEvent::Click { x: 20, y: 80 };

    inspect(load);
    inspect(press);
    inspect(click);

    dbg_size!(Foo);
}
