fn main() {
    let mut some_str = String::from("I am a variable");
    borrow_ownership(&mut some_str); // passing the reference of some_str
    some_str = returns_ownership(some_str); // This will return the string so some_str will remain the owner of the string
    // -------------------------------------------------------------------------------------------------------------
    let ref_of_ss = &mut some_str;
    println!(
        "{} <- this is ref to the original string and doesn't takes the ownership",
        ref_of_ss
    );
    // let another_ref_of_ss = &mut some_str; <- this will throw an error as we can have only one mutable reference and n numbers of immutable refs
    take_ownership(some_str);
    // println!("{}", some_str)  this will throw an error as ownership is transferred to the function
}

fn take_ownership(str: String) {
    println!("Dropped the owner '{}'", str)
}

fn borrow_ownership(str: &mut String) {
    println!("Not the variable but a reference of '{}'", &str)
}

fn returns_ownership(str: String) -> String {
    println!("Giving back the ownership by returning the value");
    str
}
