use better_term::{Color, flush_styles};
use better_term::fancy::gradient;
use rand::{Rng, thread_rng};

const FAX_INCLUDE: &str = include_str!("../assets/fax.txt");

const MAX_LINE_LENGTH: usize = 30;

fn print_gradient<S: Into<String>>(text: S) {
    let text = text.into();
    // handle multiline as well
    let split = text.split('\n').collect::<Vec<&str>>();
    let length = split.iter().map(|s| s.len()).max().unwrap();

    let rs = thread_rng().gen_range(0..255);
    let gs = thread_rng().gen_range(0..255);
    let bs = thread_rng().gen_range(0..255);

    let re = thread_rng().gen_range(0..255);
    let ge = thread_rng().gen_range(0..255);
    let be = thread_rng().gen_range(0..255);
    let gradient = gradient((rs,gs,bs), (re,ge,be), length);

    for (i, line) in text.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let color_index = (i + j) % gradient.len();
            print!("{}{}", gradient[color_index], c);
        }
        println!();
    }
}

fn word_wrap(input: &str, max_size: usize) -> Vec<String> {
    if input.len() <= max_size {
        return vec![input.to_string()];
    }

    let mut result = Vec::new();
    let mut current_line = String::new();
    let mut current_size = 0;

    for word in input.split_whitespace() {
        if current_size + word.len() < max_size {
            // Add word to the current line
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
            current_size += word.len() + 1;
        } else {
            // Start a new line
            result.push(current_line.clone());
            current_line = word.to_string();
            current_size = word.len();
        }
    }

    if !current_line.is_empty() {
        result.push(current_line);
    }

    result
}

struct TerminalArgs {
    color: bool,
    raw: bool,
    num: Option<usize>,
}

fn parse_args(args: Vec<String>) -> TerminalArgs {
    let mut terminal_args = TerminalArgs {
        color: false,
        raw: false,
        num: None,
    };
    let mut expect_num = false;
    for x in 0..args.len() {
        let s = args.get(x).unwrap();

        if expect_num {
            // check if s is a valid number and parse to a u16
            match s.parse::<usize>() {
                Ok(num) => {
                    terminal_args.num = Some(num);
                    expect_num = false;
                }
                Err(_) => {
                    println!("Expected a number after -n or --num!");
                    std::process::exit(1);
                }
            }
            continue;
        }

        match s.as_str() {
            "-c" | "--color" => {
                if terminal_args.raw {
                    println!("Failed to set color mode: raw mode can not be colored!");
                    continue;
                }
                terminal_args.color = true;
            }
            "-r" | "--raw" => {
                terminal_args.raw = true;
                terminal_args.color = false; // raw mode is always colorless
            }
            "-n" | "--num" | "--number" => {
                expect_num = true;
            }
            "-h" | "--help" | "-?" => {
                let version = env!("CARGO_PKG_VERSION");
                println!("{pri}┌─ {hi}Cat-Fax Help: {pri}───  ───  ─   ──  ───  ────    ─    ──   ───  ────   ─ ──{pri}─┐\n\
                {pri}  {sec}`-h` or `--help`    {pri}->{sec} displays this message                             {pri}│\n\
                {pri}│ {sec}`-c` or `--color`   {pri}->{sec} displays the cat fact in a gradient\n\
                {pri}  {sec}`-n #` or `--num #` {pri}->{sec} displays a specific cat fact                      {pri}│\n\
                {pri}│ {sec}`-r` or `--raw`     {pri}->{sec} displays the cat fact raw, no special formatting  {pri} \n\
                {pri}└── ──  ──  ─   ──  ───  ───    ─    ──   ──  ────   ─ ─── {thr}version {version} {pri}──┘",
                         pri=Color::Cyan, sec=Color::White, thr=Color::BrightBlack, hi=Color::BrightCyan);
                flush_styles();
                std::process::exit(0);
            }
            _ => {}
        }
    }

    if expect_num {
        println!("Expected a number after -n or --num!");
        std::process::exit(1);
    }

    terminal_args
}

fn main() {
    let arguments = std::env::args().collect::<Vec<String>>();
    let terminal_args = parse_args(arguments);

    let facts = FAX_INCLUDE.split('\n').map(|s| s.to_string()).collect::<Vec<String>>();

    // get a random fact from the list of facts
    let fact_num = if let Some(n) = terminal_args.num {
        if n > facts.len() {
            println!("The number you entered is too large! There are only {} facts!", facts.len());
            std::process::exit(1);
        }
        if n == 0 {
            println!("The number you entered is too small! Facts start at 1!");
            std::process::exit(1);
        }
        n - 1
    } else {
        thread_rng().gen_range(0..facts.len())
    };
    let fact = facts[fact_num].clone();

    if terminal_args.raw {
        println!("Cat Fax #{}: {}", fact_num + 1, fact);
        return;
    }

    // handle if the fact is too long
    let fact_vec = word_wrap(fact.as_str(), MAX_LINE_LENGTH);

    let fact_len = fact_vec.iter().map(|s| s.len()).max().unwrap();

    // generate the box
    let spaces = " ".repeat(2 + fact_len);

    let num_display = fact_num + 1;

    let top_line = format!("   ┌─ Cat Fact {}┐\n /\\_/\\ {spaces}│", "─".repeat(fact_len - 6));
    let btm_line = format!("\n   └{} #{} ─┘", "─".repeat(fact_len - 1),
                           if num_display < 10 {
                               format!("0{}", num_display)
                           } else {
                               format!("{}", num_display)
                           });


    let first = fact_vec[0].clone();

    let mut cat_box = format!("{top_line}\
                                \n( o.o ) {}{} │", first, " ".repeat(fact_len - first.len()));

    if fact_vec.len() > 1 {
        for (x, fact) in fact_vec.iter().enumerate().skip(1) {
            if x == 1  {
                cat_box.push_str(&format!("\n > ^ <  {}{} │", fact, " ".repeat(fact_len - fact.len())));
            } else {
                cat_box.push_str(&format!("\n   │    {}{} │", fact, " ".repeat(fact_len - fact.len())));
            }
        }
        cat_box.push_str(&format!("\n   │   {spaces}│"));
    } else {
        cat_box.push_str(&format!("\n > ^ <{spaces} │"))
    }

    cat_box.push_str(btm_line.as_str());

    if terminal_args.color {
        print_gradient(&cat_box);
    } else {
        println!("{}", cat_box);
    }

    flush_styles();
}
