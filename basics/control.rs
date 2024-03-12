fn main() {
  let mut num = 5;
  if num == 5 {
    println!("{num}");
  }

  let mut msg;
  if num == 5 {
    msg = "five";
  } else if num == 4 {
    msg = "four";
  } else {
    msg = "other";
  }
  println!("{msg}");

  num = 4;
  // all tails return same type
  msg = if num == 5 {
    "five"
  } else if num == 4 {
    "four"
  } else {
    "other"
  };
  println!("{msg}");

  let (a, b, c, x, y, z) = (true, 1, 2, false, 4, 5);

  num = if a { b } else { c };
  println!("{num}");

  num = if a {
    if x {
      y
    } else {
      z
    }
  } else {
    c
  };
  println!("{num}");

  // *** loops
  // unconditional
  'bob: loop {
    loop {
      loop {
        break 'bob;
      }
    }
  }

  // 'bob: loop {
  //   loop {
  //     continue 'bob;
  //   }
  // }

  // conditional
  let mut count = 5;
  while count > 0 {
    println!("hello");
    count -= 1;
  }

  // do-while
  loop {
    // do stuff
    // check condition
    break;
  }

  for num in [1, 2, 3].iter() {
    println!("{num}");
  }

  let arr = [(1, 2), (3, 4)];
  for (x, y) in arr.iter() {
    println!("{x}, {y}");
  }

  for num in 0..5 {
    // 0 inclusive 5 exclusive
    print!("{num} ");
  }
  println!();

  for num in 0..=5 {
    // both inclusive
    print!("{num} ");
  }
  println!();
}
