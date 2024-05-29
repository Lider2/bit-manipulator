use std::io::{self, Read, Write};

struct BitManipulator {}

impl BitManipulator {
  fn set_bit(num: u32, index: u32) -> u32 {
    return num | (1_u32 << index);
  }

  fn unset_bit(num: u32, index: u32) -> u32 {
    return num & (!(1_u32 << index));
  }

  fn toggle_bit(num: u32, index: u32) -> u32 {
    return num ^ (1_u32 << index);
  }
}

fn pause() {
  let mut stdin = io::stdin();
  let mut stdout = io::stdout();

  write!(stdout, "Press any key to continue...").unwrap();
  stdout.flush().unwrap();

  stdin.read(&mut [0u8]).unwrap();
}

fn read_u32_from_console() -> u32 {
  let stdin = io::stdin();
  let mut number_string = String::new();

  stdin
    .read_line(&mut number_string)
    .expect("Ошибка ввода числа");

  let number = number_string
    .trim()
    .parse::<u32>()
    .expect("Ошибка парсинга числа");

  return number;
}

fn main() {
  // set_bit
  println!("Введите число для set_bit");
  let number = read_u32_from_console();

  println!("Введите индекс бита");
  let index = read_u32_from_console();

  let result = BitManipulator::set_bit(number, index);

  println!();
  println!("> {:#034b} ({})", number, number);
  println!("> {:#034b} ({})", result, result);
  println!("set bit at {index} index of {number}: {result}");
  println!();

  // set_bit
  println!("Введите число для unset_bit");
  let number = read_u32_from_console();

  println!("Введите индекс бита");
  let index = read_u32_from_console();

  let result = BitManipulator::unset_bit(number, index);

  println!();
  println!("> {:#034b} ({})", number, number);
  println!("> {:#034b} ({})", result, result);
  println!("unset bit at {index} index of {number}: {result}");
  println!();

  // set_bit
  println!("Введите число для toggle_bit");
  let number = read_u32_from_console();

  println!("Введите индекс бита");
  let index = read_u32_from_console();

  let result = BitManipulator::toggle_bit(number, index);

  println!();
  println!("input  > {:#034b} ({})", number, number);
  println!("result > {:#034b} ({})", result, result);
  println!("unset bit at {index} index of {number}: {result}");
  println!();

  pause();
}
