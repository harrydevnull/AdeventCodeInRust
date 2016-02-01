

extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
fn main() {

    let input:&str = "iwrupvqb";
    let mut hash = Md5::new();
    let mut count = 1;
    loop {
        let hash_str = format!("{}{}",input,count);
        hash.input_str(&hash_str);


        if hash.result_str().starts_with("00000") {
            println!("{},{:?}",input,count );

            break;
        }
        count += 1;
        hash.reset();
    }

    println!("Hello, world!");
}
