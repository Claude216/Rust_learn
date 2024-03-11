fn main() {
    let f:f32 = 50.0;
    let c:f32 = 30.0;

    println!("f: {} to c: {}", f, f_to_c(f));
    println!("c: {} to f: {}", c, c_to_f(c));

    // println!("Hello, world!");
}

fn f_to_c(fa:f32)->f32 {
    (fa - 32.0) * 5.0 / 9.0
}

fn c_to_f(ce:f32)->f32 {
    ce * 9.0 / 5.0 + 32.0
}