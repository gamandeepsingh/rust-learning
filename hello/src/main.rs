// --------------------------------------------------------------------------------------------- //
// 1. Create a function that takes an integer as an argument and returns true if the integer is even, false if it is not.
// fn main() {
//     println!("{}",is_even(234567890));
// }

// fn is_even(i:i32) -> bool{
//     if i%2==0{
//         return true;
//     }
//     return false;
// }


// --------------------------------------------------------------------------------------------- //
//  A `fib` function that calculates the Fibonacci number for a given integer using recursion.
// fn main(){
//     let a:i32=10;
//     let b:i32=20;
//     println!("{} + {} = {}",a,b,a+b);
//     for i in 0..10 {    
//         println!("{}",fib(i));
//     }
// }

// fn fib(num:i32)->i32{
//     if num<=1 {
//         return num;
//     }
//     else{
//         return fib(num-1)+fib(num-2);
//     }
// }

// --------------------------------------------------------------------------------------------- //
// A function that takes a string as an argument and returns a length of the string.
// fn main(){
//     let my_string = String::from("Hello world");
//     let size=get_string_length(&my_string);
//     println!("the length of the `{}` is {}",my_string,size);
// }

// fn get_string_length(s: &str)-> usize{
//     s.chars().count()
// }


// --------------------------------------------------------------------------------------------- //
// Implementaion of Structure using `struct` keyword in Rust with methods using `impl` keyword
struct User{
    name:String,
    email:String,
    mobile_number:i32,
    sign_in_count:u64,
    active:bool
}

// here `self` is the instance of the structure and `&self` is the reference of the instance like `this` in JS
impl User {
    fn display_all(&self){
        println!("Username: {}",self.name);
        println!("Email: {}",self.email);
        println!("Mobile Number: {}",self.mobile_number);
        println!("SignIn Count: {}",self.sign_in_count);
        println!("Active: {}",self.active);
    }

    fn email_name(&self,i:i32){
        println!("{}{}{}",self.email,i,self.name.to_lowercase().replace(" ", ""));
    }

    fn update_email(&mut self,email:String){
        self.email=email;
    }

    // static method : we can call this method without creating an instance of the structure i.e. User::debug()
    fn debug(){
        println!("This is a static method");
    }
}

fn main(){
    let user1:User = User{
        active:true,
        email:String::from("gamandeep@gmail.com"),
        mobile_number:1234567890,
        name:String::from("Gamandeep Singh"),
        sign_in_count:60
    };
    user1.display_all();
    user1.email_name(10);
    let mut user2:User = User{
        active:true,
        email:String::from("gamandeep"),
        mobile_number:1234567890,
        name:String::from("Gamandeep Singh"),
        sign_in_count:60
    };
    user2.update_email(String::from("gaman@gmail.com"));
    User::debug()  // calling static method
}


// --------------------------------------------------------------------------------------------- //