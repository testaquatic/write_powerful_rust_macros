struct FirstName {
    value: String,
}



struct LastName {
    value: String,
}



struct Age {
    value: i32,
}

impl Age {
    pub fn new(age: i32) -> Result<Self, String> {
        if age < 0 || age > 150 {
            Err("Age should be between 0 and 150".to_string())
        } else {
            Ok(Self { value: age })
        }
    }
}

struct Pay {
    value: i32,
}

impl Pay {
    pub fn new(pay: i32) -> Result<Self, String> {
        if pay < 0 {
            Err("Pay should be positive".to_string())
        } else {
            Ok(Self { value: pay })
        }
    }
}

macro_rules! generate_get_value {
    ($struct_type:ident) => {
        generate_get_value!($struct_type, &str);
    };

    ($struct_type:ident, $return_type:ty) => {
        impl $struct_type {
            pub fn get_value(&self) -> $return_type {
                &self.value
            }
        }
    };
}

macro_rules! generate_try_from {
    ($type:ty => $return_type:ident) => {
        impl TryFrom<$type> for $return_type {
            type Error = String;
            fn try_from(value: $type) -> Result<Self, Self::Error> {
                $return_type::new(value)
            }
        }
    };
}

macro_rules! generate_newtype_methods {
    ($struct_type:ident, $input_type: ty) => {
        generate_get_value!($struct_type, &$input_type);
        generate_try_from!($input_type => $struct_type);
    };
}

generate_get_value!(FirstName);
generate_get_value!(LastName);
generate_newtype_methods!(Age, i32);
generate_newtype_methods!(Pay, i32);

fn calcullate_raise(
    first_name: &FirstName,
    last_name: &LastName,
    age: &Age,
    current_pay: &Pay,
) -> Pay {
    if first_name.get_value() == "Sam" {
        Pay::new(current_pay.get_value() + 1000).unwrap()
    } else {
        Pay::new(*current_pay.get_value()).unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
