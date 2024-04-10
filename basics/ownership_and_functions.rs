fn main() {
  let mut x = 5;
  let mut y = String::from("hello");

  // Passing by value
  // takes_ownership2(x); // doesn't give error as i32 implements Copy trait
  takes_ownership(x, y);

  // Error: x and y are no longer accessible here
  // println!("x: {}, y: {}", x, y);

  let z = 10;
  let w = String::from("world");

  // Passing by reference
  borrows_variables(&z, &w);

  // Variables z and w are still accessible here
  println!("z: {}, w: {}", z, w);
}

fn takes_ownership(mut a: i32, mut b: String) {
  a += 1;
  b.push_str(" world");
  println!("Inside function: a: {}, b: {}", a, b);
}

fn takes_ownership2(mut a: i32) {
  a += 1;
  println!("Inside function: a: {}", a);
}

fn borrows_variables(c: &i32, d: &String) {
  // Cannot modify c or d here
  // *c += 1; // Error: Cannot assign to immutable borrowed content
  // d.push_str(" world"); // Error: Cannot borrow `*d` as mutable, as it is not declared as mutable

  println!("Inside function: c: {}, d: {}", c, d);
}
