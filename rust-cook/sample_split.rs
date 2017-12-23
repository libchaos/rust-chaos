//-- #########################
//-- Task: To create a sample file structure
//-- Author: libchaos
//-- Version: 1.0.0
//-- #########################

mod sample_module;

fn sample_function() {
  println!("called `sample_function`");
}

fn main() {
  sample_module::sample_function();
  sample_function();
  sample_module::indirect_access();
  sample_module::nested_mod::sample_function();
}