use std::io;
use std::{i32};

fn main() {
  println!("Enter the first number ? ");
  let mut input1 = String::new();
  io::stdin().read_line(&mut input1).expect("Failed to read line");

  println!("Enter the second number? ");
  let mut input2 = String::new();
  io::stdin().read_line(&mut input2).expect("Failed to read line");

  let aint:i32 = input1.trim().parse().ok().expect("Program only process number");
  let bint:i32 = input2.trim().parse().ok().expect("Program only process numbers");

  println!("sum is {}", aint + bint);

}