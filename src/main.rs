mod lol;

use std::os;
use better_term::rainbowify;
use rand::Rng;
use crate::lol::{Control, print_chars_lol};

#[derive(Debug, Clone)]
struct TerminalArgs {
    color: bool,
    raw: bool,
}

fn parse_args(args: Vec<String>) -> Option<TerminalArgs> {
    if !args.contains(&"-t".to_string()) {
        return None
    }
    let mut targs = TerminalArgs {
        color: true,
        raw: false,
    };
    for x in 0..args.len() {
        let a = args.get(x).unwrap();
        match a.as_str() {
            "-c" | "--color" => {
                targs.color = true;
            }
            "-r" | "--raw" => {
                targs.raw = true;
            }
            "-t" => {}
            _ => {
                println!("unrecognized argument: {}", a);
                std::process::exit(1);
            }
        }
    }
    Some(targs)
}

/// prints the cat fact formatted to the terminal
fn cat_fax_term(targs: TerminalArgs, fact: &str) {



    let ascii = if targs.raw {
        format!("{}", fact)
    } else {
        format!("+{line}+\n\
    |  /\\_/\\{spaces}|\n\
    | ( o.o ) {} |\n\
    |  > ^ <{spaces}|\n\
    +{line}+\n\
    ", fact,
                spaces = " ".repeat(fact.len() + 3),
                line = "-".repeat(fact.len() + 10))
    };

    let mut lol_ctrl = Control {
        seed: 0.0,
        spread: 3.0,
        frequency: 0.1,
        background_mode: false,
        dialup_mode: false,
        print_color: targs.color
    };
    print_chars_lol(ascii.chars(), &mut lol_ctrl, false);
}

fn cat_fax_window(fact: &str) {
    println!("This is not yet implemented!");
}

fn main() {
    // get the command line arguments
    let mut args = std::env::args().collect::<Vec<_>>();
    args.remove(0); // remove the program name
    let targs = parse_args(args);
    // get the cat fax from the file
    let fax = include_str!("fax.txt").split("\n").collect::<Vec<_>>();

    // randomly select a cat fax
    let fax_index = rand::thread_rng().gen_range(0, fax.len());
    let fact = *fax.get(fax_index).unwrap();
    drop(fax);

    if targs.is_some() {
        cat_fax_term(targs.unwrap(), fact);
    } else {
        cat_fax_window(fact);
    }
}
