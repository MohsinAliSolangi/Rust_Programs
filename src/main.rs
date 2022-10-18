use rand ::{thread_rng,Rng};
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Gessing Game");
    let secret_number = rand::thread_rng().gen_range(1..101); //limit between 1 to 101
    println!("this is Secret Number : {} ",secret_number);
loop{
    println!("Please input your guess");
    let mut  guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("Your gussed: {} ",guess);
    let guess:u32 = guess.trim().parse().expect("type an integer");
    match guess.cmp(&secret_number){
        Ordering::Less =>println!("To Small "),
        Ordering::Greater=>println!("Big Value Guess"),
        Ordering::Equal=>{
        println!("congrates You win Game ");
        break;
    }, 
    }
}

}
