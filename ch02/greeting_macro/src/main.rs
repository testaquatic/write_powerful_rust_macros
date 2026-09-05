use greeting_macro::greeting;

fn main() {
    let greet = greeting!("Sam", "Hello");
    println!("{}", greet);

    let greet_with_default = greeting!("Sam");
    println!("{}", greet_with_default);
}
