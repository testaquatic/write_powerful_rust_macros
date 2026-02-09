pub fn base_greeting_fn(name: &str, greeting: &str) -> String {
    format!("{}, {}!", greeting, name)
}

#[macro_export]
macro_rules! greeting {
    ($name:literal) => {
        greeting::greeting_macro::base_greeting_fn($name, "Hello")
    };

    ($name:literal, $greeting:literal) => {
        greeting::greeting_macro::base_greeting_fn($name, $greeting)
    };

    (test $name: literal) => {{
        log_syntax!("The name passed to test is ", $name);
        println!("Returning default greeting");
        greeting::greeting_macro::base_greeting_fn($name, "Hello")
    }};
}
