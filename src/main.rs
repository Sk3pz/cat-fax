mod lol;
mod window;

use rand::Rng;
use crate::window::cat_fax_window;
use crate::lol::{Control, print_chars_lol};

#[derive(Debug, Clone)]
struct TerminalArgs {
    color: bool,
    raw: bool,
}

/// parse the command line arguments into a struct
fn parse_args(args: Vec<String>) -> Option<TerminalArgs> {
    if args.contains(&"-w".to_string()) {
        return None
    }
    let mut targs = TerminalArgs {
        color: true,
        raw: false,
    };
    for x in 0..args.len() {
        let s = args.get(x).unwrap();
        match s.as_str() {
            "-c" | "--color" => {
                if x + 1 < args.len() {
                    let next = args.get(x + 1).unwrap();
                    if next == "false" {
                        targs.color = false;
                        continue;
                    }
                }
                if targs.raw {
                    println!("Failed to set color mode: raw mode can not be colored!");
                    continue;
                }
                targs.color = true;
            }
            "-r" | "--raw" => {
                if x + 1 < args.len() {
                    let next = args.get(x + 1).unwrap();
                    if next == "false" {
                        targs.raw = false;
                        continue;
                    }
                }
                targs.raw = true;
                targs.color = false; // raw mode is always colorless
            }
            _ => {}
        }
    }
    Some(targs)
}

/// prints the cat fact formatted to the terminal
fn cat_fax_term(targs: TerminalArgs, fact: &str, num: usize) {
    let fax_num = format!("Cat Fax #{}", num);
    // if its raw, then just print the fact
    let ascii = if targs.raw {
        format!("{}: {}", fax_num, fact)
    } else { // otherwise, format it
        format!("+{tline}{}{tline}{}+\n\
    |  /\\_/\\{spaces}|\n\
    | ( o.o ) {} |\n\
    |  > ^ <{spaces}|\n\
    +{}+\n\
    ",
                fax_num,
                if fax_num.len() % 2 != 0 { "-" }
                else { "" },
                fact, "-".repeat(fact.len() + 10),
                tline = "-".repeat(((fact.len() + 10) - fax_num.len()) / 2),
                spaces = " ".repeat(fact.len() + 3))};

    if targs.color {
        // settings for the lolcat code
        let mut lol_ctrl = Control {
            seed: 0.0,
            spread: 3.0,
            frequency: 0.1,
            background_mode: false,
            dialup_mode: false,
            print_color: targs.color
        };
        // print the fact in color
        print_chars_lol(ascii.chars(), &mut lol_ctrl, false);
    } else {
        // print the fact in plane ascii
        println!("{}", ascii);
    }
}

fn main() {
    // get the command line arguments
    let mut args = std::env::args().collect::<Vec<_>>();
    args.remove(0); // remove the program name
    let targs = parse_args(args);
    // get the cat fax from the file
    let fax = include_str!("../assets/fax.txt").split("\n").collect::<Vec<_>>();

    // randomly select a cat fax
    let fax_index = rand::thread_rng().gen_range(0, fax.len());
    let fact = *fax.get(fax_index).unwrap();
    drop(fax);

    // display the cat fax to where it needs to go
    if targs.is_some() {
        cat_fax_term(targs.unwrap(), fact, fax_index + 1);
    } else {
        cat_fax_window(fact, fax_index + 1);
    }
}
