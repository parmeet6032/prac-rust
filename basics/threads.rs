use std::thread;

fn main() {
  let handle = thread::spawn(move || {
    // some stuff in child thread
  });

  // do stuff simultaneously in main thread

  // wait until thread has exited
  handle.join().unwrap();
}
