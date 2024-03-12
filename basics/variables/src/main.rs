const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

use variables::greet;
// use std::collections::HashMap;

fn main() {
  let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);

  println!("Firing {} of my {} missiles...", ready, missiles);
  missiles = missiles - ready;
  println!("{} missiles left", missiles);

  // STARTING_MISSILES = 10; // error[E0070]: invalid left-hand side of assignment

  greet();
}
