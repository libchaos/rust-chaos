use std:: {i32, f32};
fn main() {
  let x_val = 5u32;
  let y_val = {
    let x_squared = x_val * x_val;
    let x_cube = x_squared * x_val;

    x_cube + x_squared + x_val
  };
  let z_val = {
    2 * x_val
  };
  println!("{}", x_val);
  println!("{}", y_val);
  println!("{}", z_val);

  const UPPERLIMIT: i32 = 12;
  fn is_big(n: i32) -> bool {
    n > UPPERLIMIT
  }

  let random_number = 15;
  println!("{}", is_big(random_number));
  println!("{} is {}", random_number, if is_big(random_number) {"big"} else {"small"} );

  // binding
  let a = 5;
  let (b, c) = (1, 2);
  let x_val: i32 = 5;
  let y_val: i32 = 8;

  {
    println!("scopede {}", y_val);
    let y_val = 12;
    println!("scoped {}", y_val);
  }
  println!("outer {}", y_val);
  let y_val = 42;
  println!("{}", y_val);

  let (true_pos, true_neg, false_pos, false_neg) = (100, 50, 10, 5);
  let total = true_neg + true_pos + false_neg + false_pos;

  println!("total {}", total);

  println!("{:.2}", percentage(accuracy(true_pos, true_neg, total)));

  fn accuracy(tp: i32, tn: i32, total: i32) -> f32 {
    (tp as f32 + tn as f32) / (total as f32)
  }
  fn percentage(value: f32) -> f32 {
    value * 100.0
  }

  // conditonal

  let age: i32 = 10;
  if age <= 19 {
    println!("go to school");
  } else  {
    println!("do someting with your life");
  }
  let can_vote = if age >= 18 {true} else {false};
  println!("can vote {}", can_vote);

  let mut x = 1;
  println!("loop event coming");

  loop {
    if x % 2 == 0 {
      x += 1;
      println!("{}", x);
      continue;
    }
    if x > 10 {
      break;
    }
    x += 1;
  }

  let mut y = 1;

  println!("while 1 to 9");
  while y < 10 {
    println!("{}", y);
    y += 1;
  }
  let mut z = 1;
  println!("for z 1 to 9");
  for z in 1..10 {
    println!("{}", z);
  }

  let hulk = Hero::Strong(100);
  let fasty = Hero::Fast;
  let spiderman = Hero::Info{
    name: "spiderman".to_owned(),
    secret: "peter parker".to_owned(),
  };
  get_info(spiderman);
  get_info(hulk);
  get_info(fasty);

  enum Hero {
    Fast,
    Strong(i32),
    Info {name: String, secret: String}
  }


  fn get_info(h: Hero) {
    match h {
        Hero::Fast => println!("FAST"),
        Hero::Strong(i) => println!("lifts {} tons", i),
        Hero::Info {name, secret} => {println!("name is {} secret is {}", name, secret);}
    }
  }
  struct Pointer {
    x: i32,
    y: i32
  };

  let y = Pointer {x: 1, y: 3};
  println!("{}, {}", y.x, y.y);

  // let s = String::from("sfsf");
  // {
  //   let sy = s;
  //   println!("{}", sy);

  //   // println!("{}", );
  // }
  // // println!("{}", s);
  // {
  //   let mut s = String::from("hello, world");
  //   // let sy = s;
  //   fn copy(s: String) -> String {
  //     s
  //   }
  //   s = copy(s);
  //   println!("{}", s);
  // }

  // define closures eql lambda

  let sum_num = |x: i32, y: i32| x + y;
  println!("7 + 8 = {}", sum_num(7, 8));

  let num_ten = 10;
  let add_ten = |x: i32| x + num_ten;
  println!("3 + 10 = {}", add_ten(3));

  // pointer

  let vec1 = vec![1, 2, 3];
  let vec2 = vec1;
  // println!("{:?}", vec1); error has been moved;

  let prim_val = 1;
  let prim_val2 = prim_val;
  println!("primitive value: - {}", prim_val); // copy

  println!("sum os vects: {} ", sum_vecs(&vec2));

  fn sum_vecs(v1: &Vec<i32>)->i32{
    let sum = v1.iter().fold(0, |mut sum, &x| {sum += x; sum});
    sum
  }

  println!("{:?}", vec2.iter().next());


  let mut circle = Circle {
    x: 10.0,
    radius: 10.0
  };
  println!("x: {:.3}, radius: {:.3}", circle.x, circle.radius);
  println!("{}", get_radius(&circle));

  println!("get_x: {}", circle.get_x());

  struct Circle {
    x: f64,
    radius: f64
  }

  fn get_radius(circle: &Circle) -> f64 {
    circle.radius
  }
  impl Circle {
    pub fn get_x(&self) -> f64 {
      self.x
    }
  }

  let mut circle = Circle1 {
    r: 10f64,
  };

  println!("{}", circle.area());

  let mut rect = Rectangle {
    h: 10.0,
    b: 10.0
  };
  println!("{}", rect.area());

  struct Rectangle {
    h: f64,
    b: f64
  }

  struct Circle1 {
      r: f64
  }

  trait HasArea {
    fn area(&self) -> f64;
  }

  impl HasArea for Circle1 {
    fn area(&self) -> f64 {
      3.14 * (self.r * self.r)
    }
  }

  impl HasArea for Rectangle {
      // add code here
    fn area(&self) ->f64 {
      self.h * self.b
    }
  }
}

