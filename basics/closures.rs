fn main() {
  let add = |x, y| x + y;
  println!("{}", add(1, 2));

  let s = "Hello".to_string();
  // closure borrows reference to the variable s
  let f = || {
    println!("{s}");
  };

  // 'move' semantice moves the ownership of variable it uses to closure
  // this closure can now be sent across various threads
  // let f = move || {
  //   println!("{s}");
  // };

  f();

  // example
  let v = vec![2, 4, 6];
  let sum = v
    .iter()
    .map(|x| x * 3)
    .filter(|x| *x > 10)
    .fold(0, |acc, x| acc + x);

  println!("{sum}");
}
