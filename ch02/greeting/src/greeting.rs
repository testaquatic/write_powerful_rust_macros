pub fn base_greeting_fn(name: &str, greeting: &str) -> String {
    format!("{}, {}", greeting, name)
}

#[macro_export]
macro_rules! greeting {
    ($name: literal) => {
        $crate::greeting::base_greeting_fn($name, "Hello")
    };
    ($name: literal, $greeting: literal) => {
        $crate::greeting::base_greeting_fn($name, $greeting)
    };
    (test $name: literal) => {{
        log_syntax!("The name passed to test is ", $name);
        println!("Returing defualt greetings");
        $crate::greeting::base_greeting_fn($name, "Hello");
    }};
}
