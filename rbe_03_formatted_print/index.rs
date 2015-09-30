// format! - write formatted text to string
// print! - same as format, but print to console
// println! - same as print! but a newline is appended

fn main () {
  println!("{} days", 31);
  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
  println!("My name is {0}, {1} {0}", "Bond", "James");

  // debug prints literal strings
  println!("{:?}", "Bond");
}
