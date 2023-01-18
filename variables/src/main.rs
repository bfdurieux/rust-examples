use core::num;
use std::io;
fn main() {
//   let number: f64 = 1.0;
//     let width: usize = 5;
//     println!("{number:>width$}");


//     let condition = true;
//     let number = if condition { 6 } else { 2 };

//     println!("The value of number is: {number}");

//     if number < 5 {
//       println!("condition was true!");
//     } else {
//       println!("condition was false!");
//     }


//     let mut counter:i32 = 5;
//     let result = 'first: loop {
//       println!("again");
//       counter -= 1;

//       if counter == 0 {
//         break  'first counter + 5;
//       }
//     };

//     println!("The result is {result}");


//     let mut while_num = 3;
//     while while_num != 0 {
//       println!("{}", while_num);
//       while_num -= 1;
//     }
//     println!("done");

//     for element in (1..10).rev() {
//       println!("{}", element);
//     }


//     // convert temperatures F to C

//     let f_temperature = 60;
//     // equation: (32°F − 32) × 5/9 
//     let c_temperature = (f_temperature - 32) * 5/9;
//     println!("{f_temperature}F in C: {c_temperature}");
// {
//   const NUM_TO_MATCH: i32 = 5;
//   let mut sequence = Vec::new();
//   let mut curr_fibonacci = 1;
//   let mut previous_fibonacci = 0;

//   while curr_fibonacci <= NUM_TO_MATCH {
//     let temp = curr_fibonacci;
//     curr_fibonacci = curr_fibonacci + previous_fibonacci;
//     previous_fibonacci = temp;

//     sequence.push(curr_fibonacci);
//     if curr_fibonacci == NUM_TO_MATCH {
//     }
//   }
//   if sequence.contains(&NUM_TO_MATCH) {
//     println!("{NUM_TO_MATCH} is part of the sequence!");
//   } else {
//     println!("{NUM_TO_MATCH} is not part of the sequence!");
//   }
// }


// use rand::Rng;
// {

//   const DAYS_IN_YEAR: i32 = 365;
//   let mut income: Vec<i32> = Vec::new();
//   for element in 0..DAYS_IN_YEAR {
//     let random_income = rand::thread_rng().gen_range(0..=500);
//     income.push(random_income);
//   }
  
//   let mut smallest: i32;
//   let mut largest: i32;
//   for i in income {
//     if i == 0 {
//       continue;
//     }
//     if i < smallest {
//       smallest = i;
//     }
//     if i > largest {
//       largest = i;
//     }

//   }
// }

// {
//   let mut state_income:(str, f32) = [("sp", 67836.43),("rj", 36678.66),("mg", 29229.88),("es", 27165.48), ("outros", 19849.53)];
//   let mut v = vec![5, 6, 8, 4, 2, 7];

//   println!("{:?}", result);
// }

//     let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh","eighth", "ninth", "tenth", "eleventh", "twelveth"];
//     let gifts = ["A partridge in a pear tree", "Two turtle doves,", "Three French hens,", "four calling birds," , "five gold rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming
//   "];

//     for day in 0..12 {
//       println!("On the {} day of Christmas my true love sent to me", days[day]);
//       let mut buffer = day;
//       while buffer > 0 {
//         println!("{}", gifts[buffer]);
//         buffer -= 1;
//       }
//       println!("{}\n", gifts[0]);
//     }

//     let mut s = String::from("hello"); //Mutable string

//     s.push_str(" world");

//     println!("{s}");

//     let s1 = String::from("hello");
//     let s2 = s1; //s1 is no longer valid

//     // println!("{}, world!", s1); would cause an error
//     println!("{}, world!", s2);
//   //instead of creating a shallow copy, rust performs a move by invalidating the first variable
//   //rust never automatically creates deep copies of your data
//     // to deeply copy use s.clone()
// }

  let mut s1 = String::from("hello");
  log(&get_len_by_ref(&s1).to_string());

  change(&mut s1);//borrows as mutable
  log(&s1);
}

fn get_len_by_ref(str: &String) -> usize {//using & means only a reference is passed to the function
  return str.len();
}

fn log(value: &String) {
  println!("{}", value);
}

fn change(str: &mut String) {
  str.push_str(" ayy");
}

// Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
