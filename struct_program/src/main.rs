fn main() {

    Struct User {
        name:&Str,
        age:i32
    };

    let a = User{
        name="Mohsin",
        age:25

    };

    println!("Welcome {}",a);
}
