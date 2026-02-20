use hello_world_macros_alt::Hello;

#[derive(Hello)]
struct Hello;

fn main() {
    let hello = Hello;
    hello.hello_world();
}
