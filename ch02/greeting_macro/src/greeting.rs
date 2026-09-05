pub fn base_greeting_fn(name: &str, greeting: &str) -> String {
    format!("{}, {}!", greeting, name)
}

#[macro_export]
macro_rules! greeting {
    ($name: literal) => {
        greeting::base_greeting_fn($name, "Hello")
    };

    ($name: literal, $greeting: literal) => {
        greeting::base_greeting_fn($name, $greeting)
    };
}
