#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
    grade: u32,
    alive: bool,
}
// Structs allows you to group related data together
// For ts background people it could be called as an interface.

fn main() {
    let student1 = Student {
        name: String::from("Adam"),
        age: 19,
        grade: 50,
        alive: false,
    };
    // Created an instance of struct Student, now we can access the data of student1 using dot notaion
    let name = student1.name;
    println!(
        "My name is {} age: {}, grade: {}",
        name, student1.age, student1.grade
    ); // if we will not use the remaining values then it will show us never read warning. Try running code till here to witness that.

    // Create an mutable student record so we could change the data
    let mut student2 = Student {
        name: String::from("eve"),
        age: 18,
        grade: 50,
        alive: true,
    };
    student2.alive = false;
    println!("Is {} alive ?: {}", student2.name, student2.alive);

    let student3 = add_student(String::from("Zeus"), 12); // creating student using a function that returns Student
    // let student4 = Student {
    //     name: String::from("Thor"),
    //     ..student1 // except name all other values will be copied from student1
    // };
    println!("Data of student3: {:#?}", student3)
}

fn add_student(name: String, grade: u32) -> Student {
    Student {
        name,
        age: 18,
        grade,
        alive: true,
    }
}
