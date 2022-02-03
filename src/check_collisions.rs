use std::fs::File;
use std::str;
use std::fs::OpenOptions;
use std::fs;

use std::io::prelude::*;
use std::io::{BufReader, BufRead};

pub fn check_col() -> std::io::Result<()>{
    

    let file_location = std::env::current_dir()?.into_os_string().into_string().unwrap().to_string() + "/rez.txt";
    let f = File::open(file_location.clone())?;

    let mut count: usize = 0;

    let reader = BufReader::new(f);

    for line in reader.lines() {
        count += check_repeating_count(line.unwrap());
    }
    println!("The ammount of collision: {}", count.to_string());
    Ok(())
}

// fn check_dec() -> f64{

// }

// fn check_hex() -> f64{

// }

fn check_repeating_count(value: String) -> usize{
    let v: Vec<&str> = value.split(",").collect();
    let mut array: [&str;3] = ["";3];



    let mut counter: usize = 0;
    for elem in v {
        array[counter] = elem;
        counter +=1;
    }
    counter = 0;
    if array[0] == array[1] {
        counter+=1;
    }
    counter

}