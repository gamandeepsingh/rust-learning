trait Summary{
    fn summarise(&self) -> String;
}

struct User{
    name: String,
    age: u32,
}

impl Summary for User{
    fn summarise(&self) -> String{
        return format!("{} age is {}", self.name, self.age);
    }
}

fn main() {
    let user = User{
        name: String::from("Gamandeep singh"),
        age: 22
    };
    println!("{}", user.summarise());
}