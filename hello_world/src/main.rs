#![allow(dead_code)]
#![allow(unused_imports)] 
// the lines above are used to prevent compiler warnings

use std::mem;

fn main() {
    let _x = 0; // unused variable, if the above allow dead_code 
                    // statement was not here, then the compiler would give 
                    // warning

    let _y: u8 = 123; // unsigned, 8 bits
                      // if you intentionally want to have unused variable then
                      // add an underscore before the variable name.
    
    
    let a: u8 = 123;
    println!("a = {}", a); // {} inside the print macro represents that 
                           // the value of a will be substituted in the 
                           // place of curly braces.

    // let binding represents immutable variable
    // in Rust to create mutable variable you need to specify it as below

    let mut b: i8 = 123;
    println!("Value of b before = {}", b);
    b = -123;
    println!("Value of b after = {}", b);

    let c: i32 = 123456789;
    println!("Size of c is {} byte(s)", mem::size_of_val(&c));

    // usize, isize

    let z: isize = 123;
    let size_of_z: usize = mem::size_of_val(&z);

    println!("z = {}, and it takes up to {} byte(s), hence this is {} bits OS", z, size_of_z, size_of_z * 8); 

    // char
    let d: char = 'x';
    println!("d = {}, and its size {} byte(s)", d, mem::size_of_val(&d));

    // f32 f64 --> single precision and double precision floating point numbers
    // IEEE 745 - signed

    let f: f32 = 2.4;
    println!("Value of f is {} and its size {} byte(s)",f, mem::size_of_val(&f));

    // boolean
    let g: bool = false;
    println!("Value of g {},  it size is {} byte(s)", g, mem::size_of_val(&g));
    
}
