mod sample_private;
pub mod nested_mod;


pub fn sample_function() {
  println!("called `sample_module::sample_function`");
}

fn private_function() {
  println!("called `sample_module::private_function()`");
}

pub fn indirect_access() {
  println!("called `sample_module::indirect_function()`");
  private_function();
}