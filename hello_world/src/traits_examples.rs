#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt::Debug; // debug trait

trait Animal {
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result: i32 = 0;
        for x in self {
            result += *x;
        }
        return result;
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Square {
    side: f64,
}

trait Shape{
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

// fn print_info(shape: impl Shape + Debug) 
// fn print_info<T: Shape + Debug>(shape: T)
fn print_info<T>(shape: T) 
    where T: Shape + Debug
{
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

// Into trait

struct Person {
    name: String
}

impl Person {
    // fn new(name: &str) -> Person {
    //     Person { name: name.to_string() }
    // }

    fn new<T: Into<String>> (name: T) -> Person
    // fn new<T> (name: T) -> Person
    //     where T: Into<String>
    {
        Person{name: name.into()}
    }
}

// drop trait -- functionaly equivalent to destructor


// try to implement ADL


mod lib {
    pub trait Processable {
        fn process(self);
    }
    
    pub struct Impl1 {}
    impl Processable for Impl1 {
        fn process(self) {
            println!("Impl1");
        }
    }
    
    pub fn generalize<T: Processable>(t: T) { t.process(); }
}

mod client {
    pub struct Impl2 {}
    impl super::lib::Processable for Impl2 {
        fn process(self) {
            println!("Impl2");
        }
    }
}

// drop trait

struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} entered the game", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}

pub fn drop_trait () {
    let _goblin = Creature::new("Jeff");
    println!("Game proceeds..");
}

// operator overloading
// done using traits

use std::ops::Add;

struct Complex<T> 
{
    re: T,
    im: T
}

impl<T> Complex<T> {
    // fn new(re: T, im: T) -> Complex<T> {
    //     Complex(re, im)
    // }
}


// static dispatch --> templated functions

// dynamic dispatch --> polymorphism



pub fn traits_examples() {
    let h = Human{name: "John"};
    h.talk();

    let c = Cat{name: "Misty"};
    c.talk();

    // lets add traits to values we dont even own
    let a = vec![1, 2, 3];
    println!("Sum = {}", a.sum()); // there is no sum defined for vector

    let c = Circle{radius: 2.0};
    print_info(c);

    let _john = Person::new("John");
    let name = "Jane".to_string();
    let _jane = Person::new(name);

    let impl1 = lib::Impl1{};
    lib::generalize(impl1);

    let impl2 = client::Impl2{};
    lib::generalize(impl2);

}