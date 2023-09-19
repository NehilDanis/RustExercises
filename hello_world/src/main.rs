#![allow(dead_code)]
#![allow(unused_imports)] 
// the lines above are used to prevent compiler warnings

use std::mem;
mod stack_and_heap;
mod match_statement;
mod ownership;
mod references_and_barrowing;
mod slicing;
mod data_structures;
mod traits_examples;

fn core_data_types() {
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

fn operators() {
    let mut a = 2 + 3 * 4;
    println!("{}", a);

    a = a + 1;
    a += 1; // ++ or -- operators do not exist in Rust

    // also there is no power operator, you need to use the build in function
    let a_cubed: i32 = i32::pow(a, 3);
    println!("{}", a_cubed);

    // in case you want to take the floating number power of another number
    // you need to use some approximation

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi: f64 = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}",b , b_cubed, b, b_to_pi);

    // bitwise operators

    let c = 1 | 2;
    println!("1|2 = {}", c);

    let two_to_10: i32 = 1 << 10; // left shift multiplies by 2^n;
    println!("2^n = {}", two_to_10);

    // logical operators

    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    let v = 5;
    let v_is_5 = v == 5;

    println!("Pi is less then 4 = {}", pi_less_4);
    println!("v is equal to 5 = {}", v_is_5);


}

fn while_loop() {
    let mut x: i16 = 1;
    while x < 1000 {
        x *= 2;

        if x == 64 {
            continue;
        }
        println!("x = {}", x);
    }

    let mut y = 1;

    loop // equavalent of while true
    {
        y *= 2;
        println!("y = {}", y);

        if y == 1 << 10 { break; }
    } 
}

fn for_loop() {
    // print all values from 1 to 10
    for ind in 1..11 {
        println!("ind = {}", ind);
    }

    for (ind, y) in (30..41).enumerate() {
        // ind is in [0, 11)
        // while y is in [30, 41)
        println!("{}: {}", ind, y);
    }
}

fn main() {
    // core_data_types();
    // operators();
    // stack_and_heap::stack_and_heap_fn();
    // while_loop();
    // for_loop();
    // match_statement::match_statement();
    // ownership::ownership();
    //references_and_barrowing::references_and_barrowing();

    // let mut s  = String::from("Hello world!");

    // let word = slicing::slice_by_space_using_slicing(&s);

    // s.clear();  --> compile error, cannot borrow `s` as mutable because it is also borrowed as immutable

    // println!("First word is {}", &word);

    //data_structures::structures();

    traits_examples::traits_examples();

}
