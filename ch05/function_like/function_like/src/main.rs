fn add_one(n: i32) -> i32 {
    n + 1
}

fn stringfy(n: i32) -> String {
    n.to_string()
}

fn main() {
    let composed = function_like_compose_macro::compose!(add_one.add_one.stringfy);
    println!("{:?}", composed(3));
}
