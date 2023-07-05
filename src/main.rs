fn main() {
    for x in 0..100 {
        println!("{}", fizz_buzz(x));
    }
}

fn fizz_buzz(x: i32) -> String {
    let divisible_by_three = x % 3 == 0;
    let divisible_by_five = x % 5 == 0;
    let result: String;

    if divisible_by_three && divisible_by_five {
        result = String::from("FizzBuzz");
        return result;
    } else if x.to_string().contains("3") && x.to_string().contains("5") {
        if divisible_by_five {
            result = String::from("FizzBuzzBuzz");
            return result;
        }
    } else if divisible_by_three {
        result = String::from("Fizz");
        return result;
    } else if divisible_by_five {
        result = String::from("Buzz");
        return result;
    }
    return x.to_string();

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz_for_3() {
        assert_eq!(fizz_buzz(3), "Fizz");
    }

    #[test]
    fn test_fizzbuzz_for_5() {
        assert_eq!(fizz_buzz(5), "Buzz");
    }

    #[test]
    fn test_fizzbuzz_for_15() {
        assert_eq!(fizz_buzz(15), "FizzBuzz");
    }

    #[test]
    fn test_fizzbuzz_for_19() {
        assert_eq!(fizz_buzz(19), "19");
    }



}
