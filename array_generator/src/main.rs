extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;


fn main(){
    println!("input the size of 2d array");

		let mut file = File::create("tmp.foo").expect("Unable to create file");
/*		let mut guess = String::new();
		
		io::stdin().read_line(&mut guess)
							.expect("failed to read line");

		let mut guess2: u32 = guess.trim().parse()
							.expect("Please type a number");
*/
		let mut array_2d = [[0u32;100];  100];
		for x in 0.. 100
		{
			for y in 0.. 100
			{
				array_2d[x][y] = rand::thread_rng().gen_range(1,1001);
				file.write_all(array_2d[x][y]).expect("Unable to write file!!");
	//			file.write_all(b"Hello")?;
			}
	//		file.wrtie(b"\n");
		}

}
