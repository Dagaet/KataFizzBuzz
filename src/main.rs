fn main() {
    for x in 1..100 {
        if x % 3 == 0 {
            println!("Fizz")
        } else if x.to_string().contains("3") && x.to_string().contains("5") {
            if x % 5 == 0 {
                println!("FizzBuzzBuzz")
            } else {
                println!("FizzBuzz")
            }
        } else if x % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", x);
        };
    }
}
