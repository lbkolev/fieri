use std::env;
use std::path::PathBuf;

use clap::Parser;

use fieri::{
    chat::{chat, ChatParam, ChatRole},
    Client,
};
use rustyline::{error::ReadlineError, DefaultEditor};

mod version;

fn history_path() -> PathBuf {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
    path.push(format!(".{}_history", clap::crate_name!()));

    path
}

#[derive(Parser, Debug)]
enum Commands {
    /// Opens a REPL console
    Console,

    Chat {
        #[clap(flatten)]
        param: ChatParam,

        #[clap(short, long, default_value = "user")]
        role: ChatRole,

        #[clap(short, long, default_value = "")]
        name: String,
    },
}

#[derive(Parser, Debug)]
#[clap(author, version, about="OpenAI command-line interface.", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// File to write history to
    /// If not specified, history is by default saved to $HOME/.fieri_history
    #[arg(long, env = "FIERI_HISTORY", default_value = history_path().into_os_string())]
    history_file: PathBuf,
}

fn run_console(file: &PathBuf) -> rustyline::Result<()> {
    let mut rl = DefaultEditor::new()?;
    let _ = rl.load_history(file).is_err();

    loop {
        let readline = rl.readline(format!("{}>> ", clap::crate_name!()).as_str());
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Exiting");
                break;
            }
            Err(err) => {
                println!("{:?}", err);
                break;
            }
        }
    }

    if rl.save_history(file).is_err() {
        println!("Could not save history");
    };
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let client = Client::new().api_key(std::env::var("OPENAI_API_KEY")?);

    match cli.command {
        Commands::Console => run_console(&cli.history_file)?,
        Commands::Chat {
            mut param,
            role,
            name,
        } => {
            println!("{:#?}", param);
            param.messages.iter_mut().for_each(|m| {
                m.role = role;
                m.name = Some(name.clone())
            });
            let param = ChatParam { ..param };
            let resp = chat(&client, &param).await?;
            println!("{:#?}", resp);
        }
    }

    Ok(())
}
