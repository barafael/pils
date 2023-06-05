use pils::{help::HELP_TEXT, process};
use rustyline::{error::ReadlineError, Editor};

// TODO just one file
fn main() -> anyhow::Result<()> {
    let mut prompt = Editor::<()>::new().expect("Failed to create prompt");
    loop {
        match prompt.readline("pils >> ") {
            Ok(line) => {
                if line == "exit" || line == "quit" {
                    break;
                }
                prompt.add_history_entry(&line);

                if line == "help" {
                    println!("{}", HELP_TEXT);
                    continue;
                }

                match process(line.as_str()) {
                    Ok(v) => println!("{}", v),
                    Err(e) => println!("error: {}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
