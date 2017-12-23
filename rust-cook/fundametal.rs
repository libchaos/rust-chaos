use std::{i8};


fn main() {
  println!("understanding the assignment");

  let num:i8 = 10;
  println!("Num is {}", num);
  println!("{}", i8::MAX);
  println!("{}", i8::MIN);

  let (fname, lname) = ("viki", "d");
  println!("{}, {}", fname, lname);

  let bool_val: bool = true;
  let x_char: char = 'a';

  println!("{}, {}", bool_val, x_char);

  println!("{:.2}", 1.2345);

  println!("B: {:b} H: {:x} O: {:o}", 10, 10, 10);

  // shifts
  println!("{ten:>ws$}", ten=10, ws=5);
  println!("{ten:>0ws$}", ten=10, ws=5);

  println!("5 + 4 = {}", 5 + 4);
  println!("5 - 4 = {}", 5 - 4);
  println!("5 * 4 = {}", 5 * 4);
  println!("5 / 4 = {}", 5 / 4);
  println!("5 % 4 = {}", 5 % 4);

  // assigning data types and mathematical operations
  let neg_4 = -4i32;
  println!("abs(-4) = {}", neg_4.abs());
  println!("pow(2)_(-4) = {}", neg_4.pow(2));
  println!("round(1.234) = {}", 1.234f64.round());
  println!("ceil(1.234) = {}", 1.234f64.ceil());
  println!("sin(3.14) = {}", 3.14f64.sin());

  // string
  let rand_string: &str = "i love Rust <|";
  println!("{}", rand_string);
  println!("{}", rand_string.len());
  let (first, second) = rand_string.split_at(7);
  println!("{}, {}", first, second);

  let count = rand_string.chars().count();
  println!("count {}", count);

  println!("_______________");
  let mut chars = rand_string.chars();
  let mut indiv_chars = chars.next();
  loop {
    match indiv_chars {
      Some(x) => println!("{}", x),
      None => break
    }
    indiv_chars = chars.next();
  }

  println!("-----------------");

  let mut iter = rand_string.split_whitespace();
  let mut indiv_word = iter.next();
  loop {
    match indiv_word {
      Some(x) => println!("{}", x),
      None => break
    }
    indiv_word = iter.next();
  }

  println!("----------");
  let rand_string2 = "I LOLVE \n EVERYTHING ABOUD RUST \n HAHA~~~";
  let mut iter_line = rand_string2.lines();
  let mut indiv_sent = iter_line.next();
  loop {
    match indiv_sent {
      Some(x) => println!("{}", x),
      None => break
    }
    indiv_sent = iter_line.next();
  }


  let rand_array = [1, 2, 3];
  println!("{:?}", rand_array);
  println!("{}", rand_array[0]);

  println!("{}", rand_array.len());
  println!("{:?}", &rand_array[1..3]);
  println!("{:?}", &rand_array);
  // println!("{}", rando_array.type_name());
  let rand_array = ['a', 'b', 'c'];

  // vector what ' s the difference  bettween vector and array
  // common in these two types  malloc on heap
  // vector can descrse or increate in runtime, but array can not

  let mut vec1 = vec![1, 2, 3, 4, 5];
  println!("{}", vec1[2]);

  for i in &vec1 {
    println!("{}", i);
  }
  vec1.push(6);
  println!("{:?}", vec1);
  vec1.pop();
  println!("{:?}", vec1);

  let rand_tuple = ("Rust", 2017);
  let rand_tuple2: (&str, i8) = ("viki", 4);
  println!("Name : {}", rand_tuple2.0);
  println!("Lucky no: {}", rand_tuple2.1);
}