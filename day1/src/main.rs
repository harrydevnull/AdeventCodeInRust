use std::io::prelude::*;
use std::fs::File;
use std::io::Error;


fn main() {

let s = read_file_string().unwrap();
   println!("diff is! {}",count(&s).unwrap());
}

fn count(str:&str)->Result<usize,Error>{
//let mut lvec = Vec::new();
let  lvec:Vec<char> = str.chars().into_iter().filter(|x| *x == '(').collect();
let  rvec:Vec<char> = str.chars().into_iter().filter(|x| *x == ')').collect();

Ok((lvec.len()-rvec.len()))
}

fn read_file_string()-> Result<String, Error>{
let mut s = String::new();
let mut f = try!(File::open("foo.txt"));
try!(f.read_to_string(&mut s));
Ok((s))
}
