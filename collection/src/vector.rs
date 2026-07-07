pub fn vector() {
    let mut data_vector = Vec::new(); // one way to create a vector {its mutable so we could insert data later}
    data_vector.push(1);
    data_vector.push(2);
    data_vector.push(3);
    let third_item: &i32 = &data_vector[2]; // access data using index value
    println!("Third item in vector '{}'", third_item);

    let another_vector = vec![String::from("A"), String::from("B")]; // another way to create vector
    // using get method <vector_name>.get(<index>);
    let second_item: Option<&String> = another_vector.get(1);
    match second_item {
        Some(second_item) => println!("Second item in vector using get '{}'", second_item),
        None => println!("There is no second item in the vector"),
    }
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i);
    }
}
