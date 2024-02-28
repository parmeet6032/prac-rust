fn main() {
  let a = 1;
  let b: i32 = 2;
  let (c, d) = (3, 4);

  /*
   => Everything is immutable (safety, concurrency and speed)
   let bunnies = 16;
   bunnies = 17; // error
  */
  let mut bunnies = 16;
  bunnies = 17;

  const GRAVITY_CONSTANT: f64 = 9.8;
  // const GRAVITY_CONSTANT = 9.8;
  // const abc: f64 = 9.8;
  // GRAVITY_CONSTANT = 9.1;
}
