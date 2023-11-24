mod cli;
mod config;
mod srun;

use std::error::Error;

use cli::process_cli;
use config::OutputFormat;
use json::object;
use srun::client::SRUNClient;
use srun::response::SRUNResponse;

fn main() {
    let app_config = process_cli();

    let client = SRUNClient::from_app_config(&app_config);

    let mut resp: Option<Box<dyn SRUNResponse>> = None;
    let mut err: Option<Box<dyn Error>> = None;

    match app_config.command.unwrap().as_str() {
        "query" => {
            match query(&client) {
                Ok(r) => {
                    resp = Some(r);
                }
                Err(e) => {
                    err = Some(e);
                }
            };
        }
        "login" => {
            // check whether username and password are provided
            if app_config.username.is_none() || app_config.password.is_none() {
                println!("Username and password must be provided");
                std::process::exit(1);
            }

            match login(
                &client,
                app_config.redirect,
                matches!(app_config.output, OutputFormat::Plain),
            ) {
                Ok(r) => {
                    resp = Some(r);
                }
                Err(e) => {
                    err = Some(e);
                }
            };
        }
        "logout" => {
            if app_config.username.is_none() {
                println!("Username must be provided");
                std::process::exit(1);
            }

            match logout(&client) {
                Ok(r) => {
                    resp = Some(r);
                }
                Err(e) => {
                    err = Some(e);
                }
            };
        }
        _ => {}
    }

    match app_config.output {
        OutputFormat::Plain => {
            if err.is_some() {
                println!("{}", err.unwrap());
                std::process::exit(1);
            }
            if resp.is_some() {
                print!("{}", resp.unwrap().to_string());
            }
        }
        OutputFormat::Json => {
            if err.is_some() {
                let error_obj = object! {
                "error" => err.unwrap().to_string()};
                print!("{}", error_obj.dump());
                std::process::exit(1);
            }
            if resp.is_some() {
                print!("{}", resp.unwrap().to_json());
            }
        }
    }
}

fn query(client: &SRUNClient) -> Result<Box<dyn SRUNResponse>, Box<dyn Error>> {
    let r = client.query()?;
    Ok(Box::new(r))
}

fn login(
    client: &SRUNClient,
    redirect: bool,
    output_warning: bool,
) -> Result<Box<dyn SRUNResponse>, Box<dyn Error>> {
    // In some rare cases, http hijacking (redirection) must be triggered once to kick off BAS response
    if redirect {
        let status = client.access_redirect_host()?;

        if status && output_warning {
            println!("Portal testing returned 204 code, which indicates you're online.");
        }
    }

    let r = client.query()?;
    let cr = client.get_challenge(&r.online_ip)?;
    let ac_id = client.get_ac_id()?;
    let lr = client.login(&cr.challenge, &r.online_ip, &ac_id)?;
    Ok(Box::new(lr))
}

fn logout(client: &SRUNClient) -> Result<Box<dyn SRUNResponse>, Box<dyn Error>> {
    let r = client.query()?;
    let ac_id = client.get_ac_id()?;
    let lr = client.logout(&r.online_ip, &ac_id)?;
    Ok(Box::new(lr))
}
