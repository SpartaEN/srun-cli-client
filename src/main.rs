mod cli;
mod config;
mod srun;

use cli::process_cli;
use config::OutputFormat;
use srun::client::SRUNClient;

fn main() {
    let app_config = process_cli();

    let client = SRUNClient::from_app_config(&app_config);

    match app_config.command.unwrap().as_str() {
        "query" => {
            let resp = client.query();
            let r = match resp {
                Ok(r) => r,
                Err(e) => {
                    panic!("Failed to query: {}", e);
                }
            };
            match app_config.output {
                OutputFormat::Plain => {
                    print!("{}", r.to_string());
                }
                OutputFormat::Json => {
                    print!("{}", r.to_json());
                }
            }
        }
        "login" => {
            // check whether username and password are provided
            if app_config.username.is_none() || app_config.password.is_none() {
                panic!("Username and password must be provided");
            }

            // In some rare cases, http hijacking (redirection) must be triggered once to kick off BAS response
            if app_config.redirect {
                let resp = client.access_redirect_host();
                let status = match resp {
                    Ok(status) => status,
                    Err(e) => {
                        panic!("Failed to access captive portal: {}", e);
                    }
                };

                if status {
                    match app_config.output {
                        OutputFormat::Plain => {
                            println!(
                                "Portal testing returned 204 code, which indicates you're online."
                            );
                        }
                        _ => {}
                    }
                }
            }

            let resp = client.query();
            let qr = match resp {
                Ok(qr) => qr,
                Err(e) => {
                    panic!("Failed to query: {}", e);
                }
            };

            let resp = client.get_challenge(&qr.online_ip);
            let cr = match resp {
                Ok(cr) => cr,
                Err(e) => {
                    panic!("Failed to get challenge: {}", e);
                }
            };

            let resp = client.get_ac_id();
            let ac_id = match resp {
                Ok(ac_id) => ac_id,
                Err(e) => {
                    panic!("Failed to get ac_id: {}", e);
                }
            };

            let resp = client.login(&cr.challenge, &qr.online_ip, &ac_id);
            let lr = match resp {
                Ok(lr) => lr,
                Err(e) => {
                    panic!("Failed to login: {}", e);
                }
            };

            match app_config.output {
                OutputFormat::Plain => {
                    print!("{}", lr.to_string());
                }
                OutputFormat::Json => {
                    print!("{}", lr.to_json());
                }
            }
        }
        "logout" => {
            if app_config.username.is_none() {
                panic!("Username must be provided");
            }
            let resp = client.query();
            let qr = match resp {
                Ok(qr) => qr,
                Err(e) => {
                    panic!("Failed to query: {}", e);
                }
            };

            let resp = client.get_ac_id();
            let ac_id = match resp {
                Ok(ac_id) => ac_id,
                Err(e) => {
                    panic!("Failed to get ac_id: {}", e);
                }
            };

            let resp = client.logout(&qr.online_ip, &ac_id);
            let lr = match resp {
                Ok(lr) => lr,
                Err(e) => {
                    panic!("Failed to logout: {}", e);
                }
            };

            match app_config.output {
                OutputFormat::Plain => {
                    print!("{}", lr.to_string());
                }
                OutputFormat::Json => {
                    print!("{}", lr.to_json());
                }
            }
        }
        _ => {}
    }
}
