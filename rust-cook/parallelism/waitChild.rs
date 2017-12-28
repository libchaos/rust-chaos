use std::process::Command;

fn main() {
  let mut child = Command::new("sleep")
    .arg("5")
    .spawn()
    .unwrap();
  let _result = child.wait().unwrap();

  print!("Status if child process {}\n", _result);
  println!("reached end of the main function");
}