#![allow(unused)]

use std::io;
// use rand::Rng;
use std::io::{Write,BufRead,ErrorKind};

use rand::Rng;

fn main(){
    let rand_num=rand::thread_rng().gen_range(1..101);
    println!("{}",rand_num);
}
