use std::io;

fn main() {

    println!("Enter an integer:");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input: i32 = input.trim().parse().expect("Please enter a valid integer");

    for i in 0..=input {
        //println!("{}",i)
    }

    let mut j = 0;
    while j <= input {
        println!("{}",j);
        j +=1;
    }

}

 