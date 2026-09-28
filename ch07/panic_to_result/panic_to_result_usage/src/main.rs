use panic_to_result_macro::panic_to_result;

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
}

#[panic_to_result]
fn create_person_two_issues(name: String, age: u32) -> Result<String, Person> {
    if age > 30 {
        panic!();
    }
    Ok(Person { name, age })
}

fn main() {}

#[cfg(test)]
mod tests {

    #[test]
    fn happy_path() {
        let actual = create_person("Sam".to_string(), 22).unwrap();

        assert_eq!(actual.name, "Sam");
        assert_eq!(actual.age, 22);
    }

    #[test]
    fn should_panic_on_invalid_age() {
        let actual = create_persion("S".to_string(), 32);

        assert_eq!(
            actual.expect_err("this should be an error"),
            "I hope I die before I get old".to_string()
        )
    }
}
