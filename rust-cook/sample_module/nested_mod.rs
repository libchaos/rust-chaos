pub fn sample_function() {
  println!("called sample::nested_mod::sample_function");
}

#[allow(deadcode)]
fn private_function() {
  println!("called sample_module::nested_mod::private_function()");
}

#[allow(deadcode)]
pub fn public_function() {
  println!("called `sample_module::sample_private::public_function()`");
}