
use std::fs::File ;
use std::io::prelude::*;
use std::io::BufReader;
use std::{thread, time};
use std::time::{Duration, Instant};


fn read_input(filename: &str) -> Vec<u32> {

    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut vec = Vec::new();
    let mut value = Ok(0 as usize);

    while value.is_ok() {
        let mut line = String::new();
        value = reader.read_line(&mut line);
        let integer = match line.trim().parse::<u32>(){
            Ok(x)=>{x}
            Err(err)=>{break}
        };
        vec.push(integer);
    };
    vec
}

pub fn day1(){

    let input = read_input("./input/day1.txt");
    let mut first= 0;
    let mut second= 0;
    for i in 0..input.len(){
        for j in i..input.len(){
            if input[i]+input[j] == 2020 {
                first = input[i];
                second = input[j];
                break
            }
        }      
    }

}
