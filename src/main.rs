
// there are two files within the main directory, in.txt and out.txt
// read in.txt and compute number of words that show up more than once,
// output that number to out.txt

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;




fn main() {
    let mut f = File::open("in.txt").expect("file not found");
    let mut contents = String::new();
    print!("reading file...");
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("done");

    
}