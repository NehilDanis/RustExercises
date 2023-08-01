#![allow(dead_code)]
#![allow(unused_imports)] 

pub fn match_statement() {
    let country_code: i32 = 44;

    let country: &str = match country_code {
        44 => "UK",
        46 => "Sweden",
        49 => "Germany",
        90 => "Turkey",
        1..=1000 => "unknown", // [1, 1000] 
        _ => "invalid" // inf you comment this line, you will 
        // get an error, since Rust is smart enough to realize you 
        // did not cover all the cases
    };

    println!("the country with code {} is {}", country_code, country);
}