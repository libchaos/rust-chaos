use std::thread;

fn main() {
  thread::spawn(move || {
    println!("hello from spawn thread");
  });

  let join_handle = thread::spawn(move || {
    println!("hello from second spawned thread");
    17
  });

  println!("hello from main thread");

  match join_handle.join() {
    Ok(x) => println!("second spawned thread returned {}", x),
    Err(_) => println!("sencond spawned thread panicked")
  }
}