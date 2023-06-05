use anyhow::Context;
use pils::{help::HELP_TEXT, process};
use rustyline::{error::ReadlineError, DefaultEditor};

fn main() -> anyhow::Result<()> {
    let mut prompt = DefaultEditor::new().context("Failed to create prompt")?;
    loop {
        match prompt.readline("pils >> ") {
            Ok(line) => {
                if line == "exit" || line == "quit" {
                    break;
                }

                if line == "help" {
                    println!("{HELP_TEXT}");
                    continue;
                }

                prompt.add_history_entry(&line)?;

                println!("{}", process(line.as_str())?);
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
                println!("{}", anyhow::format_err!(err));
                break;
            }
        }
    }
    Ok(())
}
