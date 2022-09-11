fn main() {
    println!("10 + 5 is {}", add(10, 5));
}

fn add(first_number: i32, second_number: i32) -> i32 {
    return first_number + second_number;
}

// The #[cfg(test)] attribute indicates that the module contains test functions.
#[cfg(test)]
mod tests;
