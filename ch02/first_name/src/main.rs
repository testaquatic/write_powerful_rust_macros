struct FirstName {
    value: String,
}

struct LastName {
    value: String,
}

struct Age {
    value: i32,
}

struct Pay {
    value: i32,
}

macro_rules! generate_get_values {
    ($struct_type: ident) => {
        generate_get_values!($struct_type, String);
    };

    ($struct_type: ident, $return_type: ty) => {
        impl $struct_type {
            pub fn get_value(&self) -> &$return_type {
                &self.value
            }
        }
    };
}

generate_get_values!(FirstName);
generate_get_values!(LastName);
generate_get_values!(Age, i32);
generate_get_values!(Pay, i32);

fn calculate_raise(
    first_name: FirstName,
    _last_name: LastName,
    _age: Age,
    current_pay: Pay,
) -> Pay {
    if first_name.get_value() == "Sam" {
        Pay {
            value: current_pay.get_value() + 1000,
        }
    } else {
        current_pay
    }
}

fn main() {
    let raise = calculate_raise(
        FirstName {
            value: "Same".to_string(),
        },
        LastName {
            value: "Value".to_string(),
        },
        Age { value: 20 },
        Pay { value: 1000 },
    );
    println!("Raise: {}", raise.get_value());
}
