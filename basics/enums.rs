#![allow(unused_mut, unused_variables, dead_code)]

use std::fs::File;
use DispenserItem::*;

fn main() {
  let item = Place { x: 24, y: 36 };

  let apple = Some(5);
  if let Some(x) = apple {
    println!("value is {x}");
  }

  // either all branch arms return something of same type OR return nothing
  match apple {
    Some(x) => {
      println!("value is {x}");
    }
    None => {
      println!("no value");
    }
    _ => {
      println!("Default case");
    }
  }

  let mango = match apple {
    Some(x) => 1,
    None => 2,
    _ => 3,
  };
  println!("{mango}");

  // option
  // let mut x: Option<i32> = None;
  let mut x = None;
  x = Some(5);
  println!("{:?}", x);

  let (some, none) = (x.is_some(), x.is_none());
  println!("{} {}", some, none);

  for i in x {
    println!("{}", i);
  }

  // result
  // File::open("foo"); // warning: unused `Result` that must be used
  let res = File::open("foo");
  // let f = res.unwrap();  // crash
  // let f = res.expect("custom error message for details"); // crash with custom error

  if res.is_ok() {
    let f = res.unwrap();
  }
  
  match res {
    Ok(f) => {}
    Err(e) => {}
  }
}

enum Color {
  Red,
  Green,
  Blue,
}

enum DispenserItem {
  Empty,
  Ammo(u8),
  Things(String, i32),
  Place { x: i32, y: i32 },
}

impl DispenserItem {
  fn display(&self) {}
}

enum Option<T> {
  Some(T),
  None,
}
