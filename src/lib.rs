use std::io::{prelude::*, stdin, stdout};

pub fn option_input(min: u8, max: u8) -> u8 {
  loop {
    let mut inp = String::new();
    stdin().read_line(&mut inp).expect("Failed to read line");
    let op: u8 = match inp.trim().parse() {
      Ok(n) => n,
      Err(_) => {
        println!("Invalid option. Try again.");
        print!(">>");
        stdout().flush().ok().expect("Could not flush stdout");
        continue;
      }
    };
    if op < min || op > max {
      println!("Unsupported option. Try again.");
      print!(">>");
      stdout().flush().ok().expect("Could not flush stdout");
      continue;
    }
    return op;
  }
}

pub fn number_input() -> f32 {
  loop {
    let mut inp = String::new();
    stdin().read_line(&mut inp).expect("Failed to read line");
    let num: f32 = match inp.trim().parse() {
      Ok(n) => n,
      Err(_) => {
        println!("Invalid number. Try again.");
        print!(">>");
        stdout().flush().ok().expect("Could not flush stdout");
        continue;
      }
    };
    return num;
  }
}

pub fn add(n1: f32, n2: f32) -> f32 {
  n1 + n2
}

pub fn multiply(n1: f32, n2: f32) -> f32 {
  n1 * n2
}

pub fn divide(n1: f32, n2: f32) -> f32 {
  n1 / n2
}

pub fn subtract(n1: f32, n2: f32) -> f32 {
  n1 - n2
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_add() {
    let n1 = 3.45;
    let n2 = 9.;
    assert_eq!(add(n1, n2), 12.45);
  }

  #[test]
  fn should_subtract() {
    let n1 = 13.;
    let n2 = 45.;
    assert_eq!(subtract(n1, n2), -32.);
  }

  #[test]
  fn should_multiply() {
    let n1 = 0.3;
    let n2 = 8.;
    assert_eq!(multiply(n1, n2), 2.4);
  }

  #[test]
  fn should_divide() {
    let n1 = 90.;
    let n2 = 3.;
    assert_eq!(divide(n1, n2), 30.);
  }
}
