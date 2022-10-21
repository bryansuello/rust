use std::io;

fn main(){
    println!("What is your name?");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("your name is {}", input);
    if input == "Hannah" {
        println!("hello, Bryan's wifey");
    }
    else{
        println!("hello, stranger");
    }
}
