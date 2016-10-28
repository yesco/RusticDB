// Store functionality of RusticDB
// (C) 2016-10-24 Jonas S Karlsson
// 
// This is NOT bigtable ( http://static.googleusercontent.com/media/research.google.com/en//archive/bigtable-osdi06.pdf )
//
// Overall intentional design
// - stores buckets of tuples
// - a tuple has an status header, bytes keyvalue, ~timestamp, bytes data
// - a bucket is "one file", can probably be memory mapped
// - a bucket is mostly sorted, at least up to a certain point
// - new values are appended, in memory and on (log)file
//   actually appending to file is expensive, if done often
//   so we instead can log a number of buckets appends to a single bucket
//   since the definition of key contains fully defined key we can see
//   = it as a message queue to be delivered to actual physical buckets
// - sort order of vector should be retained, on file, no need, but need encoding relative previous key
// - TBD: append only, no delete, just deletemarkers, GC policy?
//
// We have a global naming system for data by key:
//   key     : [byteslen,location,datakey], lexically ordered starting from location
//     byteslen: u64 bytes length of location + datakey
//     location: utf8 components - (universe\0galaxy\0star\0planet\0moon\0region\0table\0\ff)
//     datakey : bytes - (just bytes, not termination, )
//
// the location: \0\0\0\0\0\0\0\ff is not user insertable - system defined delimiter marker ("globals"?)
// - we don't separate the elements in our tuple, to achieve generic processing
// - all location components must exist, delimited by \0, non-empty strings are not allowed
//   (think about all the aliens!)
// - we terminate the location with \ff that is invalid unicode (this may allow for extensibilty)
// - datakey is just bytes, the length is not explicit (TODO: should it be?)
// - byteslen is
//
// accepted values, for normalization purposes it's english (until everyone learned chinese)
//   universe : "universe"
//   galaxy   : "milkyway"
//   star     : "sol"
//   planet   : "earth"
//   moon     : "earth"
//   region   : "asia", "africa", "america", "europe", "reversesite", "reverseurl"
//   table    : non-empty / can be a reverse url
//
// TODO: is the "location" bullshit? it's not mean to be location of data but the "authoritive owner"
// - is the owner the hoster? or the user it's stored on's behalf? In such case we need keys ;-)

use std::io::prelude::*;
use std::io::Error;
use std::fs::File;
use std::mem::transmute;

// Tuples are strcuts pointing into slices of the block,
// or if added later, on heap (how to do?)
#[derive(Debug)]
struct Tuple<'a> {
  status: u8,
  tbd: [u8; 3],
  key: &'a [u8],
  timestamp: u64,
  data: &'a [u8],
}

// Bucket contains a list of Tuples
#[derive(Debug)]
struct Bucket<'a> {
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
      // TODO: add timestamp, status, tbd
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
    status: 0xff,
    timestamp: 1,
    tbd: [0,0,0],

    key: &[65,66,67],
    data: &[68,69,70],
  };

  let tb = Tuple{
    status: 0xff,
    timestamp: 1,
    tbd: [0,0,0],

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

