use std::fs::File;
use std::str;
// use std::fs::OpenOptions;
// use std::fs;

// use std::io::prelude::*;
use std::io::{BufRead, BufReader};

pub fn check_col() -> std::io::Result<()> {
    let file_location = std::env::current_dir()?
        .into_os_string()
        .into_string()
        .unwrap()
        .to_string()
        + "/rez.txt";
    let f = File::open(file_location.clone())?;

    let mut count: usize = 0;
    let reader = BufReader::new(f);
    let mut diff_avg: u32 = 0;
    let mut nr_checks = 0;
    for line in reader.lines() {
        nr_checks += 1;
        let temp_line = line.unwrap().clone();
        count += check_repeating_count(temp_line.clone());
        diff_avg += check_byte(temp_line.clone()) as u32;
    }
    diff_avg = diff_avg / nr_checks;
    println!("The average difference in bytes: {}", diff_avg.to_string());
    
    println!("The ammount of collision: {}", count.to_string());
    Ok(())
}

fn check_byte(value: String) -> u32 {
    let split = value.split(" ");

    let v: Vec<&str> = split.collect();

    let mut diff: [u32; 2] = [0, 2];

    let mut array: [&str; 3] = [""; 3];

    let mut counter: usize = 0;
    for elem in v {
        array[counter] = elem;
        counter += 1;
    }

    let mut rez: u32 = 0;
    let str1: String = array[0].to_string();
    let str2: String = array[1].to_string();
    for elem in str1.chars() { 
        diff[0] += elem as u32;

    }
    for elem in str2.chars() { 
        diff[1] += elem as u32;
        
    }
    let sub: i32 = (diff[0] as i32 - diff[1] as i32) as i32;
    if sub > 0 {
        rez = diff[0] - diff[1];
    } else {
        let temp: i32 = diff[0] as i32 - diff[1] as i32;
        rez = (temp * -1) as u32;
    }
    rez
}
// fn check_hex() -> f64{

// }

//"7:**Fc?*::+F**X7.$.m:dXX!!:+*Q.@+:"+UX:*C!:%O"$m*c!*@![$7*:*$XQC77C!O*m*:d*!@X3*$?Q7:cmmOc7!X!:!XX!:!XOQ$X.c:"$.cQCX77XXO"Q"?C$1 7:**Fc?*::+F**X7.$.m:dXX!!:+*Q.@+:"+UX:*C!:%O"$m*c!*@![$7*:*$XQC77C!O*m*:d*!@X3*$?Q7:cmmOc7!X!:!XX!:!XOQ$X.c:"$.cQCX77XXO"Q"?C$1 "
//"QcO*CO$!Q"Q"U@O$CX:+CccCd..%m"QHXm%$?%7dXFQ?X:(@QQ7*$X[dcmXd::%yX*c$mU7:Qc$Q%:c1UOpXXccOXO!!7X+:?$7$UCQm.[cQFOO+:*:OO*dCd*.7$C:d +@?@CXX"?p7Qm7F3XX?":"Q*O*C:X:[3**Q@Q:":m7%XcQQX!X!C%7*UU*+%c@X*7!Q%UQpdc@$U%.3@FpU.dcQ*:"7X*QQ*X.mFOX**[!Q"?*O1OQ@O+$:O!!*+7U:= "
fn check_repeating_count(value: String) -> usize {
    let split = value.split(" ");

    let v: Vec<&str> = split.collect();

    let mut array: [&str; 3] = [""; 3];

    let mut counter: usize = 0;
    for elem in v {
        array[counter] = elem;
        counter += 1;
    }
    counter = 0;
    if array[0] == array[1] {
        counter += 1;
    }
    counter
}
