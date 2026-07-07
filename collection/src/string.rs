pub fn string() {
    let mut empty_string = String::new();
    let some_string = "Some string";
    empty_string = some_string.to_string();

    let mut any_string = String::from("I am any string.");
    any_string.push_str("Pushe"); // push_str adds a whole string
    any_string.push('d'); // push only adds a character
    // No ownership is transferred in any of these methods above

    let owner1 = String::from("Hi");
    let owner2 = String::from("bye");
    let final_owner = owner1 + &owner2; // owner1 ownership has been moved and no more accessible after this point
    // this is just a demo to concat a sting

    println!(
        "Not an empty string anymore {} and {}",
        empty_string, any_string
    );
    println!("Final owner: {}, owner2: {}", final_owner, owner2);
}
