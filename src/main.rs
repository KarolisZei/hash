// use std::fs::File;
use std::fs;
use std::path::Path;
// use futures::executor::block_on;
mod hash;
use sha256::digest;

use hash::get_hash;
use hash::hand_input;
use hash::hash;

mod create_files;
use std::time::{Duration, Instant};

use create_files::create_1;
use create_files::create_1000_1;
use create_files::create_1000_diff;
use create_files::create_empty;
use create_files::generate_100000;
use create_files::generate_100000_different_1;

mod check_collisions;

use check_collisions::check_col;

// use std::io::*;ca
use std::io;

pub trait Copy: Clone {}

fn create_dir() -> std::io::Result<()> {
    let cur_dir = std::env::current_dir()?;

    let create_gen_files =
        cur_dir.into_os_string().into_string().unwrap().to_string() + "/gen_files";

    let temp_dir = create_gen_files.clone();

    if Path::new(&create_gen_files).exists() == false {
        fs::create_dir(create_gen_files)?;
    }

    create_1(temp_dir.clone())
        .map_err(|err| println!("{:?}", err))
        .ok();
    create_empty(temp_dir.clone())
        .map_err(|err| println!("{:?}", err))
        .ok();

    create_1000_diff(temp_dir.clone())
        .map_err(|err| println!("{:?}", err))
        .ok();
    create_1000_1(temp_dir.clone())
        .map_err(|err| println!("{:?}", err))
        .ok();

    generate_100000(temp_dir.clone())
        .map_err(|err| println!("{:?}", err))
        .ok();
    Ok(())
}

fn file_exists(path: String) -> bool {
    return std::path::Path::new(&path).exists();
}

fn read() -> std::io::Result<()> {
    println!("Do you want to create new files[Y/N]: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("unable to parse input");

    if input.eq("y\n") || input.eq("Y\n") {
        println!("Creating new files...");
        create_dir().map_err(|err| println!("{:?}", err)).ok();
    } else if !input.eq("n\n") && !input.eq("N\n") {
        println!("Please enter the correct input!");
        read().map_err(|err| print!("{:?}", err)).ok();
    }
    let cur_dir = std::env::current_dir()?
        .into_os_string()
        .into_string()
        .unwrap()
        .to_string()
        + "/gen_files";

    loop {
        println!("What file do you want to read?");
        println!("Empty file: 1");
        println!("Single char files: 2");
        println!("Single string 1000 char length files: 3");
        println!("Single string 1000 char length files that differ by one char: 4");
        println!("Different length string pairs file: 5");
        println!("Different length strings that differ by one value: 6");
        println!("Custom text file: 7");
        println!("Hand input: 8");
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("unable to parse input");
        let temp_dir = cur_dir.clone();
        if input.eq("1\n") {
            if !file_exists(cur_dir.clone() + "/empty_file.txt") {
                println!("File doesn't exists, Creating file!");
                create_empty(temp_dir.clone())
                    .map_err(|err| println!("{:?}", err))
                    .ok();
                println!("File created!");
            }
            hash(temp_dir.clone() + "/empty_file.txt", 0)
                .map_err(|err| println!("{:?}", err))
                .ok();
            break;
        } else if input.eq("2\n") {
            if !file_exists(cur_dir.clone() + "/single_char2.txt")
                || !file_exists(cur_dir.clone() + "/single_char2.txt")
            {
                println!("File doesn't exists, Creating file!");
                create_1(temp_dir.clone())
                    .map_err(|err| println!("{:?}", err))
                    .ok();
                println!("File created!");
            }

            print!("First file: ");
            hash(temp_dir.clone() + "/single_char1.txt", 0)
                .map_err(|err| println!("{:?}", err))
                .ok();

            print!("Second file: ");
            hash(temp_dir.clone() + "/single_char2.txt", 0)
                .map_err(|err| println!("{:?}", err))
                .ok();

            break;
        } else if input.eq("3\n") {
            if !file_exists(cur_dir.clone() + "/1000_rand1.txt")
                || !file_exists(cur_dir.clone() + "/1000_rand2.txt")
            {
                println!("File doesn't exists, Creating file!");
                create_1000_1(cur_dir.clone())
                    .map_err(|err| println!("{:?}", err))
                    .ok();
                println!("File created!");
            }

            print!("First file: ");
            hash(temp_dir.clone() + "/1000_rand1.txt", 0)
                .map_err(|err| println!("{:?}", err))
                .ok();
            print!("Second file: ");
            hash(temp_dir.clone() + "/1000_rand2.txt", 0)
                .map_err(|err| println!("{:?}", err))
                .ok();

            break;
        } else if input.eq("4\n") {
            if !file_exists(cur_dir.clone() + "/1000_rand_diff_1.txt")
                || !file_exists(cur_dir.clone() + "/1000_rand_diff_2.txt")
            {
                println!("File doesn't exists, Creating file!");
                create_1000_diff(temp_dir.clone())
                    .map_err(|err| println!("{:?}", err))
                    .ok();
                println!("File created!");
            }
            print!("First file: ");
            hash(temp_dir.clone() + "/1000_rand_diff_1.txt", 0)
                .map_err(|err| println!("{:?}", err))
                .ok();
            print!("Second file: ");
            hash(temp_dir.clone() + "/1000_rand_diff_2.txt", 0)
                .map_err(|err| println!("{:?}", err))
                .ok();

            break;
        } else if input.eq("5\n") {
            if !file_exists(cur_dir.clone() + "/100000_rand_pairs.txt") {
                println!("File doesn't exists, Creating file!");
                generate_100000_different_1(temp_dir.clone())
                    .map_err(|err| println!("{:?}", err))
                    .ok();
                println!("File created!");
            }
            hash(temp_dir.clone() + "/100000_rand_pairs.txt", 1)
                .map_err(|err| println!("{:?}", err))
                .ok();

            break;
        } else if input.eq("6\n") {
            if !file_exists(cur_dir.clone() + "/100000_pairs_diff.txt") {
                println!("File doesn't exists, Creating file!");
                generate_100000_different_1(temp_dir.clone())
                    .map_err(|err| println!("{:?}", err))
                    .ok();
                println!("File created!");
            }

            hash(temp_dir.clone() + "/100000_pairs_diff.txt", 1)
                .map_err(|err| println!("{:?}", err))
                .ok();

            check_col();

            break;
        }
        // /home/kek/Documents/prog/Hash/konstitucija.txt
        else if input.eq("7\n") {
            println!("Please enter a location: ");
            let mut location = String::new();
            io::stdin()
                .read_line(&mut location)
                .expect("Unable to parse input!");
            location.pop();

            while !file_exists(location.clone()) {
                println!("File does not exist! Please enter a real location: ");
                location = String::new();
                io::stdin()
                    .read_line(&mut location)
                    .expect("Unable to parse input");
                location.pop();
            }
            hash(location, 5)?;
            break;
        } else if input.eq("8\n") {
            println!("Please enter your value: ");
            let mut hand = String::new();
            io::stdin()
                .read_line(&mut hand)
                .expect("Unable to parse input!");
            hand.pop();
            hand_input(hand)?;
            break;
        } else {
            println!("Please select one of the numbers! \n");
        }
    }
    Ok(())
}

fn main() {
    println!("Do you want to make a Sha256 speed comparison hashing 'hello_world' [Y]");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("unable to parse input");

    if input.eq("Y\n") || input.eq("y\n") {
        let hello_world = "hello_world";
        let mut sha_val: String = String::new();
        let now = Instant::now();

        for elem in 1..1000000 {
            sha_val = digest(hello_world);
        }
        let sha_time = now.elapsed().as_secs();

        println!(
            "It took sha256 {} secs to hash 'hello_world' 1000000 times, resulting hash: {}",
            now.elapsed().as_secs(),
            sha_val
        );

        let now = Instant::now();
        let mut hash_val: String = String::new();

        for elem in 1..1000000 {
            hash_val = get_hash(hello_world);
        }
        let hash_time = now.elapsed().as_secs();
        println!(
            "It took my hash {} secs to hash 'hello_world' 1000000 times, resulting hash: {}",
            now.elapsed().as_secs(),
            hash_val
        );
        println!("Sha256 is {} times faster", hash_time / sha_time);
    }

    read().map_err(|err| print!("{:?}", err)).ok();
}
