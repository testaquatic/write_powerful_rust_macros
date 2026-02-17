struct FirstName {
    value: String,
}

impl FirstName {
    pub fn new(name: &str) -> Result<FirstName, String> {
        if name.len() < 2 {
            Err("Name should be at least two characters".into())
        } else {
            Ok(FirstName { value: name.into() })
        }
    }
}

struct LastName {
    value: String,
}

impl LastName {
    pub fn new(name: &str) -> Result<FirstName, String> {
        if name.len() < 2 {
            Err("Name should be at least two characters".into())
        } else {
            Ok(FirstName { value: name.into() })
        }
    }
}

macro_rules! generate_get_value {
    ($struct_type: ident) => {
        generate_get_value!($struct_type, String);
    };
    ($struct_type: ident, $return_type: ty) => {
        impl $struct_type {
            pub fn get_value(&self) -> &$return_type {
                &self.value
            }
        };
    };
}

generate_get_value!(FirstName);
generate_get_value!(LastName);

struct Age {
    value: i32,
}

impl Age {
    pub fn new(age: i32) -> Result<Age, String> {
        if age >= 150 {
            Err("Age should be not exceed 150.".into())
        } else {
            Ok(Age { value: age })
        }
    }
}

struct Pay {
    value: i32,
}

impl Pay {
    pub fn new(pay: i32) -> Result<Pay, String> {
        if pay < 0 {
            Err("Pay should be positive.".into())
        } else {
            Ok(Pay { value: pay })
        }
    }
}

fn calculate_raise(
    first_name: FirstName,
    _last_name: LastName,
    _age: Age,
    current_pay: Pay,
) -> i32 {
    if first_name.value == "Sam" {
        current_pay.value + 1_000
    } else {
        current_pay.value
    }
}

fn main() {
    let first_raise = calculate_raise("Smith".to_string(), "Sam".into(), 20, 1000);
    println!("{}", first_raise);

    let second_raise = calculate_raise("Sam".into(), "Smith".into(), 1000, 20);
    println!("{}", second_raise);
}
