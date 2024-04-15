fn main() {
    main_three(16);
    main_three(18);
    main_two(10, 5);
    main_four("Monday");
    main_five("Saturday");
    main_five("Monday");
    main_six(18);
    main_seven("admin");
    main_seven("user");
}

//if else if
fn main_two(a: i64, b: i64) {
    if a > b {
        println!("A ({}) is greater than B ({})!", a, b);
    } else if a == b {
        println!("A ({}) is equal to B ({})!", a, b);
    } else {
        println!("A ({}) is less than B ({})!", a, b);
    }
}

//if else
fn main_three(age: i32) {
    if age >= 18 {
        println!("You are Eligible ! ({})", age);
    } else {
        println!("You are not Eligible! Please wait for ({})", 18 - age);
    }
}

//if else if else with String
fn main_four(today: &str) {
    if today == "Monday" {
        println!("Today Is ({})", today);
    } else if today == "Tuesday" {
        println!("Today Is ({})", today);
    } else if today == "Friday" {
        println!("Today Is ({})", today);
    } else {
        println!("Invalid Day ({})", today);
    }
}

//String match (switch case)
fn main_five(today: &str) {
    match today {
        "Monday" => println!("Today Is ({})", today),
        "Friday" => println!("Today Is ({})", today),
        "Tuesday" => println!("Today Is ({})", today),
        _ => println!("Invalid Day ({})", today),
    }
}

//String match with if else (switch case)
fn main_six(today: i32) {
    match today {
        g if g > 18 => println!("You are Eligible ! ({})", g),
        e => println!("Please wait 1 year now you are ({}) years old", e),
        l if l < 18 => println!("You are not Eligible! Please wait for ({})", 18 - l),
        _ => println!("Invalid Day"),
    }
}

//String match with if else (switch case)
fn main_seven(operator: &str) {
    let result = if operator == "admin" { true } else { false };
    println!("Result is ! ({})", result);
}
