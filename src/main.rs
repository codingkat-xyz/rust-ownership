fn main() {
    // 1. Ownership basics
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // println!("{}", s1); // Error: s1 is no longer valid
    println!("{}", s2);

    // 2. Clone
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // 3. Borrowing
    let s5 = String::from("rust");
    let len = calculate_length(&s5);
    println!("The length of '{}' is {}.", s5, len);

    // 4. Mutable references
    let mut s6 = String::from("hello");
    change(&mut s6);
    println!("{}", s6);

    // 5. Lifetime example
    let result;
    let string1 = String::from("long string");
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
