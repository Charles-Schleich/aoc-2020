
use std::fs::File ;
use std::io::prelude::*;
use std::io::BufReader;
use std::{thread, time};
use std::time::{Duration, Instant};

fn read_input(filename: &str) -> Vec<u16> {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut vec = Vec::new();
    let mut value = Ok(0 as usize); 
    while value.is_ok() {
        let mut line = String::new();
        value = reader.read_line(&mut line);
        if value.is_err(){ break; }
        let integer = match line.trim().parse::<u16>(){
            Ok(x)=>{x}
            Err(err)=>{break}
        };
        vec.push(integer);
    };
    vec
}


fn main() {
    let start = Instant::now();
    let input = read_input("./input/day1.txt");
    for i in 0..input.len(){
        for j in i..input.len(){
            if input[i]+input[j] == 2020 {
                println!("{},{}", input[i],input[j] );
                break
            }
        }      
    }

    let duration = start.elapsed();

    println!("time: {:?}",duration);
}


