// * 1) first make a way to get the command line arguments from terminal 
use std::env;

struct TodoItem{
    name: String,
    completed: char
    
}


fn main() {
    // println!("Hello, world!");
    let mut arguments:Vec<String> = env::args().collect();
    
    // ? also what if you dont provide any command line arguments then you will end up having an error 
    // let mut arguments: = env::args(); in order this to work we are supposed to use collect that will make a collection 
    // for x in arguments.iter() {
        // println!("{}",x);
    // }
    let command = arguments[1].clone();
    // this is not the best practise 
    if command == "get"{
        println!("get is invoked");
    }
}
