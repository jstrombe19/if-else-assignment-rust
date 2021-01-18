/*
1. Determine whether a given letter is a vowel
2. Create a basic calculator that supports +, -, *, /
3. Determine a final grade based on a supplied percentage
90%>= A
80%>= && <90% B
70%>= && <80% C
60%>= && <70% D
else F
*/

use std::io;

fn main() {// 1
  let mut a = String::new();
  println!("Enter a letter");
  io::stdin().read_line(&mut a).expect("Failed");
  a = a.trim().to_string().parse().expect("Failed");
  if a.to_lowercase() == "a" || a.to_lowercase() == "e" || a.to_lowercase() == "i" || a.to_lowercase() == "o" || a.to_lowercase() == "u" || a.to_lowercase() == "y" {
    println!("{} is a vowel!", a);
  } else {
    println!("{} is not a vowel.", a);
  }
}