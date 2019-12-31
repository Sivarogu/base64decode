use std::str;

fn base64decode(i:String)->String{String::from_utf8(i.trim().chars().filter(|&c|c!='=').fold(Vec::new(),|mut a,c|{a.extend(format!("{:06b}","ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".find(c).unwrap()).bytes());a}).chunks(8).map(|c|{u8::from_str_radix(str::from_utf8(c).unwrap(),2).unwrap()}).collect()).unwrap()}
 
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    println!("Input: {}", input);
 
    let output = base64decode(input);
    println!("Output: {}", output);
}