use std::env;
use std::fs;
use std::string;
use unicode_segmentation::UnicodeSegmentation;

fn check_aussies(lslocs: Vec<Option<usize>>) -> bool {
    let res:bool;
    if lslocs.contains(&None){
        res = true
    } else {
        res = false
    }
res
}
//read in string
/*let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Searching for the hidden aussie in {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file"); 
*/
//to lower case, split into a vector
// let s_lower = &s.to_lowercase().as_bytes.to_owned;
//split to graphemes
//match input graphemes to pattern graphemes, returning index vals of input graphemes
//let pat_lower = &pat.to_lowercase.as_bytes().to_owned
//check that each pattern grapheme has a match
/*for i in pat_lower.len() {
    let mut start = 0;
    let end = s_lower.len();
    let mut loc = s_lower[start..end].find().unwrap();

}
*/
//capitalize all graphemes that match the pattern
//create correct ranges of sting indicies, based on index vals above
//set colours
//recomine and print
fn find_aussies(s: &String) -> Vec<String> {
    let s_lower = &s.to_lowercase();
    let pattern:&str = "gday mate";
    //let inbytes = s_lower.as_bytes();
    //let patbytes = pattern.as_bytes();
    let mut locchecks:Vec<Option<usize>> = Vec::new();
    let mut snippets:Vec<String> = Vec::new();
    let mut start:usize = 0;
    let end:usize = s_lower.len();
    for i in pattern.chars() {
        let mut loc:usize = s_lower[start..end].find(i).unwrap();
        locchecks.push(s[start..end].find(i));
        snippets.push(s_lower[start..(loc-1)].to_string());
        snippets.push(s_lower.chars().nth(loc).unwrap().to_uppercase().to_string());
        /*if 2 < pattern.chars().find(i).unwrap() {
            start = loc + 1;
        } else {
            snippets.push(s_lower[loc+1..end].to_string());
        }*/
        start = loc + 1;
    }
    snippets.push(s_lower[start..end].to_string());
    let notfound = check_aussies(locchecks);
    if notfound {
        println!("No hidden aussies found!")
    } else {
        return snippets;
    }
    snippets
}
/*fn find_aussies(s: &String) -> Vec<T> {
    let schars = s.chars();
    let pattern:&str = "gday mate";
    let mut index:i32 = 0;
    let locs:Vec<T> = Vec<T>::new();
    let mut currentchar = schars.next().unwrap();
    for i in 0..(pattern.chars().count()-1) {
        let mut currentchar = schars.next().unwrap();
        if currentchar == pattern.chars().nth(i).unwrap() {
            locs.push((currentchar, index));
            index = index + 1;
            currentchar = schars.next().unwrap();
        } else {
            currentchar = schars.next().unwrap();
        }
        locs
        }
        locs
    }
*/
//fn aussie_convert();
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Searching for the hidden aussie in {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let matching = find_aussies(&contents);
    //let my_check = check_aussies(matching);
    /*if my_check {
        println!("No hidden aussies found!")
    } else {
        aussie_convert
    }*/
    println!("{:?}",matching);
}