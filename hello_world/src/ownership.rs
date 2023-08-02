/*
Understanding ownership
Rust makes memory safety guarantees without needing
a garbage collector

barrowing, slces, ..
*/

#![allow(dead_code)]
#![allow(unused_imports)]

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("some_string = {}", some_string);
}   // here some_string goes out of the scope, and the drop is called on it
    // hence the backing memory is deleted.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("some_integer = {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

pub fn ownership() {

    /*
    There are many ways to do memory management
    - garbage collector
    - programmer explicity allocates and frees memory
    - In Rust --> memory is managed through a system of 
    ownership with set of rules that are checked by the 
    compiler. If any of the rules are violates, the program
    won't compile.
    Also ownership does not slow down your program while running!
    
    The ownership rules in Rust
    - Each value has an owner
    - There can only be a one owner at a time (essentially unique_ptr of C++)
    - When owner goes out of scope, the value will be dropped.

    When the owner goes out of scope, rust calls drop on the variable. (similarly RAII)
    
    */

    // Example 1
    let s1 = String::from("hello"); // allocated in the heap
    let s2 = s1;

    /*
    Since these variables are are both on the heap, Rust does not want the deep copy
    since it would be expensive. 

    So it does shallow copy then?
    Not really, in case of a shallow copy you would get the double deletion issue, when the 
    variables go out of the scope. So Rust, instead moves the ownership from one variable to 
    another. In the above case s1 will be moved into s2. s1 will be invalid and in case you
    try using s1, you will get a compile error.

    In cpp, it would not give compile error.
    */

    println!("{}", s2);

    // in case we really want to do a deep copy, clone method can be used
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);

    // Example 2

    /*
    For basic types copying directly happens, since the size of the variable is already known
    in the compile time,  it will be kept in the stack as well.
    */
    let x =  5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // if a type uses a drop trait (type that is allocated in the heap), then it won't have copy trait
    // as a general rule, any group of simple scalar values can implement copy trait


    // Example 3 

    let s4 =  String::from("hey!");
    takes_ownership(s4);


    let z = 3;
    makes_copy(z);

}

