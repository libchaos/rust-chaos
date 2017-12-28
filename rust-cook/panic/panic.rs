use std::num::ParseIntError;


#[warn(dead_code)]
fn compare_stmt(stmt: &str) {
  if stmt == "Another book" {
    panic!("Rust Cookbook is not selected!");
  }
  println!("Statements is {}", stmt);
}

fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
  match number_str.parse::<i32>() {
    Ok(n) => Ok(2 * n),
    Err(e) => Err(e),
  }
}
type AliasedResult<T> = Result<T, ParseIntError>;

fn double_number_map(number_str :&str) -> AliasedResult<i32> {
  number_str.parse::<i32>().map(|n| 2 * n)
}

fn print(result: AliasedResult<i32>) {
  match  result {
      Ok(n) => println!("n is {}", n),
      Err(e) => println!("Error: {}", e),
  }
}

type Result<T> = std::result::Result<T, String>;

fn double_first(vec: Vec<&str>) -> Result<i32> {
  vec.first()
    .ok_or("Please use vector at least one".to_owned())
    .and_then(|s| s.parse<i32>().map_err(|e| e.to_string()))
    .map(|i| 2 * i)
}

fn main() {
  // compare_stmt("Rust book");
  // compare_stmt("Another book");

  // let twenty = double_number("10");
  // print(twenty);
  // print(double_number("t"));
  // print(double_number_map("12t"))
  double_number(vec!["1", "2"])
}