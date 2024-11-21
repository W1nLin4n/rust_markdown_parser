mod parser;
use std::{fs, io::{self, Read}, process::Output};

use anyhow::Result;
use clap::{Arg, Command};
use parser::{markdown_file_to_html, markdown_to_html};

fn main() -> Result<()> {
    let matches = Command::new("Markdown Parser")
    .author("Artem Hrechka, grechkaartema@gmail.com")
    .about("Converts markdown into HTML.")
    .subcommand(
        Command::new("help")
        .about("Displays help information for this program")
    )
    .subcommand(
        Command::new("credits")
        .about("Displays credists information for this program")
    )
    .subcommand(
        Command::new("parse")
        .about("Parses markdown into HTML")
        .arg(
            Arg::new("input")
            .help("Input file(default: standard input)")
            .short('i')
            .long("input")
            .required(false)
            .default_value("-")
        )
        .arg(
            Arg::new("output")
            .help("Output file(default: standard output)")
            .short('o')
            .long("output")
            .required(false)
            .default_value("-")
        )
    ).get_matches();
    
    match matches.subcommand() {
        Some(("help", _)) => {
            print!(
                "Commands:\n\
                help: This message\n\
                credits: Info about creator\n\
                parse: Parse markdown into HTML"
            )
        }
        Some(("credits", _)) => {
            print!("Artem Hrechka, grechkaartema@gmail.com")
        }
        Some(("parse", args)) => {
            let input_path = args.get_one::<String>("input").unwrap();
            let output_path = args.get_one::<String>("output").unwrap();

            let mut html = String::new();
            if input_path == "-" {
                let mut markdown = String::new();
                io::stdin()
                    .read_to_string(&mut markdown)?;
                html = markdown_to_html(markdown)?;
            } else {
                html = markdown_file_to_html(input_path.to_string())?;
            }
            if output_path == "-" {
                println!("{}", html);
            } else {
                fs::write(output_path, html)?;
            }
        }
        _ => print!("Invalid command. To get a list of commands use `help`")
    }
    Ok(())
}
