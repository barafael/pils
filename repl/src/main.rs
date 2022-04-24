use pils::{help_text, process};
use rustyline::{error::ReadlineError, Editor};

fn main() -> anyhow::Result<()> {
    let mut prompt = Editor::<()>::new();
    loop {
        match prompt.readline("pils >> ") {
            Ok(line) => {
                if line == "exit" || line == "quit" {
                    break;
                }
                prompt.add_history_entry(&line);

                if line == "help" {
                    println!("{}", help_text());
                    continue;
                }

                match process(line.as_str()) {
                    Ok(v) => println!("{}", v),
                    Err(e) => println!("error: {:#?}", e),
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
