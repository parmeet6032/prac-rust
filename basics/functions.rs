fn main() {
  let result = multiply(2.0, 12.5);
  println!("{result}");
}

fn multiply(qty: f64, oz: f64) -> f64 {
  print!("{qty} * {oz} = ");
  qty * oz
}
