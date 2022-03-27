#[macro_use]
extern crate text_io;

fn main() {
    // read until a whitespace and try to convert what was read into an i32
    let i: i32 = read!();
    let u: i32 = read!();
    println!("Read in: {}", i);
    println!("Read in: {}", u);

    let  soma = i + u;

    println!("A soma é: {}", soma);
    

}
