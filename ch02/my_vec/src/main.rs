macro_rules! my_vec {
    () => {
        Vec::new()
    };
    (make an empty vec) => (
        Vec::new()
    );
    ($x: expr) => {
        {
           vec![$x]
        }
    };
    ($($x: expr),+$(,)?) => {
        {
            vec![$($x),+]
        }
    };
}

fn main() {
    let empty: Vec<i32> = my_vec!();
    println!("{:?}", empty);

    let also_empty: Vec<i32> = my_vec!(make an empty vec);
    println!("{:?}", also_empty);

    let three_numbers = my_vec!(1, 2, 3);
    println!("{:?}", three_numbers);

    let last_comma = my_vec!(1,2, 3,);
    println!("{:?}", last_comma);
}
