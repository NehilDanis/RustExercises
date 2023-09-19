#![allow(dead_code)]
#![allow(unused_imports)] 

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

pub fn traits_examples() {
    let h = Human{name: "John"};
    h.talk();

    let c = Cat{name: "Misty"};
    c.talk();

    // lets add traits to values we dont even own
    let a = vec![1, 2, 3];
    println!("Sum = {}", a.sum()); // there is no sum defined for vector
}