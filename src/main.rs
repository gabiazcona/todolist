use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{self, BufRead};
use std::io::prelude::*;
use std::str::FromStr;



const FILENAME : &str = "todolist";
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
            //let c = line.chars().next().unwrap();
            let n: u32 = FromStr::from_str(number).unwrap();
            (n+1).to_string()
            },
        Some(Err(_)) => panic!("error with file"),
    };
    // write item with number+1 in todolist
    writeln!(&mut file, "{} {}", last_number, item)?;
    // save file
    Ok(())
}


// type Result<T> = std::result::Result<T, FileError>;
#[derive(Debug, Clone)]
struct FileError;
impl Error for FileError {}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error operating on file")
    }
}

// borrar item
// marcar item como hecho