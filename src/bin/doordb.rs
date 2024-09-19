use anyhow::Result;
use anyhow::Context;
use clap::Parser;
use clap::Subcommand;
use doordb::Client;
use doordb::Method;

#[derive(Parser)]
#[command(name = "doordb", about = "Fast key-value store for local processes")]
struct Cli {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand)]
enum Command {
    Counter {
        #[command(subcommand)]
        action: CounterAction
    },
    Delete {
        key: String
    },
    Stat {
        key: String
    },
    Text {
        #[command(subcommand)]
        action: TextAction
    },
}

#[derive(Subcommand)]
enum CounterAction {
    Read {
        key: String
    },
    Increment {
        key: String
    },
    Create {
        key: String
    },
    Delete {
        key: String
    },
}

impl CounterAction {
    fn main(&self, client: Client) -> Result<()> {
        match self {
            Self::Create { key } => Self::create(key, client),
            Self::Delete { key } => Self::delete(key, client),
            Self::Increment { key } => Self::increment(key, client),
            Self::Read { key } => Self::read(key, client),
        }
    }

    fn create(key: &str, client: Client) -> Result<()> {
        let method = Method::Create;
        let value = client.submit_query(method, key)?;
        println!("{}", value);

        Ok(())
    }

    fn delete(key: &str, client: Client) -> Result<()> {
        let method = Method::Delete;
        let value = client.submit_query(method, key)?;
        println!("{}", value);

        Ok(())
    }

    fn read(key: &str, client: Client) -> Result<()> {
        let method = Method::Get;
        let value = client.submit_query(method, key)?;
        println!("{}", value);

        Ok(())
    }

    fn increment(key: &str, client: Client) -> Result<()> {
        let method = Method::Increment;
        let value = client.submit_query(method, key)?;
        println!("{}", value);

        Ok(())
    }
}

#[derive(Subcommand)]
enum TextAction {
    Read {
        key: String
    },
    Write {
        key: String,
        value: String
    }
}

impl TextAction {
    fn main(&self, client: Client) -> Result<()> {
        match self {
            Self::Read { key } => Self::read(key, client),
            Self::Write { key, value } => Self::write(key, value, client),
        }
    }

    fn read(_key: &str, _client: Client) -> Result<()> {
        Err(anyhow::anyhow!("Not implemented")).context("Can't read text yet")
    }

    fn write(_key: &str, _value: &str, _client: Client) -> Result<()> {
        Err(anyhow::anyhow!("Not implemented")).context("Can't write text yet")
    }
}

struct DefaultAction {}

impl DefaultAction {
    fn delete(_key: &str, _client: Client) -> Result<()> {
        Err(anyhow::anyhow!("Not implemented")).context("Can't delete arbitrary keys yet")
    }

    fn stat(_key: &str, _client: Client) -> Result<()> {
        Err(anyhow::anyhow!("Not implemented")).context("Can't stat arbitrary keys yet")
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let client = Client::new();

    match &cli.command {
        Command::Counter { action } => action.main(client),
        Command::Delete { key } => DefaultAction::delete(key, client),
        Command::Stat { key } => DefaultAction::stat(key, client),
        Command::Text { action } => action.main(client),
    }
}
