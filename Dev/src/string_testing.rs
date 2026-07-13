
/// Calculate the length of a string.
pub fn calculate_length(s: &String) -> usize {
    s.len()
}

/// Appending to strings
pub fn append_test() {
    // Strings can be modified easily
    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

    let mut test_string = String::from("Test");
    test_string.push_str(", Appending to strings.");
    println!("{test_string}");

    // Strings can be cloned like this
    let test_string2 = test_string.clone();
    println!("{test_string2}");
}

/// Slicing strings
pub fn slice_test() {
    // Slices
    // https://doc.rust-lang.org/book/ch04-03-slices.html

    let mut test_string = String::from("Test");
    // This is like what I used to do in Python, this is using references.
    // let text_slice = &test_string[0..4];

    // This is the same as the above, the 0 can be ommited from this if starting at 0.
    let text_slice = &test_string[..4];

    println!("{}", text_slice);
}

/// Referencing strings, which can be used for just about any variable.
pub fn references_test() {
    // Refrences and borrowing
    // https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

    let mut test_string = String::from("Test");

    let len = calculate_length(&test_string);
    println!("The length of '{}' is {}.", test_string, len);
}






