#![allow(dead_code)]
#![allow(unused_imports)] 

use std::mem;

struct Point{
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{x: 0.0, y: 0.0} // implisit return statement
    // return Point{x: 0.0, y: 0.0}; // explicit return statement
}

pub fn stack_and_heap_fn() {
    let p1 = origin();
    let p2 = Box::new(origin());

    println!("In the stack p1 takes up to {} bytes", std::mem::size_of_val(&p1));
    println!("In the stack p2 takes up tp {} bytes", std::mem::size_of_val(&p2));

    println!("The value pf p2 x: {}, y: {}", (*p2).x, (*p2).y);
}

