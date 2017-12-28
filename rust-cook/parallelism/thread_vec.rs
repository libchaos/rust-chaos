use std::thread;

static NO_THREADS: i32 = 10;


fn main() {
  let mut thread_holder = vec![];
  for i in 0..NO_THREADS {
    thread_holder.push(thread::spawn(move || {
      println!("thread number is {}", i);
      i
    }));
  }
  println!("**************");
  for thread_elem in thread_holder {
    println!("{:?}", thread_elem.join().unwrap());
  }
}