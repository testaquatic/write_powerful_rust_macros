use hello_world_macro::Hello;

#[derive(Hello)]
struct Example;

#[derive(Hello)]
enum Pet {
    Cat,
}

fn main() {
    let e = Example;
    e.hello_world();

    let p = Pet::Cat;
    p.hello_world();
}
