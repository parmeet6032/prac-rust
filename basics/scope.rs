fn main() {
  // scope_test();
  shadow_variable();
}

fn scope_test() {
  let x = 5;
  {
    let y = 99;
    println!("{}, {}", x, y); // prints 5, 99
  }
  // println!("{}, {}", x, y); // error[E0425]: cannot find value `y` in this scope
}

fn shadow_variable() {
  let x = 5;
  {
    let x = 99;
    println!("{}", x); // prints 99
  }
  println!("{}", x); // prints 5

  // *** shadow from mutable to immutable
  let mut y = 9;
  y = 10;
  println!("{}", y); // prints 10

  let y = y;
  // y = 11; // error[E0384]: cannot assign twice to immutable variable `y`
  println!("{}", y); // prints 10

  // *** shadow to a different type
  let meme = "abc";
  println!("{meme}"); // prints abc
  let meme = return_one();
  println!("{meme}"); // prints 1
}

fn return_one() -> i32{
  return 1;
}