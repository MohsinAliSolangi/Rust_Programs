fn main() {
    main_two();
    main_three();
    main_four();
    main_five();
    main_six()
    main_seven();
    main_eight();
    main_nine();
}

//for loop (infinite)
//Exit control loop
fn main_two() {
    let num = 2;
    let mut i = 1;
    loop {
        if i == 10 {
            break;
        }
        println!("{num}X{i} => {}", num * i);
        i = i + 1;
    }
}

//Do while
//Entry control loop
fn main_three() {
    let mut i = 1;
    let num = 2;

    while i <= 10 {
        println!("{num} X {i} => {}", num * i);
        i = i + 1;
    }
}

//Exclusive range Operator
fn main_four() {
    for i in 1..10 {
        // This run for only 9 times
        println!("{} Mohsin ", i);
    }
}

//In Exclusive range Operator
fn main_five() {
    for i in 1..=10 {
        // This run for 10 times
        println!("{} Mohsin ", i);
    }
}

//In Exclusive range Operator In Condition based
fn main_six() {
    for i in 1..=10 {
        if i == 5 {
            //This skip the print 5
            continue;
        }
        println!("{} Mohsin ", i);
    }
}

//Get index in for loop
fn main_seven() {
    for (index, i) in (1..=10).enumerate() {
        if i == 5 {
            //This skip the print 5
            continue;
        }
        println!("index {index} => Mohsin {}", i);
    }
}

//Get elements in loop by using array
fn main_eight() {
    let arr = ["Mohsin", "Ali", "Solangi", "siraj", "Kami"];

    let mut i = 0;
    for item in arr {
        println! {"for => {item}"};
    }

    while i < arr.len() {
        println!("while => {}", arr[i]);
        i = i + 1;
    }

    let mut i = 0;
    loop {
        if i == arr.len() {
            break;
        }
        println!("loop => {}", arr[i]);
        i = i + 1;
    }
}

//Get index in loop by using array
fn main_nine() {
    let arr = ["Mohsin", "Ali", "Solangi", "siraj", "Kami"];

    let mut i = 0;
    for (index, item) in arr.iter().enumerate() {
        println! {"for index {index} => {item}"};
    }

    while i < arr.len() {
        println!("while => {}", arr[i]);
        i = i + 1;
    }

    let mut i = 0;
    loop {
        if i == arr.len() {
            break;
        }
        println!("loop => {}", arr[i]);
        i = i + 1;
    }
}
