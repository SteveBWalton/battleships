// Attempt to convert the Python battleships solver into Rust.
//@file battleships.rs
//@author Steve Walton

// This allows camelCase variables and functions.
#![allow(non_snake_case)]

mod battleship;

enum CommandLine
{
    Searching,
    GameIndex,
    Indent,
    Start,
    Finish,
}

/// The main entry point for the "battleships" program.
fn main() {
    // Fetch the command line arguments.
    let args: Vec<String> = std::env::args().collect();
    // println!("{:?}", args);

    // Initialise the variables.
    let mut game = battleship::Game::new();
    let mut isShowHelp = false;

    // Decode the command parameters (ignore the program name).
    // for (int parameter = 1; parameter < argc; parameter++)
    let mut status = CommandLine::Searching;
    for parameter in 1..args.len() {
        match status {
            CommandLine::Searching => {
                // println!("Parameter {} is {}", parameter, args[parameter]);

                let firstChar = &args[parameter][..1];
                if firstChar == "-" {
                    let otherChar = &args[parameter][1..];
                    // println!("Switch is {}", otherChar);

                    match otherChar {
                        "h" => {
                            isShowHelp = true;
                        }

                        "g" => {
                            status = CommandLine::GameIndex;
                        }

                        "i" => {
                            status = CommandLine::Indent;
                        }

                        "s" => {
                            status = CommandLine::Start;
                        }

                        "f" => {
                            status = CommandLine::Finish;
                        }

                        _ => {
                            // Unknown arguments.

                        }
                    }
                }
            }

            CommandLine::GameIndex => {
                game.index = args[parameter].parse::<i32>().unwrap();
                status = CommandLine::Searching;
            }

            CommandLine::Indent => {
                game.indent = args[parameter].parse::<i32>().unwrap();
                status = CommandLine::Searching;
            }

            CommandLine::Start => {
                game.start = args[parameter].parse::<f64>().unwrap();
                status = CommandLine::Searching;
            }

            CommandLine::Finish => {
                game.finish = args[parameter].parse::<f64>().unwrap();
                status = CommandLine::Searching;
            }
        }
    }

    if isShowHelp {
        println!("Solver for Logic Battleships.");
        println!("optional arguments:");
        println!("  -h\t\tShow this help message and exit.");
        println!("  -g GAME\tThe index of the game to solve.");
        println!("  -i INDENT\tThe identation for the progress information.");
        println!("  -s START\tThe starting position for the search.");
        println!("  -f FINISH\tThe finish position for the search.");
    }
    else {
        println!("game index is {}, indent is {}, start is {}, finish is {}.", game.index, game.indent, game.start, game.finish);
        println!("Goodbye.");
    }

    // Return success.
    return;
}