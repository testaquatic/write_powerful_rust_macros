#![feature(trace_macros)]
#![feature(log_syntax)]

macro_rules! count {
    ($val: expr) => {
        if $val == 1 { 1 } else { count!($val - 1) }
    };
}

fn main() {
    trace_macros!(true);
    count!(5);
    trace_macros!(false);
}
