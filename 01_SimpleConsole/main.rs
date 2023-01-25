// This example shows how to create a simple console application.

// Import the io library
use std::io;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;


fn check_message_quit(passed_message: String) -> bool {
  let mut ret_val = false;
  let cleaned_message = passed_message.replace("\r", "").replace("\n", "");

  // println!("[DEBUG] Checking message: '{}'", cleaned_message);

  if cleaned_message == "quit" || cleaned_message == "exit" {
    // println!("[DEBUG] Quit message received (quit or exit)");
    ret_val = true;
  }

  return ret_val;
}

fn main() {
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    loop {
      let mut input = String::new();
      io::stdin().read_line(&mut input).unwrap();
      tx.send(input).unwrap();
    }
  });

  loop {
    if let Ok(msg) = rx.try_recv() {

      if check_message_quit(msg.clone()) {
        println!("Exiting...");
        break;
      }

      println!("You typed: {}", msg.clone());
      
    }

    thread::sleep(Duration::from_millis(100));
  }

}
