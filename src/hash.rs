use std::fs::File;
use std::str;
use std::fs::OpenOptions;
use std::fs;

use std::io::prelude::*;
use std::io::{BufReader, BufRead};
fn file_exists(path: String) -> bool{ return std::path::Path::new(&path).exists() }

pub fn hand_input(input: String) ->std::io::Result<()>{
    
    let rez = get_hash(&input.clone().to_owned());
    println!("Your hashed input: {}", rez);
    
    
    Ok(())
}


pub fn hash(file_location: String, known_format :i32) -> std::io::Result<()>{

    let f =  File::open(file_location.clone())?;
    let reader =  BufReader::new(f);
    let cur_dir = std::env::current_dir()?.into_os_string().into_string().unwrap().to_string() + "/";
    
    if file_exists(cur_dir.clone() + "/rez.txt"){
        fs::remove_file(cur_dir.clone() + "/rez.txt")?;
    }
    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .create_new(true)
    .open(cur_dir.clone() + "rez.txt")
    .unwrap();
    
    if known_format == 0 {
        for line in reader.lines(){
            let line = line.expect("Unable to read line!");
            println!("{}", get_hash(&line));
        }
    }
    else if known_format == 1{
        for line in reader.lines() {
            let line = line.expect("Unable to read line!");
            let split = line.split(",");
            
            let vec: Vec<&str> = split.collect();

            // println!("File generated!");
            for elem in vec {
               if let Err(e) = write!(file, "{} ", get_hash(elem)) {
                eprintln!("Couldn't write to file: {}", e);
                }
            }
            if let Err(e) = write!(file, "\n") {
             eprintln!("Couldn't write to file: {}", e);

            }
        }
    }
    else{
        let mut rez: String = String::new();
        for lines in reader.lines() {
            rez.push_str(&lines.unwrap().to_owned());
        }
        println!("{}",get_hash(&rez.to_owned()));
    }
    Ok(())
}

// problem input
// aHLev1n16Wfm6jeYsZeg8B8ztQFFQTU4d3zG3SeRdtejX1qymHZtW3Ju2DGKfFTEt5oSLkV6ti8Was34i7pZxkpG00NIBhfl3CjkLaXJzpjpQXNiuwsxMI7XbKk41fwIU6tHJaCyV3fZ9PUFtcQbAvoo27cv5TQEUeL4vyDc8NvQZFzJAfrLlwkRIQDyVgFnPKcfxs8Rx6F9Edoxwk2bPN7rTRWoyGo5rc3qLdBviKV8z62xNA8z4PYbGd9ORF6tdjPLlzqBO9Byy2hGuDSSGkN8IYhbfMUaEfiau8Q6FVDy60vfTibuVOtgcebPM7BZxbY0QFVpF60wRdCInX3XaDN38SAvQEqNJUGiukDSP0TZgv9FHhHqUcVRMekg2CB9HkrHuxvFC7hWwaSCVyEhv13nryo0ThL6bjSmGDSx6w4NnZqVr0QL1g2K5VS58PfhYNdAEuoKdWW1Dni8ql48LK4co1qPL6ngAac7hZrnsfRgg4dgPtmvDRDdYqxGpipjzOeKxIArKwwoxfMu4yXBUOXwmWgdVJ7ARi68uiRSPiRXlYInK3FXwF1ykie43SP3Ck7KFqc4jgR6XjtrD3pc6lDBIwYAcYnhKqXMQcMkoTxNXH4MgZTcQGmcJoCjEzd2u2Mar1OgsWpwQbPrYeHT6FnboESrOuA6VPtM6x1ZTbEOaQ5yNoyLHJO9HjofQBCZYuleV14cYd17g2VEM7b776NPuW4YmNlGG2iXfskAOcyT6F8nJhtLffXHu17T3djDaCpD0avmFeKbUv17GoJpoRsbR4feCoW5ZyTn4IIkZ2jZgN9MpMQutz83kA1mwJ4YTBwgTCun6loPpDxzAb0MxDTbD2W0bM0LRKL1I6Qsp1tEsDvF6ySfTtHI856Pq32LP5prRHuXEsoauX5qanKyCRVh5Uj1cCvGp82QXCQt7KvSIj2mt9e2nvv1vtNRIdjH1t5BOUJi5nBIULPfop8A21vQK9VsxYKyCu1Pz6YY
// 2HLev1n16Wfm6jeYsZeg8B8ztQFFQTU4d3zG3SeRdtejX1qymHZtW3Ju2DGKfFTEt5oSLkV6ti8Was34i7pZxkpG00NIBhfl3CjkLaXJzpjpQXNiuwsxMI7XbKk41fwIU6tHJaCyV3fZ9PUFtcQbAvoo27cv5TQEUeL4vyDc8NvQZFzJAfrLlwkRIQDyVgFnPKcfxs8Rx6F9Edoxwk2bPN7rTRWoyGo5rc3qLdBviKV8z62xNA8z4PYbGd9ORF6tdjPLlzqBO9Byy2hGuDSSGkN8IYhbfMUaEfiau8Q6FVDy60vfTibuVOtgcebPM7BZxbY0QFVpF60wRdCInX3XaDN38SAvQEqNJUGiukDSP0TZgv9FHhHqUcVRMekg2CB9HkrHuxvFC7hWwaSCVyEhv13nryo0ThL6bjSmGDSx6w4NnZqVr0QL1g2K5VS58PfhYNdAEuoKdWW1Dni8ql48LK4co1qPL6ngAac7hZrnsfRgg4dgPtmvDRDdYqxGpipjzOeKxIArKwwoxfMu4yXBUOXwmWgdVJ7ARi68uiRSPiRXlYInK3FXwF1ykie43SP3Ck7KFqc4jgR6XjtrD3pc6lDBIwYAcYnhKqXMQcMkoTxNXH4MgZTcQGmcJoCjEzd2u2Mar1OgsWpwQbPrYeHT6FnboESrOuA6VPtM6x1ZTbEOaQ5yNoyLHJO9HjofQBCZYuleV14cYd17g2VEM7b776NPuW4YmNlGG2iXfskAOcyT6F8nJhtLffXHu17T3djDaCpD0avmFeKbUv17GoJpoRsbR4feCoW5ZyTn4IIkZ2jZgN9MpMQutz83kA1mwJ4YTBwgTCun6loPpDxzAb0MxDTbD2W0bM0LRKL1I6Qsp1tEsDvF6ySfTtHI856Pq32LP5prRHuXEsoauX5qanKyCRVh5Uj1cCvGp82QXCQt7KvSIj2mt9e2nvv1vtNRIdjH1t5BOUJi5nBIULPfop8A21vQK9VsxYKyCu1Pz6YY
pub fn get_hash(input: &str) -> String{

    let mut rez: String = String::new();
    let mut byte_array: [[u8; 16];8] = [[00; 16]; 8];
    

    let mut block: usize = 0;
    let mut counter: usize = 0;

    for elem in input.chars() {
        let byte_val = elem as u32;


        byte_array[block][counter] = ((byte_array[block][counter] as i32 + byte_val as i32 * 256) % 126) as u8;
        
        if byte_array[block][counter] < 32{
            byte_array[block][counter] = byte_array[block][counter] + 32;
        }

        counter = counter + 1; 

        if counter == 16{
            block = block +1;
            counter = 0;
            if block == 8{
                block = 0;
            }
        }
        
    }
    let mut temp_str = String::new();
    let mut temp_ascii: u8;
    
    for elem in padding(byte_array) {

        for  single_char in elem {
        temp_ascii = single_char;
        temp_str = format!("{}{}",temp_str,temp_ascii as char);
        }
    }
    rez = temp_str;
    rez
}

fn padding(mut input: [[u8; 16]; 8]) -> [[u8; 16]; 8]{
    
    let mut block: usize = 0;

    let mut back_counter: usize = 15;

    let mut excess: i32 = 1;


    let mut temp_val: i32 = 1;

    let mut counter = 0;

    loop{
        input[block][counter] = (((input[block][0] as i32 * 127) % 255 + input[block][back_counter] as i32) % 255) as u8;


        temp_val += input[block][counter] as i32 * input[block][counter] as i32 * excess as i32;

        temp_val /= 11;

        input[block][counter] = temp_val as u8;
        
        let mut temp_counter: usize = 0;

        for mut elem in input[block] {

            if elem > 126 {
                excess += (elem as i32 / 126 * 5) as i32; 
                elem %= 126;
                
            }

            excess += elem as i32 * 10 /126;


            elem = ((elem as i32 * input[block][temp_counter] as i32 * excess)%126) as u8;


    
            if elem < 32 {
                excess += ((elem)) as i32;
                elem += 33;
            }
            input[block][temp_counter] = elem;
            temp_counter +=1;

            
    
            
        }
        
        if back_counter > 0 {
            back_counter = back_counter - 1;
        }
        

        if counter == 15 {
            if block == 7{
                break;
            }
            block += 1;

            // excess /=45;

            input[block][0] = (input[block][0] + ((excess * excess)% 126)as u8) % 126;
            
            if input[block][0] < 32 {
                excess += ((input[block][0])) as i32;
                input[block][0] +=  33;
            }
            
            back_counter = 15;
            temp_val %= 255;
            counter = 0;
        }
        counter +=1;
    }
    let mut count1: usize = 0;
    let mut count2: usize = 0;
    for blocks in input {
        for mut chars in blocks {
            chars = ((chars as i32 * excess) % 125) as u8;
            if chars <= 32 {
                chars += 33;
            }
            excess -=50;
            input[count1][count2] = chars;
            count2 +=1;
        }
        excess =excess + excess /2;
        count1 +=1;
        count2 = 0;
    }

    input
}
