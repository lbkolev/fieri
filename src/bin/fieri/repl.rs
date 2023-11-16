use std::env;
use std::path::PathBuf;

use clap::Parser;

use fieri::{
    chat::{chat, ChatMessageBuilder, ChatParamBuilder},
    Client,
};
use rustyline::{error::ReadlineError, DefaultEditor};

pub fn run_console(file: &PathBuf) -> rustyline::Result<()> {
    let mut rl = DefaultEditor::new()?;
    let _ = rl.load_history(file).is_err();

    println!("{}", crate::version::LONG_VERSION);
    loop {
        let readline = rl.readline(format!("{}>> ", clap::crate_name!()).as_str());
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                todo!("Implement REPL");
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

    if rl.save_history(file).is_err() {
        println!("Could not save history");
    };
    Ok(())
}
