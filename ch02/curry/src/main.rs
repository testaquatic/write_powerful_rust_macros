macro_rules! curry {
    (|$first_arg: ident $(, $arg: ident)* | $body: expr) => {
        move |$first_arg| {
            $(move |$arg|)* {
                $body
            }
        }
    };
}

fn main() {
    let add = curry!(|x, y, z| x - y - z);
    let result = add(1)(2)(3);
    println!("{}", result);
}
