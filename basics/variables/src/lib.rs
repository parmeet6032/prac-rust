use rand::{thread_rng, Rng};

pub fn greet() {
  let mut rng = thread_rng();
  let x: u32 = rng.gen_range(0..100);
  println!("Hello! {x}");
}
