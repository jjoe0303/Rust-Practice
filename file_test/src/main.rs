use std::fs::File;
use std::io::prelude::*;

fn main()  {
    let mut file = File::create("foo.txt");
		    file.write_all(b"Hello, world!");
						}
