#![allow(dead_code)]
#![allow(unused_imports)]

pub fn slice_string_by_space(s: &String) -> usize {
    /*
    write a function that takes a string of words separated by spaces and
    returns the first word it finds in that string
     */

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

pub fn slice_by_space_using_slicing(s: &String) -> &str { /* string slice, immutable */
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}