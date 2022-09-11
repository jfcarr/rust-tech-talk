extern "C" {
    fn square(x: f32) -> f32;
}

fn main() {
    let input_value: f32 = 10.0;

    unsafe {
        println!("The square of {} is {}", input_value, square(input_value));
    }
}
