// traits are just like interfaces
// it shares a common method for multiple stucts (atleast this is what I understood)

use std::println;

trait About {
    fn about(&self) -> String;
}

struct Userdata {
    name: String,
    gender: String,
    major: bool,
}

struct Education {
    course: String,
    year: i32,
    current_year: i32,
}

impl About for Userdata {
    fn about(&self) -> String {
        let data = format!("Student {}, have major {}", self.name, self.major);
        data
    }
}

impl About for Education {
    fn about(&self) -> String {
        let data = format!(
            "Education data: course {}, overall years: {}, student current year: {}",
            self.course, self.year, self.current_year
        );
        data
    }
}

fn display_data(source: &impl About) {
    println!("{}", source.about());
}

trait Secondary {
    fn random_print(&self) {
        println!("A secondary trait")
    }
}

impl Secondary for Userdata {}

fn fn_with_multiple_traits<T: Secondary + About>(item: &T) {
    item.random_print();
}
fn another_way(item: &(impl Secondary + About)) {}

fn some_function<T: About + Secondary, U: About>(t: &T, u: &U) -> i32 {
    44
}
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Secondary + About,
    U: About,
{
    32
}

fn main() {
    let user = Userdata {
        name: String::from("Alok"),
        gender: String::from("male"),
        major: true,
    };

    let education = Education {
        course: String::from("CSE"),
        year: 4,
        current_year: 3,
    };
    println!("{}", education.about());
    display_data(&user);
    fn_with_multiple_traits(&user);
}
