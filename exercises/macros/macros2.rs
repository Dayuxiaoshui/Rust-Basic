// macros2.rs

// Define the macro first
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // Call the macro
    my_macro!();
}
