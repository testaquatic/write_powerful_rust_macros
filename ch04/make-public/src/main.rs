use make_public_macro::public;
use make_public_macro_field_unnamed::make_public_macro_field_unnamed;
use make_public_macro2::public2;
use make_public_macro3::public3;
use make_public_macro4::public4;

#[public]
#[derive(Debug)]
struct Example {
    first: String,
    pub second: String,
}

#[public2]
struct Example2 {
    first: String,
    pub second: String,
}

#[public3]
struct Example3 {
    first: String,
    pub second: String,
}

#[public4]
struct Example4 {
    first: String,
    pub second: String,
}

#[make_public_macro_field_unnamed]
#[derive(Debug, PartialEq)]
struct Example5 {
    first: String,
    pub second: String,
}

#[make_public_macro_field_unnamed]
struct Example6(String, pub String);

#[make_public_macro_field_unnamed]
#[derive(Debug, PartialEq)]
enum Example7 {
    UnNamedFields(String, pub String),
    NamedFields { first: String, second: String },
    Unit,
}

#[make_public_macro_field_unnamed]
struct Example8;

#[make_public_macro_field_unnamed]
#[derive(Debug, PartialEq)]
enum Example9 {}

fn main() {}
