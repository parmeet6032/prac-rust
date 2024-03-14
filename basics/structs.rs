#![allow(unused_mut, unused_variables, dead_code)]

fn main() {
  let fox = RedFox {
    enemy: true,
    life: 70,
  };

  let fox2 = RedFox::new();
  // let life_left = fox2.life;
  // fox2.enemy = false; // error[E0594]: cannot assign to `fox2.enemy`, as `fox2` is not declared as mutable
  // fox2.some_method();

  print_noise(5_u8);

  let robot = Robot {};
  robot.run();
}

struct RedFox {
  enemy: bool,
  life: u8,
}

trait Noisy {
  fn get_noise(&self) -> &str;
}

impl Noisy for RedFox {
  fn get_noise(&self) -> &str {
    "Meow?"
  }
}

impl RedFox {
  // associated functions
  // Self == RedFox
  fn new() -> Self {
    Self {
      enemy: true,
      life: 70,
    }
  }
  // fn function() ...

  // methods
  // fn move(self) ...
  // fn borrow(&self) ...
  // fn mut_borrow(&mut self) ...
}

fn print_noise<T: Noisy>(item: T) {
  println!("{}", item.get_noise());
}

impl Noisy for u8 {
  fn get_noise(&self) -> &str {
    "BYTE"
  }
}

trait Run {
  fn run(&self) {
    println!("I'm running");
  }
}

struct Robot {}
impl Run for Robot {}
