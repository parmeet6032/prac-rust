use std::collections::HashMap;

fn main() {
  // vector
  let mut v: Vec<i32> = Vec::new();
  v.push(2);
  v.push(4);
  v.push(6);

  let x = v.pop();
  println!("{}", x.unwrap());
  println!("{:?}", v);
  println!("{}", v[v.len() - 1]);

  let v2 = vec![2, 4, 6];
  println!("{:?}", v2);

  // hashmap
  let mut h: HashMap<u8, bool> = HashMap::new();
  h.insert(5, true);
  h.insert(6, false);
  h.insert(7, true);
  h.insert(8, false);
  h.insert(8, false);
  let five = h.remove(&5).unwrap();
  println!("{:?}", h);
  println!("{five}");
}
