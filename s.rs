#[derive(Debug)]
struct Tuple<'a> {
  key: &'a [u8],
  data: &'a [u8],
}

#[derive(Debug)]
struct Bucket<'a> {
  //element: &'a [&'a Tuple<'a>],
  vec: Vec<Tuple<'a>>,
}

impl<'a> Bucket<'a> {

  fn len(&self) -> usize {
    return self.vec.len();
  }

  fn insert(&mut self, t: Tuple<'a>) {
    println!("INSERT! {:?}", t);
    println!("  {:?}", self);
    self.vec.push(t);
    println!("  {:?}", self);
  }

}

fn main() {

  let ta = Tuple{
    key: &[1,2,3],
    data: &[3,42,65],
  };

  let tb = Tuple{
    key: &[1,2,7],
    data: &[3,42,67],
  };

  let mut b = Bucket{
    vec: Vec::new(),
  };

  b.insert(ta);
  b.insert(tb);

  println!("---Storage");
  println!("{:?}", b);
}

