use macro_crate::hello_world;

hello_world!(
    struct MyStruct {}
);

fn main() {
    let my_struct = MyStruct {};

    my_struct.hello_world();
}
