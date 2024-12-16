// 1. Create a function that takes an integer as an argument and returns true if the integer is even, false if it is not.
fn main() {
    println!("{}",is_even(234567890));
}

fn is_even(i:i32) -> bool{
    if i%2==0{
        return true;
    }
    return false;
}
