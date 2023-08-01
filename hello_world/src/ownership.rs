/*
Understanding ownership
Rust makes memory safety guarantees without needing
a garbage collector

barrowing, slces, ..
*/

#![allow(dead_code)]
#![allow(unused_imports)]

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



}