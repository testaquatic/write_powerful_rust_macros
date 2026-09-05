macro_rules! hello_world {
    ($world: ty) => {
        impl $world {
            fn hello_world(&self) {
                println!("hello world")
            }
        }
    };
}

struct Example {}

hello_world!(Example);

fn main() {
    let e = Example {};
    e.hello_world();
}
