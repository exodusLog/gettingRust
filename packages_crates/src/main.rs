// Crate is rust code file
// Two types of crates: binary & library
// A binary crate are complied to an executable. Each must have a function "main" that defines what happens when the executable runs
// A library crate doesn't have a main function and it goes like src/lib.rs, it doesn't compile to an executable. They defines functionality intended to be shared with multiple projects.
// by default `cargo new <project_name> creates binary application, to create library crate use --lib flag -> cargo new <project_name> --lib
// You can create as many binary crate you want but a project will have only one lib crate
// To have multiple binary crate place your binary crate in src/bin directory

mod normal_module {
    fn private_function() {
        println!("A private function")
    }
    pub fn public_function() {
        println!(
            "A public function which can be accessed outside the mod using normal_module::public_function();"
        )
    }
    pub fn indirect_pvt_access() {
        println!("Accessed private function, call me by normal_module::indirect_pvt_access();");
        private_function();
    }
    pub mod nested {
        pub fn function() {
            println!("called `normal_module::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `normal_module::nested::private_function()`");
        }

        // Functions declared using `pub(in path)` syntax are only visible
        // within the given path. `path` must be a parent or ancestor module
        pub(in crate::normal_module) fn public_function_in_my_mod() {
            print!("called `normal_module::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // Functions declared using `pub(self)` syntax are only visible within
        // the current module, which is the same as leaving them private
        pub(self) fn public_function_in_nested() {
            println!("called `normal_module::nested::public_function_in_nested()`");
        }

        // Functions declared using `pub(super)` syntax are only visible within
        // the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called `normal_module::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `normal_module::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `normal_module::public_function_in_crate()`");
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `normal_module::private_nested::function()`");
        }

        // Private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `normal_module::private_nested::restricted_function()`");
        }
    }
}
fn main() {
    println!("ez!!");
    normal_module::public_function();
    normal_module::indirect_pvt_access();
    normal_module::public_function_in_crate();
    normal_module::call_public_function_in_my_mod();
    normal_module::nested::function();
}
