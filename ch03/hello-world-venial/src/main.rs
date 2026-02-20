use hello_world_venial_macro::Hello;

#[derive(Hello)]
enum Hello {
    World,
}

fn main() {
    let hello = Hello::World;
    hello.hello_world();
}
