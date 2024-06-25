use std::fs;

use clap::{Parser, Subcommand, ValueEnum};
use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    admin_url: String,
    pwhash: String,
}
impl Config {
    pub fn read_config() -> Self {
        let mut path = dirs::home_dir().expect("Could not retrieve home directory");
        path.push(".pirc");

        // Read the file into a string
        let config_content = fs::read_to_string(path).expect("Could not read the config file");

        // Deserialize the string into Config
        toml::from_str(&config_content).expect("Could not deserialize the config file")
    }
}
fn print_version() -> &'static str {
    Box::leak(format!("v{}", env!("CARGO_PKG_VERSION")).into())
}

#[derive(Debug, ValueEnum, Clone)]
enum TempUnit {
    C,
    F,
    K,
}

impl std::fmt::Display for TempUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[derive(Debug, Parser)]
#[command(name = "pictl")]
#[command(version = print_version(), about = "Pihole remote cli", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    // SetTempUnit {
    //     unit: TempUnit,
    // },
    Version,
    Summary,
    Enable,
    Disable {
        #[arg(
            short,
            long,
            default_value = "false",
            help = "Required when disabling blocking permanently"
        )]
        force: bool,
        #[arg(help = "Number of seconds to disable blocking")]
        seconds: Option<u32>,
    },
    List(ListCommand),
    Db,
}

#[derive(Debug, ValueEnum, Clone)]
enum List {
    Black,
    White,
    RegexBlack,
    RegexWhite,
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self).to_lowercase())
    }
}

#[derive(Debug, Subcommand)]
enum ListAction {
    Add,
    Remove,
    Show,
}

#[derive(Debug, clap::Args)]
struct ListCommand {
    #[arg()]
    list: List,
    #[command(subcommand)]
    action: ListAction,
}

fn main() -> Result<(), reqwest::Error> {
    let cli = Cli::parse();

    let config = Config::read_config();
    let url = config.admin_url;
    let auth = config.pwhash;

    let url = match cli.command {
        // Commands::SetTempUnit { unit } => format!("{url}?setTempUnit={unit}&auth={auth}"),
        Commands::Version => format!("{url}?version&auth={auth}"),
        Commands::Summary => format!("{url}?summaryRaw&auth={auth}"),
        Commands::Enable => format!("{url}?enable&auth={auth}"),
        Commands::Disable { force, seconds } => {
            let dur = match (seconds, force) {
                (Some(dur), _) => format!("={dur}"),
                (None, false) => {
                    eprintln!("Omitting a duration permanently disables blocking. If this was intentional rerun the command with `-f/--force`");
                    std::process::exit(1);
                }
                (None, true) => "".to_owned(),
            };
            format!("{url}?disable{dur}&auth={auth}")
        }
        Commands::List(cmd) => {
            let list = cmd.list;
            match cmd.action {
                ListAction::Show => format!("{url}?list={list}&action=get_domain&auth={auth}"),
                _ => todo!(),
            }
        }
        Commands::Db => todo!(),
    };

    let response = get(url)?.text()?;

    println!("{}", response);

    Ok(())
}
