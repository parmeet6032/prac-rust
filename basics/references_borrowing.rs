fn main() {
  // let s1 = String::from("abc");
  let mut s1 = String::from("abc");
  // do_stuff(&s1);
  do_stuff(&mut s1);
  println!("{s1}"); // error[E0382]: borrow of moved value: `s1`
}
/**
 * variable x
 * immutable reference &x
 * mutable reference &mut x
 * x: &mut i32
 * *x // a mutable i32
 *
 * x: &i32
 * *x // an immutable i32
 *
 * Either:
 * 1 mutable reference
 * any number of immutable reference
 */
fn do_stuff(s: &mut String) {
  // error[E0596]: cannot borrow `*_s` as mutable, as it is behind a `&` reference
  // some stuff
  s.insert_str(0, "@ ");
  (*s).insert_str(0, "Hello ");

  *s = String::from("Replacement");
}
