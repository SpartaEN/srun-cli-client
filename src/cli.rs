use super::config::{AppConfig, OutputFormat};
use clap::{Arg, ArgAction, Command};

pub fn process_cli() -> AppConfig {
    let matches = Command::new("SRUN Client")
        .version("0.1.0")
        .author("SpartaEN <i@evo.moe>")
        .about("SRUN Commandline Client")
        .arg(
            Arg::new("username")
                .short('u')
                .long("username")
                .help("Username for the client"),
        )
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .help("Password for the client"),
        )
        .arg(
            Arg::new("server")
                .short('s')
                .long("server")
                .help("Server url"),
        )
        .arg(
            Arg::new("redirect")
                .short('r')
                .long("redirect")
                .action(ArgAction::SetTrue)
                .help(
                    "Trigger login by accessing some site (may be required under spefific network)",
                ),
        )
        .arg(
            Arg::new("redirect-host")
                .long("redirect-host")
                .default_value("http://www.google.cn/generate_204")
                .help("Site to trigger redirection"),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Configuration file"),
        )
        .arg(
            Arg::new("interface")
                .short('i')
                .long("interface")
                .help("Network interface to use"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output format"),
        )
        .subcommand(Command::new("query").about("Query account status"))
        .subcommand(Command::new("login").about("Log in to campus network"))
        .subcommand(Command::new("logout").about("Log out from campus network"))
        .get_matches();

    let username = matches.get_one::<String>("username");
    let password = matches.get_one::<String>("password");
    let server = matches.get_one::<String>("server");
    let redirect = matches.get_flag("redirect");
    let redirect_host = matches.get_one::<String>("redirect-host");
    let config_path = matches.get_one::<String>("config");
    let interface = matches.get_one::<String>("interface");
    let output = matches
        .get_one::<String>("output")
        .map(|s| s.to_lowercase());

    let mut app_config: AppConfig;
    match config_path {
        Some(config_path) => {
            app_config = AppConfig::from_file(config_path);
        }
        None => {
            app_config = AppConfig::new();
            app_config.username = username.map(|s| s.clone());
            app_config.password = password.map(|s| s.clone());
            app_config.server = server.map(|s| s.clone());
            app_config.redirect = redirect;
            app_config.redirect_host = redirect_host.map(|s| s.clone());
            app_config.interface = interface.map(|s| s.clone());
            app_config.output = match output.as_deref() {
                Some("json") => OutputFormat::Json,
                _ => OutputFormat::Plain,
            };
        }
    }

    match app_config.server {
        Some(_) => {}
        None => {
            panic!("Server not specified, exiting");
        }
    }

    match matches.subcommand_name() {
        Some(some) => app_config.command = Some(String::from(some)),
        None => {
            panic!("No subcommand specified, exiting");
        }
    }

    app_config
}
