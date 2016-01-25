use std::fs::File;
use std::io::Error;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    println!("Hello, world! {}",read_file_string_new().unwrap());
}



fn read_file_string_new()-> Result<i32, Error>{

let f = try!(File::open("foo.txt"));
let mut reader = BufReader::new(f);
let  sum:i32 = reader.lines().into_iter()
                  .map(|l| calculate(&(l.unwrap())).unwrap())
                  .fold(0, |acc, x| acc + x);
Ok(sum)
}



fn calculate(singleLine :&str) -> Result<i32,Error>{
let mut lbw:Vec<i32> = singleLine.split("x").into_iter()
                              .map(|s| s.trim())
                              .filter(|s| !s.is_empty())
                              .map(|s| s.parse().unwrap())
                              .collect();
lbw.sort();
assert!(lbw.len()==3);
let l = lbw[0];
let b = lbw[1];
let w = lbw[2];
Ok((2*l*b+2*b*w+2*l*w+l*b))
}
