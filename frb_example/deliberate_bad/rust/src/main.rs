use std::env;

fn main() {
    match &env::args().skip(1).next().unwrap()[..] {
        "Good" => println!("This is good code"),
        "StackBufferOverflow" => {
            // example from https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html
            // and https://github.com/japaric/rust-san
            let xs = [0, 1, 2, 3];
            let y = unsafe { *xs.as_ptr().offset(4) };
            println!("xs={xs:?} y={y}");
        }
        s => panic!("Unknown mode: {}", s),
    }
}
