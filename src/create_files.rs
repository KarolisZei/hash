// use std::fs::File;
extern crate rand;
// use std::io::Write;
use std::fs;
use rand::Rng;
use std::io::Write;
use rand::{distributions::Alphanumeric};
// use std::str;
// use rand::prelude::*;11

pub trait Copy: Clone { }

pub fn create_1(temp_dir: String) -> std::io::Result<()>{
    for i in 1..3 {
    let new_dir = temp_dir.clone() + "/single_char" + &i.to_string() + ".txt";

    
    let mut rng = rand::thread_rng();

    let ran_char: char = rng.gen_range(b'A'..b'Z') as char;
    
    fs::write(new_dir, ran_char.to_string())?;
    
    }
    
    Ok(())
    
}

pub fn create_empty(mut temp_dir: String)-> std::io::Result<()>{
    temp_dir = temp_dir + "/empty_file.txt";

    let write_dir = temp_dir.clone();

    fs::write(write_dir,"")?;
    
    Ok(())
    
}

pub fn create_1000_diff(temp_dir: String) -> std::io::Result<()>{
    for i in 1..3 {
        let new_dir = temp_dir.clone() + "/1000_rand" + &i.to_string() + ".txt";

        let mut rand_int = rand::thread_rng();

        let mut rez: String = String::from("");
        for __j in 1..1001 {
                        
            let rand_val: u8 = rand_int.gen_range(48..122);
            let char_nr = rand_val as char;
            // println!("{}",char_nr);
            rez = format!("{}{}", rez, char_nr);
            
            // println!("{:?}" , string_nr);
        }

        fs::write(new_dir, rez.to_string())?;

    }
    Ok(())
    
}

pub fn create_1000_1(temp_dir: String) -> std::io::Result<()>{
    let mut rand_int = rand::thread_rng();

    let mut rez: String = String::from("");
    for __j in 1..1000 {
                    
        let rand_val: u8 = rand_int.gen_range(48..122);
        let char_nr = rand_val as char;
        // println!("{}",char_nr);
        rez = format!("{}{}", rez, char_nr);
        
        // println!("{:?}" , string_nr);
    }
    let new_dir = temp_dir.clone() + "/1000_rand_diff_1.txt";

    fs::write(new_dir, rez.to_string())?;

    let rand_val: u8 = rand_int.gen_range(48..122); 

    rez.replace_range(500..501, &(rand_val as char).to_string());

    

    let new_dir = temp_dir.clone() + "/1000_rand_diff_2.txt";

    fs::write(new_dir, rez.to_string())?;

    Ok(())
    
}

fn gen_range(range: usize, temp_dir: String){
    let mut new_dir =  fs::OpenOptions::new()
    .write(true)
    .append(true)
    .open(temp_dir.clone() + "/100000_rand_pairs.txt")
    .unwrap();
    
    let mut rez: String;

    let temp_rez: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(range)
        .map(char::from)
        .collect();

    rez = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(range)
    .map(char::from)
    .collect();

    rez = format!("{},{}", rez, temp_rez);

    write!(new_dir,"{} \n",rez);
}

pub fn generate_100000(temp_dir: String) -> std::io::Result<()>{
    fs::write(temp_dir.clone() + "/100000_rand_pairs.txt", "")?;

    
    for i in 1..100000 {
        if i < 25000 {
            gen_range(10, temp_dir.clone());
        }
        else if i < 50000{
            gen_range(100, temp_dir.clone());
        }
        else if i < 75000{
            gen_range(1000, temp_dir.clone());
        }
        else{
            gen_range(10000, temp_dir.clone());
        }
    }

    Ok(())
}

fn gen_pair_range(range: usize, temp_dir: String){
    let mut new_dir =  fs::OpenOptions::new()
    .write(true)
    .append(true)
    .open(temp_dir.clone() + "/100000_pairs_diff.txt")
    .unwrap();


    let mut rng = rand::thread_rng();

    let ran_char: char = rng.gen_range('a'..'z');
    
    let mut rez: String;

    let temp_rez: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(range)
        .map(char::from)
        .collect();

    rez = temp_rez.clone();

    
    let mut temp_char = ran_char.to_string();

    while temp_char.chars().next().unwrap() == rez.chars().next().unwrap(){
        temp_char = rng.gen_range('a'..'z').to_string();

    }

    rez.replace_range(0..1, &temp_char.to_string());
    
    rez = format!("{},{}", rez, temp_rez);

    write!(new_dir,"{} \n",rez);
}


pub fn generate_100000_different_1(temp_dir: String) -> std::io::Result<()>{
    fs::write(temp_dir.clone() + "/100000_pairs_diff.txt", "")?;

    
    for i in 1..100000 {
        if i < 25000 {
            gen_pair_range(10, temp_dir.clone());
        }
        else if i < 50000{
            gen_pair_range(100, temp_dir.clone());
        }
        else if i < 75000{
            gen_pair_range(1000, temp_dir.clone());
        }
        else{
            gen_pair_range(10000, temp_dir.clone());
        }
    }

    
    Ok(())
}