fn main() {
  println!("---RusticDB---");
  let s = "Hello World!";
  println!("{:?}", s);

  // --
  for b in s.as_bytes() {
    print!("{}, ", b);
  }

  println!("");

  println!("{:?}", s.as_bytes());

  // --
  for c in s.chars() {
    print!("{}, ", c);
  }

  println!("");

  println!("{:?}", s.chars());
}


