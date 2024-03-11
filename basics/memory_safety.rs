fn main() {
  case_1();
  case_2();
  case_3();
}

fn case_1() {
  let a: i32; // error[E0381]: used binding `a` isn't initialized
  println!("a = {a}");
}

fn case_2() {
  let a: i32; // error[E0381]: used binding `a` is possibly-uninitialized
  if true {
    a = 1;
  }
  println!("a = {a}");
}

fn case_3() {
  let a: i32;
  if true {
    a = 1;
  } else {
    a = 2;
  }
  println!("a = {a}");
}
