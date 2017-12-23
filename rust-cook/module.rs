mod sample {
  fn private_function() {
    println!("called `sample::private_function()`\n");
  }

  pub fn sample_function() {
    println!("called `sample::sample_function`");
  }
  pub fn indireact_function() {
    private_function()

  }
}

fn sample_function() {
  println!("outside sample function called");
}

mod nested {
  pub mod nested_mod {
    pub fn function() {
      println!("called `nested::nested_mod::function()`");
    }
    #[allow(dead_code)]
    fn private_function() {
      println!("called `nested::nested_mod::private_function()`");
    }
  }

  mod private_nest_mod {
    #[allow(dead_code)]
    pub fn function() {
      println!("called
      `nested::private_nest_mod::function()`");
    }
  }
}

mod sample_struct {
  pub struct WhiteBox<T> {
    pub information: T
  }

  pub struct BlackBox<T> {
    information: T
  }


  impl<T> BlackBox<T> {
    pub fn const_new(information: T) -> BlackBox<T> {
      BlackBox {
        information: information
      }
    }
  }
}

pub mod root {
  use self::foo::create_foo;
  mod foo {
    pub struct Foo {
      i: i32
    }
    impl Foo {
      pub fn hello_foo(&self) {
        println!("Hello foo");
      }
    }
    pub fn create_foo(i: i32) -> Foo {
      Foo {i: i}
    }
  }
  pub mod bar {
    pub struct Bar {
      pub f: ::root::foo::Foo
    }
    impl Bar {
      pub fn new(i: i32) -> Self {
        Bar {f: ::root::foo::create_foo(i)}
      }
    }
  }
}


use deeply::nested::sample_function as other_function;
mod deeply {
  pub mod nested {
    pub fn sample_function() {
      println!("called `deeply::nested::function()`");
    }
  }
}

mod cool {
  pub fn sample_function() {
    println!("called `cool::sample_function()`");
  }
}

mod sample_mod {
  fn sample_function() {
    println!("called `sample_mod::sample_function()`");
  }
  pub fn indireact_function() {
    println!("indirect called");
    sample_function();
    self::sample_function();
    super::sample_function();
  }
}

fn main() {
  // sample::indireact_function();
  // sample::sample_function();
  // sample_function();
  // nested::nested_mod::function()

  // struct
  // let white_box = sample_struct::WhiteBox {
  //   information: "public information n"
  // };
  // println!("the white box contains {}", white_box.information);


  // let black_box = sample_struct::BlackBox {
  //   information: "black_box"
  // };
  // let _black_box = sample_struct::BlackBox::const_new("hello, world");

  // let f = root::foo::create_foo(43);
  let b = root::bar::Bar::new(32);
  b.f.hello_foo();
  other_function();

  {
    use deeply::nested::sample_function;
    sample_function();
  }
  sample_function();
  {
    use cool::sample_function as cool_function;
    cool_function();
  }

  sample_mod::indireact_function()

}
