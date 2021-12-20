mod lol;
mod window;

use rand::Rng;
use crate::window::cat_fax_window;
use crate::lol::{Control, print_lines_lol};

#[derive(Debug, Clone, Copy)]
struct TerminalArgs {
    color: bool,
    raw: bool,
}

/// parse the command line arguments into a struct
fn parse_args(args: Vec<String>) -> Option<TerminalArgs> {
    if args.contains(&"-w".to_string()) || args.contains(&"--window".to_string()) {
        return None
    }
    let mut targs = TerminalArgs {
        color: false,
        raw: false,
    };
    for x in 0..args.len() {
        let s = args.get(x).unwrap();
        match s.as_str() {
            "-c" | "--color" => {
                if targs.raw {
                    println!("Failed to set color mode: raw mode can not be colored!");
                    continue;
                }
                targs.color = true;
            }
            "-r" | "--raw" => {
                targs.raw = true;
                targs.color = false; // raw mode is always colorless
            }
            "-h" | "--help" | "-?" => {
                println!("+ Cat-Fax Help:\n\
                |  `-h` or `--help`:   displays this message\n\
                |  `-w` or `--window`: displays the cat fact in a custom window\n\
                |  `-c` or `--color`:  displays the cat fact in rainbow\n\
                |  `-r` or `--raw`:    displays the cat fact raw, no special formatting\n\
                +");
                std::process::exit(0);
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
    } else if fact.is_empty() {
        format!("\
    ┌{}┐\n\
    │  /\\_/\\{spaces} │\n\
    │ ( o.o )   │\n\
    │  > ^ <{spaces} │\n\
    └{}┘\n\
    ", fax_num,
                "─".repeat(11),
                spaces = " ".repeat(fact.len() + 3))
    } else if fax_num.len() > fact.len() + 10 {
        format!("\
    ┌{}┐\n\
    │  /\\_/\\{spaces}│\n\
    │ ( o.o ) {}{} │\n\
    │  > ^ <{spaces}│\n\
    └{}┘\n\
    ", fax_num, " ".repeat(fax_num.len() - fact.len()), fact,
                "─".repeat(fact.len() + 10),
                spaces = " ".repeat(fact.len() + 3))
    } else { // otherwise, format it
        let line_size = (fact.len() + 10) - fax_num.len();
        let left_line_len = "─".repeat(line_size / 2);
        let right_line_len = "─".repeat(line_size / 2);
        let mut fact_line = format!("{}{}{}",
                                    left_line_len,
                                    fax_num,
                                    right_line_len);
        let fl_len = (left_line_len.len() / 3) + (left_line_len.len() / 3) + fax_num.len();
        if fl_len < (fact.len() + 10) {
            fact_line += "─";
        }
        format!("┌{}┐\n\
    │  /\\_/\\{spaces}│\n\
    │ ( o.o ) {} │\n\
    │  > ^ <{spaces}│\n\
    └{}┘\n\
    ", fact_line, fact,
                "─".repeat(fact.len() + 10),
                spaces = " ".repeat(fact.len() + 3))};

    if targs.color {
        // settings for the lolcat code
        let seed = rand::thread_rng().gen_range(0.0..255.0);
        let mut lol_ctrl = Control {
            seed,
            spread: 3.0,
            frequency: 0.1,
            background_mode: false,
            dialup_mode: false,
            print_color: true
        };
        // print the fact in color
        print_lines_lol(ascii.split("\n"), &mut lol_ctrl);
    } else {
        // print the fact in plane ascii
        println!("{}", ascii);
    }
}

#[cfg(test)]
mod tests {
    use crate::{cat_fax_term, TerminalArgs};

    #[test]
    fn term_sweep() {
        let mut targs = TerminalArgs {
            color: false,
            raw: false,
        };
        let fax = include_str!("../assets/fax.txt").split("\n").collect::<Vec<_>>();
        for x in 0..fax.len() {
            cat_fax_term(targs, fax.get(x).unwrap(), x)
        }
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
    let fax_index = rand::thread_rng().gen_range(0..fax.len());
    let fact = *fax.get(fax_index).unwrap();
    drop(fax);

    // display the cat fax to where it needs to go
    if targs.is_some() {
        cat_fax_term(targs.unwrap(), fact, fax_index + 1);
    } else {
        cat_fax_window(fact, fax_index + 1);
    }
}
