use super::error::SRUNClientError;
use super::response::SRUNChallengeResponse;
use super::response::SRUNLoginResponse;
use super::response::SRUNLogoutResponse;
use super::response::SRUNQueryResponse;
use super::srbx1::SRBX1;
use hmac::{Hmac, Mac};
use local_ip_address::list_afinet_netifas;
use md5::Md5;
use regex::Regex;
use reqwest;
use reqwest::Url;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::collections::HashSet;
use std::net::IpAddr;

type HmacMd5 = Hmac<Md5>;

#[derive(Debug, Clone)]
pub struct SRUNClientCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct SRUNClientOptions {
    pub redirect_host: String,
    pub interface: Option<String>,
}

#[derive(Debug)]
pub struct SRUNClient {
    pub server: String,
    pub credentials: Option<SRUNClientCredentials>,
    pub options: SRUNClientOptions,
}

impl SRUNClient {
    pub fn query(&self) -> Result<SRUNQueryResponse, Box<dyn std::error::Error>> {
        let client = SRUNClient::get_client(self.options.interface.clone())?;
        let mut u = Url::parse(&self.server)?;
        u = u.join("/cgi-bin/rad_user_info")?;
        let resp = client
            .get(u.as_str())
            .query(&[("callback", "FuckSRUNJsonP")])
            .send()?;
        if resp.status().as_u16() != 200 {
            let status = resp.status().as_u16();
            return Err(Box::new(SRUNClientError {
                message: String::from(format!("Server responded with code {status}")),
            }));
        }
        let content = resp.text()?;

        Ok(SRUNQueryResponse::from_string(SRUNClient::extract_jsonp(
            content,
        )?)?)
    }

    pub fn access_redirect_host(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let client = SRUNClient::get_client(self.options.interface.clone())?;
        let resp = client.get(self.options.redirect_host.clone()).send()?;
        if resp.status().as_u16() == 204 {
            return Ok(true);
        }
        Ok(false)
    }

    pub fn get_challenge(
        &self,
        ip: &String,
    ) -> Result<SRUNChallengeResponse, Box<dyn std::error::Error>> {
        let client = SRUNClient::get_client(self.options.interface.clone())?;
        let username = self.credentials.clone().unwrap().username;
        let mut u = Url::parse(&self.server)?;
        u = u.join("/cgi-bin/get_challenge")?;
        let resp = client
            .get(u.as_str())
            .query(&[
                ("callback", "FuckSRUNJsonP"),
                ("username", &username),
                ("ip", &ip),
            ])
            .send()?;
        if resp.status().as_u16() != 200 {
            let status = resp.status().as_u16();
            return Err(Box::new(SRUNClientError {
                message: String::from(format!("Server responded with code {status}")),
            }));
        }
        let content = resp.text()?;

        Ok(SRUNChallengeResponse::from_string(
            SRUNClient::extract_jsonp(content)?,
        )?)
    }

    pub fn get_ac_id(&self) -> Result<String, Box<dyn std::error::Error>> {
        let client = SRUNClient::get_client(self.options.interface.clone())?;
        let mut u = Url::parse(&self.server)?;
        u = u.join("/index_1.html")?;
        let resp = client.get(u.as_str()).send()?;
        if resp.status().as_u16() != 302 {
            let status = resp.status().as_u16();
            return Err(Box::new(SRUNClientError {
                message: String::from(format!("Server responded with code {status}")),
            }));
        }
        let redirect_address = resp.headers().get("Location").unwrap();
        // extract ac_id=? via regex
        let re = Regex::new(r"ac_id=(\d+)").unwrap();
        let caps = re.captures(redirect_address.to_str().unwrap()).unwrap();
        let ac_id = caps.get(1).unwrap().as_str();
        Ok(ac_id.to_string())
    }

    pub fn login(
        &self,
        challenge: &String,
        ip: &String,
        ac_id: &String,
    ) -> Result<SRUNLoginResponse, Box<dyn std::error::Error>> {
        let client = SRUNClient::get_client(self.options.interface.clone())?;
        let mut u = Url::parse(&self.server)?;
        u = u.join("/cgi-bin/srun_portal")?;

        let username = self.credentials.clone().unwrap().username;
        let password = self.credentials.clone().unwrap().password;

        let hmac = SRUNClient::calculate_password_hash(&password, challenge);
        let srbx1 = SRBX1::new(&username, &password, &ip, &ac_id);
        let auth_code = srbx1.calculate_auth_code(challenge);

        let checksum =
            SRUNClient::generate_checksum(&challenge, &username, &hmac, &ac_id, &ip, &auth_code);

        let resp = client
            .get(u.as_str())
            .query(&[
                ("callback", "FuckSRUNJsonP"),
                ("action", "login"),
                ("username", &username),
                ("password", &format!("{}{}", "{MD5}", hmac)),
                ("os", "Windows 10"),
                ("name", "Windows"),
                ("double_stack", "0"),
                ("chksum", &checksum),
                ("info", &auth_code),
                ("ac_id", &ac_id),
                ("ip", &ip),
                ("n", "200"),
                ("type", "1"),
            ])
            .send()?;
        if resp.status().as_u16() != 200 {
            let status = resp.status().as_u16();
            return Err(Box::new(SRUNClientError {
                message: String::from(format!("Server responded with code {status}")),
            }));
        }
        let content = resp.text()?;

        Ok(SRUNLoginResponse::from_string(SRUNClient::extract_jsonp(
            content,
        )?)?)
    }

    pub fn logout(
        &self,
        ip: &String,
        ac_id: &String,
    ) -> Result<SRUNLogoutResponse, Box<dyn std::error::Error>> {
        let client = SRUNClient::get_client(self.options.interface.clone())?;
        let mut u = Url::parse(&self.server)?;
        u = u.join("/cgi-bin/srun_portal")?;

        let username = self.credentials.clone().unwrap().username;

        let resp = client
            .get(u.as_str())
            .query(&[
                ("callback", "FuckSRUNJsonP"),
                ("action", "logout"),
                ("username", &username),
                ("ac_id", &ac_id),
                ("ip", &ip),
            ])
            .send()?;
        if resp.status().as_u16() != 200 {
            let status = resp.status().as_u16();
            return Err(Box::new(SRUNClientError {
                message: String::from(format!("Server responded with code {status}")),
            }));
        }
        let content = resp.text()?;

        Ok(SRUNLogoutResponse::from_string(SRUNClient::extract_jsonp(
            content,
        )?)?)
    }

    fn extract_jsonp(s: String) -> Result<String, Box<dyn std::error::Error>> {
        let mut content = s;
        if let (Some(start), Some(end)) = (content.find('('), content.rfind(')')) {
            content = content[start + 1..end].trim().to_string();
        } else {
            return Err(Box::new(SRUNClientError {
                message: String::from(format!("Invalid json format {content}")),
            }));
        }
        Ok(content)
    }

    fn get_client(
        interface: Option<String>,
    ) -> Result<reqwest::blocking::Client, Box<dyn std::error::Error>> {
        let mut client_builder = reqwest::blocking::Client::builder();
        client_builder = client_builder.redirect(reqwest::redirect::Policy::none());
        // Try to find a local address under speficied interface
        match interface {
            Some(interface) => {
                let mut address_map: HashMap<String, HashSet<IpAddr>> = HashMap::new();
                let network_interfaces =
                    list_afinet_netifas().expect("Failed to get network interfaces");
                for (name, ip) in network_interfaces.iter() {
                    if !address_map.contains_key(name) {
                        address_map.insert(name.clone(), HashSet::new());
                    }
                    address_map.get_mut(name).unwrap().insert(ip.clone());
                }

                if address_map.contains_key(&interface) {
                    let ips = address_map.get_mut(&interface).unwrap();
                    let mut has_ipv4 = false;
                    for ip in ips.iter() {
                        if ip.is_ipv4() {
                            has_ipv4 = true;
                            client_builder = client_builder.local_address(ip.clone());
                            break;
                        }
                    }
                    if !has_ipv4 {
                        // If no ipv4 address is found, use the first ipv6 address instead
                        client_builder =
                            client_builder.local_address(ips.iter().next().unwrap().clone());
                    }
                } else {
                    return Err(Box::new(SRUNClientError {
                        message: String::from(format!("Interface {} not found", interface)),
                    }));
                }
            }
            None => {}
        }

        Ok(client_builder.build()?)
    }

    fn calculate_password_hash(password: &String, challenge: &String) -> String {
        let mut mac = HmacMd5::new_from_slice(challenge.as_bytes()).unwrap();
        mac.update(password.as_bytes());
        let result = mac.finalize();
        let bytes = result.into_bytes();
        format!("{:x}", bytes)
    }

    pub fn generate_checksum(
        challenge: &String,
        username: &String,
        hmac: &String,
        ac_id: &String,
        ip: &String,
        i: &String,
    ) -> String {
        let hash_string = String::from_iter(vec![
            challenge.clone(),
            username.clone(),
            challenge.clone(),
            hmac.clone(),
            challenge.clone(),
            ac_id.clone(),
            challenge.clone(),
            ip.clone(),
            challenge.clone(),
            "200".to_string(),
            challenge.clone(),
            "1".to_string(),
            challenge.clone(),
            i.clone(),
        ]);
        let mut hasher = Sha1::new();
        hasher.update(hash_string);
        let result = hasher.finalize();
        let hex_str = format!("{:x}", result);
        hex_str
    }
}
