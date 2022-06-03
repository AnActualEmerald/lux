mod api;
mod config;

use std::fs;

use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use riven::consts::PlatformRoute;
use std::str::FromStr;

//Clap config

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    ///Summoner name to look up
    summoner: Option<String>,

    //Info to look up
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ///Get the summoner's profile
    Profile {},
    ///Get the summoner's champion mastery
    Mastery {},
    ///Get the summoner's match history
    #[clap(name = "history")]
    MatchHistory {},
    ///Show a graph of the summoner's LP
    #[clap(name = "lp")]
    LPHistory {},
    ///Manage the configuration
    Config {
        #[clap(long, short)]
        summoner: Option<String>,
        #[clap(parse(try_from_str), long, short)]
        region: Option<WrappedRoute>,
    },
}

//End clap config

struct WrappedRoute(PlatformRoute);

impl FromStr for WrappedRoute {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NA" => Ok(WrappedRoute(PlatformRoute::NA1)),
            "EUNE" => Ok(WrappedRoute(PlatformRoute::EUN1)),
            "EUW" => Ok(WrappedRoute(PlatformRoute::EUW1)),
            "JP" => Ok(WrappedRoute(PlatformRoute::JP1)),
            "LAN" => Ok(WrappedRoute(PlatformRoute::LA1)),
            "LAS" => Ok(WrappedRoute(PlatformRoute::LA2)),
            "OCE" => Ok(WrappedRoute(PlatformRoute::OC1)),
            "PBE" => Ok(WrappedRoute(PlatformRoute::PBE1)),
            "RU" => Ok(WrappedRoute(PlatformRoute::RU)),
            "TR" => Ok(WrappedRoute(PlatformRoute::TR1)),
            "KR" => Ok(WrappedRoute(PlatformRoute::KR)),
            _ => Err("Unrecognized region string"),
        }
    }
}

fn main() {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    let dirs = ProjectDirs::from("me", "greenboi", "lux").unwrap();
    ensure_dirs(&dirs);

    dispatch_commands(&cli, &dirs);
}

fn ensure_dirs(dirs: &ProjectDirs) {
    fs::create_dir_all(dirs.config_dir()).unwrap();
}

fn dispatch_commands(cli: &Cli, dirs: &ProjectDirs) {
    let mut config = config::load_config(dirs.config_dir());
    //  if config.summoner() == "" && cli.summoner.is_none() {
    //      println!(
    //          "Supply a summoner name or set the default summmoner in {}",
    //          dirs.config_dir().join("config.ron").display()
    //      );
    //      return;
    //  }

    let summoner = if let Some(s) = &cli.summoner {
        s
    } else {
        config.summoner()
    };

    match &cli.command {
        Commands::Profile {} => todo!(),
        Commands::LPHistory {} => todo!(),
        Commands::Mastery {} => todo!(),
        Commands::MatchHistory {} => todo!(),
        Commands::Config {
            summoner: s,
            region: r,
        } => {
            if let Some(s) = s {
                config.set_summoner(&s);
                println!("Default summoner set to {}", s);
            }

            if let Some(r) = r {
                config.set_region(r.0);
                println!("Default region set to {}", r.0.as_region_str());
            }
        }
    }
}
