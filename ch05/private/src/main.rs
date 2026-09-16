use private_macro::private;

private!(
    #[derive(Clone, Debug)]
    struct Example<T> {
        pub string_value: String,
        pub number_value: i32,
        pub t_value: T,
    }
);

fn main() {
    println!("Hello, world!");
}
