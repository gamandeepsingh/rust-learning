use std::io;

fn main() {
    println!("Hello, world!");
    // io::stdin()::read_line(buf: &mut String)
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let weight:f32 = input.trim().parse().unwrap();
    println!("{}kg", calculate(weight));
}

fn calculate(w: f32) -> f32{
   (w/9.81) * 3.711
}