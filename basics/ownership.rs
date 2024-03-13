fn main() {
  /**
   * 1. Each value has an owner
   * 2. Only one owner
   * 3. Value gets dropped if it's owner goes out of scope
   *
   * In Rust, same variables in Stack memory cannot point to same data in Heap memory.
   * .clone() also creates clone of data in Heap memory.
   *
   * Functions parameters also take away the ownership of the variable
   * Use reference or borrowing if want to send copy
   */
  let s1 = String::from("abc");
  println!("{s1}");
  let s2 = s1;
  // let s2 = s1.clone();
  // println!("{s1}"); // error[E0382]: borrow of moved value: `s1`
  println!("{s2}");
}
