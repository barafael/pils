use anyhow::Context;
use rustyline::{error::ReadlineError, Editor};
use slipstream::process;

fn main() -> anyhow::Result<()> {
    let mut prompt = Editor::<()>::new();
    loop {
        match prompt.readline("slipstream >> ") {
            Ok(line) => {
                if line == "exit" || line == "quit" {
                    break;
                }
                prompt.add_history_entry(&line);

                let result = process(line.as_str()).context("Failed to process input")?;
                println!("{}", result);
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
