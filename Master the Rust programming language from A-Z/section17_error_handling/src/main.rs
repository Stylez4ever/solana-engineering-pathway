//use std::io::Result;
use std::fs;
use std::io::{self, stdin};
use std::process;

fn main() {

    let write_file = write_to_file();
    match write_file {
        Result::Ok(content) => println!("Successfully wrote to file {}", content),
        Result::Err(error) => {
            println!("The was an error: {error}");
            process::exit(1);
        }
    }

    

}

fn write_to_file() -> io::Result<String>{
    let mut file_name = String::new();
    let mut file_content = String::new();

    println!("What file would you like to write to?");
    stdin().read_line(&mut file_name)?;

    println!("What would you like to write to the file?");
    stdin().read_line(&mut file_content)?;

    fs::write(file_name.trim(), file_content.trim())?;

    Result::Ok(file_content)

}