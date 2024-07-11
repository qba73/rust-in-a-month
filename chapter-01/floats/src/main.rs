fn main() {
    println!("Ch 1.5 - Floats");

    let my_float = 5.;

    // two diffeerent floats

    println!("{}", my_float);

    let my_fl_1: f32 = 5.0;
    let my_fl_2: f64 = 23.2;

    // Error if my_fl_2 not cased to f32!
    let float_sum = my_fl_1 + my_fl_2 as f32;
    println!("{}", float_sum)
}
