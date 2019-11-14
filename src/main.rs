use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let out = b"Hello fellow Rustaceans!";
    let width = 40;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
    println!("solve(5) = {}", solve(5));
    say(format!("solve(6) = {}", solve(6)).as_bytes(), width, &mut writer).unwrap();
    say(format!("solve2(7) = {}", solve2(7)).as_bytes(), width, &mut writer).unwrap();
    say(format!("solve3(8) = {}", solve3(8)).as_bytes(), width, &mut writer).unwrap();
}

fn solve3(num: i64) -> i64 {
  (0..num).sum() 
}

fn solve2(num: i64) -> i64 {
  let mut result = 0;
  for i in 0..num {
    result += i
  }
  result
}

fn solve(num: i64) -> i64 {
  let mut result = 0;
  let mut i = 0;

  loop {
    if i >= num {
      break;
    }
    result += i;
    i += 1;
  }

  return result;
}
