#![allow(unused)]

//defining module
mod icetea;
mod iamnew;


use std::io;
use rand::Rng;
use std::io::{Write,BufRead,ErrorKind};

fn guess_the_number(){
    println!("Guess the number");
    println!("Enter your input");
    let mut guess=String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the input");
    println!("Your guess:{}",guess.trim_end());
}


fn main(){
    let x=5; //by default immutable
    let mut y=88; //mutable variable

    //printing-
    print!("Hi my name is {} + {}\n",x,y);
    println!("JI");//next line

    //creating string at runtime heap memory
    let mut a= String::new();

    //taking input from user come in form of string
    io::stdin().read_line(& mut a).expect("not received input");

    //converting string to i32
    let c:i32=a.trim().parse().expect("msg");    
    
    //loop type:1-
    let mut i=1;
    loop{
        if i>=5 {
            break;
        }
        println!("nah bro");
        i+=1;
    }

    //for loop type-2:
    for i in 0..5{
        print!("{}",i);//string concatination
    };
    
    //match-
    match c{
        1 ..=15=>println!("Hi 1"),
        21 ..= 55=>println!("Hi 2"),
        56 ..=i32::MAX =>println!("Hi 3"),
        _ =>print!("Hi 4")
    };


    //random number generation-
    let rand_num=rand::thread_rng().gen_range(1..101);
    println!("{}",rand_num);

    //fn calling-
    guess_the_number();

    //ownership and borrowing
    let mut s = String::from("hello");
    let r1 = &s; // r1 refers to s
    let r2 = &s; // r2 also refers to s
    // This is not allowed:
    // r1 = &mut s; // Trying to change r1 to refer to a mutable borrow of s
    println!("{} and {}", r1, r2);



}
