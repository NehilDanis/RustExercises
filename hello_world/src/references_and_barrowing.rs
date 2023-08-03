/*
Giving ownership and taking it back, when everytime 
a function call happened is tidious, instead we could use
references
*/

#![allow(dead_code)]
#![allow(unused_imports)]


fn sent_as_reference(s: &String) -> usize { // s is a reference to a String
    s.len()
} // s goes out of the scope, but since it does not have the ownership to 
// what it refers to, it does not dropped.


fn change_value(s: &mut String) {
    s.push_str(", how are you");
}


pub fn references_and_barrowing() {

    /*
    If the type which implements the drop traits is tried to be sent
    as a parameter to another function, then it will be moved. This way
    unnecessary copies will not happen. However since the variable moved 
    from is dropped, it won't be usable anymore. 

    So if we have a variable and we sent it as a parameter into another 
    function, this variable will not have the ownership of the value in the heap
    and we won't be able to use it after the function call. 

    Instead we can sent a reference to the value in the heap to the function, this way
    the actual variable will not lose the ownership.
    */

    // Example 1 - immutable refs

    let s1 = String::from("Hello");
    let length_s1 = sent_as_reference(&s1);

    println!("The length of the string {} is {}", s1, length_s1);


    // Example 2 - mutable refs
    let mut s1 = String::from("Hello");
    change_value(&mut s1);
    let length_s1 = sent_as_reference(&s1);
    println!("The length of the string {} is {}", s1, length_s1);


    /*
    If you have one mutable reference to a variable, you cannot have 
    any other reference to the same variable, regardless it is mutable
    or immutable. 

    Because if you have two mutable references to the same variable, it will
    create racing conditions

    If you have one mutable and one immutable reference to a variable, the person who
    has the immutable reference will not expect it to be changed. However, it can be
    changed by the mutable reference. 

    Hence the rule is not to have more than one reference to the same variable 
    at the same time.
    */

    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; --> cannot barrow s more than one time as a mutable ref.

    println!("{}", r1);
    // println!("{}", r2)

    // you can have more than one immutable references to the same variable in 
    // the same scope.

    let s3 = &s1;
    let s4 = &s1;

    println!("s3 = {}, s4 = {}", s3, s4);

    let s5 = & mut s1;
    println!("s5 = {}", s5);

}