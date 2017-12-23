## how to do it

```
创建文件
touch sample_lib.rs
编辑文件
define headers::
//-- #########################
//-- Task: To create a sample library in rust
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 4 March 17
//-- #########################

pub fn public_function() {
  println!("called sample_lib `public_function()`");
}
fn private_function() {
  println!("called sample_lib `private_function()`");
}
fn indrect_access() {
  private_function()
}
编译项目
rustc --crate-type=lib sample_lib.rs

调用
rustc test.rs --extern nested_mod=libnested_mod.rlib
```