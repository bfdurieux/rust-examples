  fn main() {
    println!("Hello, world!");
    run_fizzbuzz(Some(5));
  }


  fn run_fizzbuzz(upper_limit: Option<i32>) {
    for i in 0..upper_limit.unwrap_or(100) {
      println!("{}",fizzbuzz(i));
    }
  }

fn fizzbuzz(num: i32) -> String {
  let mut result = String::from("");

  if  num%3 == 0 {
    result.push_str("fizz");
  }

  if num%5 == 0 {
    result.push_str("buzz");
  }

  return result;
}
