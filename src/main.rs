//use core::num::{dec2flt::parse, self};
use std::{collections::HashMap, arch::x86_64::_andn_u32};
use std::io::Read;
use std::io;
use rand::{Rng, prelude::ThreadRng};
fn main() {
    let text = readfromtext();
    let Lines: Vec<&str> = text.split_inclusive('\n').collect();
    let mut numofquestions: usize = Lines.len();
    let mut userinput: String = String::new();
    let mut anshash:HashMap<&str, String> = HashMap::new();
    let mut count: usize = 0;
    let mut randnum = rand::thread_rng();
  
    for l in random_questions(numofquestions) {
        let Columns: Vec<&str> = Lines[l].split(                                                                                                                                                                                                                     '#').collect();
        println!("{}", Columns[0]);
        anshash =  displayoptions(Columns[1], generateoption_sequence(numofquestions));
        println!("Answer: ");
        userinput = readline().to_lowercase();  
        if anshash[userinput.trim()]== Columns[2].trim() {
            println!("Correct!!");
            count = count + 1;
        }else {
            println!("Incorrect!!");
        }  
    }
    total(count, numofquestions);
}
fn readfromtext() -> String {
    let mut file = std::fs::File::open("Data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
fn random_questions(number: usize) -> Vec<usize> {
    let mut sequence:Vec<usize> = Vec::new();
    let mut randnum = rand::thread_rng();
    loop {
        let selectedrand = randnum.gen_range(0..number);
        if sequence.contains(&selectedrand) {
        }else{
            sequence.push(selectedrand);
        }
        if sequence.len() == number {
            break;
        }
    }
    sequence
}
fn generateoption_sequence(number: usize) -> Vec<usize> {
    let mut sequence:Vec<usize> = Vec::new();
    let mut randnum = rand::thread_rng();
    loop {
        let selectedrand = randnum.gen_range(0..4);
        if sequence.contains(&selectedrand) {
        }else{
            sequence.push(selectedrand);
        }
        if sequence.len() == 4 {
            break;
        }
    }
    sequence
}
fn displayoptions(getoptions: &str, sequence: Vec<usize>) -> HashMap<&str,String> {
    let options: Vec<&str> = getoptions.split("|").collect();
    let optionletter: [&str; 4] = ["a", "b", "c","d"];
    let mut counter: usize = 0;
    let optionIndex: String;
    let mut anshash:HashMap<&str, String> = HashMap::new();
    for c in sequence {
       println!("{}. {}", optionletter[counter], options[c]);
       anshash.insert(optionletter[counter], c.to_string());
       counter = counter + 1;
    }
   anshash
}
fn readline() -> String {
    let mut userinput:String = String::new();
    io::stdin().read_line(& mut userinput);
    userinput
}
fn total(number: usize, quesnum: usize) {
    let percent: usize = (number * 100)/quesnum;
    let mut grade: String = String::new() ;
    println!("Overall: {}/{}", number,quesnum);
    println!("Percentage: {}%", percent);
    match percent {
        70..=100=> grade = "A".to_string(),
        60..=69=>  grade = "B".to_string(),
        45..=59=>  grade = "C".to_string(),
        35..=44=>  grade = "D".to_string(),
        0..=34=>   grade = "F".to_string(),
        _=> println!("Error Holder!!")
    }
    println!("Grade: {grade}");
}
