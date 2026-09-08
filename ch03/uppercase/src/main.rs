use uppercase_macro::UpperCaseName;

#[derive(UpperCaseName)]
struct Example {}

fn main() {
    let a = Example {};
    a.upppercase();
    Example::testing_tesing();
    a.hello();
}
