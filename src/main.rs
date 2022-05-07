use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead};
use std::io::prelude::*;
use std::str::FromStr;



const FILENAME : &str = "todolist";
const TMPFILE : &str = ".todolist.tmp";
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let command = &args[1];
        match Command::from(&command.to_lowercase()) {
            Command::Show => show_list(),
            Command::Add => {
                if args.len() >= 3 {
                    add_item(&args[2].clone())?;
                }
            },
            Command::Delete => {
                if args.len() >= 3 {
                    delete_item(&args[2].clone())?;
                }
            } 
            _ => invalid_command(),
        }
    } else {
        println!("No command received")
    }
    Ok(())
}



enum Command {
    Show, Add, Delete, Invalid
}

impl Command {
    fn from(c: &str) -> Self {
        println!("command: {}\n", c);
        match c {
            "show" => Command::Show,
            "add" => Command::Add,
            "delete" => Command::Delete,
            _ => Command::Invalid
        }
    }
}

fn invalid_command() {
    println!("Invalid command");
}

fn show_list() {
    // open todolist file & show content
    let contents = fs::read_to_string(FILENAME)
        .expect("Something went wrong reading the file");

    println!("To-do:\n{}", contents);
}

fn add_item(item: &str) -> Result<(), Box<dyn Error>> {
    println!("{}", item);
    // open todolist
    //let mut file = fs::File::open(FILENAME)?;
    let mut file = fs::OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(FILENAME)?;
    // read last line
    let lines = io::BufReader::new(&file).lines();
    let lastline = lines.last();
    // get number
    let last_number = match lastline {
        None => "1".to_string(),
        Some(Ok(line)) => {
            let number = line.split(" ").next().unwrap();
            let n: u32 = FromStr::from_str(number)?;
            (n+1).to_string()
            },
        Some(Err(_)) => panic!("error with file"),
    };
    // write item with number+1 in todolist
    writeln!(&mut file, "{} {}", last_number, item)?;
    // save file
    Ok(())
}

fn delete_item(item: &str) -> Result<(), Box<dyn Error>> {
    // open todolist
    let file = fs::OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(FILENAME)?;
    // get line number
    let line_to_delete: usize = FromStr::from_str(item).unwrap();
    // get lines from file
    let lines = io::BufReader::new(&file).lines();
    // open new tmp file
    let mut tmp_file = fs::File::create(TMPFILE)?;
    // iterate over lines
    let mut n = 1;
    for line in lines {
        let line = line?;
        // if this is not the line to be deleted, don't write it 
        if line_to_delete > n {
            writeln!(&mut tmp_file, "{}", line)?;
        } else if line_to_delete < n {
            let mut line = line.split_inclusive(" ");
            let number = line.next().unwrap();
            let mut number: u32 = FromStr::from_str(&number[0..number.len()-1])?;
            number -= 1;
            let line: String = line.collect();
            writeln!(&mut tmp_file, "{} {}", number.to_string(), line)?;
        }
        n += 1;
    }


    // let lines: Vec<Result<String, std::io::Error>> = lines.collect();
    // let mut lines: Vec<String> = lines.iter().map(|l| l.as_ref().unwrap().to_owned()).collect();
    // lines.remove(line_to_delete-1);
    
    // // Write the other lines
    // file.set_len(0)?;
    // for line in lines {
    //     writeln!(&mut file, "{}", line)?;
    // }

    fs::rename(TMPFILE, FILENAME)?;
    Ok(())
}

// marcar item como hecho