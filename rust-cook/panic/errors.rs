use std::num::ParseIntError;
use std::fmt;


type Result<T> = std::result::Result<T, CustomError>;



#[derive(Debug)]
enum CustomError {
    EmptyVec,
    Parse(ParseIntError),
}


impl fmt::Display for CustomError {
    // add code here
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match *self {
          CustomError::EmptyVec => write!(f, "please use a vector with at least one element"),
          CustomError::Parse(ref e) => e.fmt(f),
      }
    }
}

fn double_val(vec: Vec<&str>) -> Result<i32> {
  vec.first()
    .ok_or(CustomError::EmptyVec)
    .and_then(|s| s.parse::<i32>().map_err(CustomError::Parse).map(|i| 2 * i))
}


fn print(result: Result<i32>) {
  match result {
      Ok(n) => println!("the first double is {}", n),
      Err(e) => println!("Error: {}", e),
  }
}

fn main() {
  let numbers = vec!["93", "18"];
  let empty = vec![];
  let strings = vec!["tofu", "9023"];

  print(double_val(numbers));
  print(double_val(empty));
  print(double_val(strings));
}