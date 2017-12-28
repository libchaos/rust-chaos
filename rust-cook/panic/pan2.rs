type Result<T> = std::result::Result<T, String>;


fn double_first(vec: Vec<&str>) -> Result<i32> {
  let first = try!(vec.first()
    .ok_or("Please use a vector at least one element".to_owned()));
  let value = try!(first.parse::<i32>()
    .map_err(|e| e.to_string()));
  Ok(2 * value)
}


fn print(result: Result<i32>) {
  match result {
    Ok(n) => println!("The first double is {}", n),
    Err(e) => println!("Error: {}", e)
  }
}

fn main() {
  let empty = vec![];
  let strings = vec!["1", "tofu", "23"];
  print(double_first(empty));
  print(double_first(strings));
}