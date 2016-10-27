// Store functionality of RusticDB
// (C) 2016-10-24 Jonas S Karlsson
// 
// Overall intentional design
// - stores buckets of tuples
// - a tuple has an bytes keyvalue, timestamp, deletemarker, bytes data
// - a bucket is "one file"
// - a bucket is mostly sorted
// - new values are appended, in memory and on file
// - sort order of vector should be retained, only deletemarker can be set
// - can allow it to be GCed according to policy (TBD)

use std::io::prelude::*;
use std::io::Error;
use std::fs::File;
use std::mem::transmute;

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

// could use - http://stackoverflow.com/questions/29037033/how-to-slice-a-large-veci32-as-u8
fn bytes(u: usize) -> [u8; 4] {
  let i = u as i32;
  return unsafe { transmute(i.to_be()) }; // or .to_le()
}

//fn i32(b: [u8; 4]) -> i32 {
//  let i = u as i32;
//  return unsafe { transmute(i.to_be()) }; // or .to_le()
//}

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

  fn write(&self) -> Result<(), Error> {
    println!("Write!");
    let mut f = try!(File::create("bucket001.txt"));

    for t in &self.vec {
      try!(f.write_all(&bytes(t.key.len())));
      try!(f.write_all(t.key));

      try!(f.write_all(&bytes(t.data.len())));
      try!(f.write_all(t.data));
    }
    try!(f.flush());

    Ok(())
  }

  fn read(&self) -> Result<(), Error> {
    println!("Read!");
    let mut f = try!(File::open("bucket001.txt"));

    // we read all data as one byte vector
    // then we set up
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes).unwrap();
    println!("Bytes read {} {:?}", bytes.len(), bytes);

    // TODO: parse it and make slices and put in arrays...
    

    Ok(())
  }

}

fn main() {

  let ta = Tuple{
    key: &[65,66,67],
    data: &[68,69,70],
  };

  let tb = Tuple{
    key: &[65,66,68],
    data: &[70,71,72],
  };

  let mut b = Bucket{
    vec: Vec::new(),
  };

  b.insert(ta);
  b.insert(tb);

  b.read().unwrap();
  b.write().unwrap();

  println!("---Storage");
  println!("{:?}", b);
}

