pub fn say_hello() {
    say_greeting("Hello".to_string());
}

pub fn say_goodbye() {
    say_greeting("Goodbye".to_string());
}

fn say_greeting(prefix: String) {
    println!("{}!", prefix);
}
