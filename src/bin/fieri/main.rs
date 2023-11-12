use std::env;
use std::path::PathBuf;

use clap::Parser;

use fieri::{
    chat::{chat, ChatMessageBuilder, ChatParamBuilder},
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
        /// Silences bar output
        #[clap(short, long)]
        silent: bool,
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
                println!("Line: {}", line);
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Console => run_console(&cli.history_file)?,
        Commands::Chat { silent: _ } => (),
    }

    let client = Client::new().api_key(std::env::var("OPENAI_API_KEY")?);
    let message = ChatMessageBuilder::new("user", "Hello!").build()?;
    let param = ChatParamBuilder::new("gpt-3.5-turbo", vec![message]).build()?;

    let resp = chat(&client, &param).await?;
    // let as_json = serde_json::from_str(&serde_json::to_string(&resp)?)?;
    println!("{:#?}", resp);

    Ok(())
}
