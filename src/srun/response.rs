use core::fmt;

use json::{self, object};

// TODO: Figure out types of numberic fields (Since my institution doesn't use them)

#[derive(Debug)]
pub struct SRUNQueryResponse {
    // Common response
    pub error: String,
    pub online_ip: String,

    // When user not online
    pub client_ip: Option<String>,
    pub ecode: Option<u64>,
    pub error_msg: Option<String>,
    pub res: Option<String>,
    pub srun_ver: Option<String>,
    pub st: Option<u64>,

    // When user online
    pub server_flag: Option<u64>,
    pub add_time: Option<u64>,
    pub all_bytes: Option<u64>,
    pub billing_name: Option<String>,
    pub bytes_in: Option<u64>,
    pub bytes_out: Option<u64>,
    pub checkout_date: Option<u64>,
    pub domain: Option<String>,
    pub group_id: Option<String>,
    pub keepalive_time: Option<u64>,
    pub online_device_total: Option<String>,
    pub online_ip6: Option<String>,
    pub package_id: Option<String>,
    pub products_id: Option<String>,
    pub products_name: Option<String>,
    pub real_name: Option<String>,
    pub remain_bytes: Option<u64>,
    pub remain_seconds: Option<u64>,
    pub sum_bytes: Option<u64>,
    pub sum_seconds: Option<u64>,
    pub sysver: Option<String>,
    pub user_balance: Option<u64>,
    pub user_charge: Option<u64>,
    pub user_mac: Option<String>,
    pub user_name: Option<String>,
    pub wallet_balance: Option<String>,
}

impl fmt::Display for SRUNQueryResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.error == "ok" {
            write!(f, "Login Status: OK\n")?;
        } else {
            write!(f, "Login Status: No\n")?;
        }
        write!(f, "Current IP: {}\n", self.online_ip)?;
        match self.online_device_total.clone() {
            Some(count) => {
                write!(f, "{} Devices Online\n", count)?;
            }
            None => {}
        }

        Ok(())
    }
}

impl SRUNQueryResponse {
    pub fn from_string(s: String) -> Result<SRUNQueryResponse, Box<dyn std::error::Error>> {
        let j = json::parse(&s)?;

        Ok(SRUNQueryResponse {
            error: j["error"].as_str().unwrap().to_string(),
            online_ip: j["online_ip"].as_str().unwrap().to_string(),
            client_ip: j["client_ip"].as_str().map(String::from),
            ecode: j["ecode"].as_u64(),
            error_msg: j["error_msg"].as_str().map(String::from),
            res: j["res"].as_str().map(String::from),
            srun_ver: j["srun_ver"].as_str().map(String::from),
            st: j["st"].as_u64(),
            server_flag: j["ServerFlag"].as_u64(),
            add_time: j["add_time"].as_u64(),
            all_bytes: j["all_bytes"].as_u64(),
            billing_name: j["billing_name"].as_str().map(String::from),
            bytes_in: j["bytes_in"].as_u64(),
            bytes_out: j["bytes_out"].as_u64(),
            checkout_date: j["checkout_date"].as_u64(),
            domain: j["domain"].as_str().map(String::from),
            group_id: j["group_id"].as_str().map(String::from),
            keepalive_time: j["keepalive_time"].as_u64(),
            online_device_total: j["online_device_total"].as_str().map(String::from),
            online_ip6: j["online_ip6"].as_str().map(String::from),
            package_id: j["package_id"].as_str().map(String::from),
            products_id: j["products_id"].as_str().map(String::from),
            products_name: j["products_name"].as_str().map(String::from),
            real_name: j["real_name"].as_str().map(String::from),
            remain_bytes: j["remain_bytes"].as_u64(),
            remain_seconds: j["remain_seconds"].as_u64(),
            sum_bytes: j["sum_bytes"].as_u64(),
            sum_seconds: j["sum_seconds"].as_u64(),
            sysver: j["sysver"].as_str().map(String::from),
            user_balance: j["user_balance"].as_u64(),
            user_charge: j["user_charge"].as_u64(),
            user_mac: j["user_mac"].as_str().map(String::from),
            user_name: j["user_name"].as_str().map(String::from),
            wallet_balance: j["wallet_balance"].as_str().map(String::from),
        })
    }

    pub fn to_json(&self) -> String {
        let obj = object! {
            error: self.error.clone(),
            online_ip: self.online_ip.clone(),
            client_ip: self.client_ip.clone(),
            ecode: self.ecode.clone(),
            error_msg: self.error_msg.clone(),
            res: self.res.clone(),
            srun_ver: self.srun_ver.clone(),
            st: self.st.clone(),
            server_flag: self.server_flag.clone(),
            add_time: self.add_time.clone(),
            all_bytes: self.all_bytes.clone(),
            billing_name: self.billing_name.clone(),
            bytes_in: self.bytes_in.clone(),
            bytes_out: self.bytes_out.clone(),
            checkout_date: self.checkout_date.clone(),
            domain: self.domain.clone(),
            group_id: self.group_id.clone(),
            keepalive_time: self.keepalive_time.clone(),
            online_device_total: self.online_device_total.clone(),
            online_ip6: self.online_ip6.clone(),
            package_id: self.package_id.clone(),
            products_id: self.products_id.clone(),
            products_name: self.products_name.clone(),
            real_name: self.real_name.clone(),
            remain_bytes: self.remain_bytes.clone(),
            remain_seconds: self.remain_seconds.clone(),
            sum_bytes: self.sum_bytes.clone(),
            sum_seconds: self.sum_seconds.clone(),
            sysver: self.sysver.clone(),
            user_balance: self.user_balance.clone(),
            user_charge: self.user_charge.clone(),
            user_mac: self.user_mac.clone(),
            user_name: self.user_name.clone(),
            wallet_balance: self.wallet_balance.clone(),
        };
        json::stringify(obj)
    }
}

#[derive(Debug)]
pub struct SRUNChallengeResponse {
    pub challenge: String,
    pub client_ip: String,
    pub ecode: u64,
    pub error: String,
    pub error_msg: String,
    pub expire: String,
    pub online_ip: String,
    pub res: String,
    pub srun_ver: String,
    pub st: u64,
}

impl fmt::Display for SRUNChallengeResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Current IP: {}\n", self.online_ip)?;
        write!(f, "Challenge: {}\n", self.challenge)?;
        write!(f, "Expires in {} seconds\n", self.expire)?;
        Ok(())
    }
}

impl SRUNChallengeResponse {
    pub fn from_string(s: String) -> Result<SRUNChallengeResponse, Box<dyn std::error::Error>> {
        let j = json::parse(&s)?;

        Ok(SRUNChallengeResponse {
            challenge: j["challenge"].as_str().unwrap().to_string(),
            client_ip: j["client_ip"].as_str().unwrap().to_string(),
            ecode: j["ecode"].as_u64().unwrap(),
            error: j["error"].as_str().unwrap().to_string(),
            error_msg: j["error_msg"].as_str().unwrap().to_string(),
            expire: j["expire"].as_str().unwrap().to_string(),
            online_ip: j["online_ip"].as_str().unwrap().to_string(),
            res: j["res"].as_str().unwrap().to_string(),
            srun_ver: j["srun_ver"].as_str().unwrap().to_string(),
            st: j["st"].as_u64().unwrap(),
        })
    }

    #[allow(dead_code)]
    pub fn to_json(&self) -> String {
        let obj = object! {
            challenge: self.challenge.clone(),
            client_ip: self.client_ip.clone(),
            ecode: self.ecode.clone(),
            error: self.error.clone(),
            error_msg: self.error_msg.clone(),
            expire: self.expire.clone(),
            online_ip: self.online_ip.clone(),
            res: self.res.clone(),
            srun_ver: self.srun_ver.clone(),
            st: self.st.clone(),
        };
        json::stringify(obj)
    }
}

#[derive(Debug)]
pub struct SRUNLoginResponse {
    pub client_ip: String,
    pub ecode: Option<u64>,
    pub error: String,
    pub error_msg: String,
    pub online_ip: String,
    pub res: String,
    pub srun_ver: String,

    pub st: Option<u64>,
    pub server_flag: Option<u64>,
    pub services_intf_server_ip: Option<String>,
    pub services_intf_server_port: Option<String>,
    pub access_token: Option<String>,
    pub checkout_date: Option<u64>,
    pub poly_msg: Option<String>,
    pub real_name: Option<String>,
    pub remain_flux: Option<u64>,
    pub remain_times: Option<u64>,
    pub suc_msg: Option<String>,
    pub sysver: Option<String>,
    pub username: Option<String>,
    pub wallet_balance: Option<u64>,
}

impl fmt::Display for SRUNLoginResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.error == "ok" {
            write!(f, "Login OK\n")?;
        } else {
            write!(f, "Login Failed: {}\n", self.error_msg)?;
        }
        write!(f, "Current IP: {}\n", self.online_ip)?;
        Ok(())
    }
}

impl SRUNLoginResponse {
    pub fn from_string(s: String) -> Result<SRUNLoginResponse, Box<dyn std::error::Error>> {
        let j = json::parse(&s)?;

        Ok(SRUNLoginResponse {
            client_ip: j["client_ip"].as_str().unwrap().to_string(),
            ecode: j["ecode"].as_u64(),
            error: j["error"].as_str().unwrap().to_string(),
            error_msg: j["error_msg"].as_str().unwrap().to_string(),
            online_ip: j["online_ip"].as_str().unwrap().to_string(),
            res: j["res"].as_str().unwrap().to_string(),
            srun_ver: j["srun_ver"].as_str().unwrap().to_string(),
            st: j["st"].as_u64(),
            server_flag: j["ServerFlag"].as_u64(),
            services_intf_server_ip: j["services_intf_server_ip"].as_str().map(String::from),
            services_intf_server_port: j["services_intf_server_port"].as_str().map(String::from),
            access_token: j["access_token"].as_str().map(String::from),
            checkout_date: j["checkout_date"].as_u64(),
            poly_msg: j["poly_msg"].as_str().map(String::from),
            real_name: j["real_name"].as_str().map(String::from),
            remain_flux: j["remain_flux"].as_u64(),
            remain_times: j["remain_times"].as_u64(),
            suc_msg: j["suc_msg"].as_str().map(String::from),
            sysver: j["sysver"].as_str().map(String::from),
            username: j["username"].as_str().map(String::from),
            wallet_balance: j["wallet_balance"].as_u64(),
        })
    }

    pub fn to_json(&self) -> String {
        let obj = object! {
            client_ip: self.client_ip.clone(),
            ecode: self.ecode.clone(),
            error: self.error.clone(),
            error_msg: self.error_msg.clone(),
            online_ip: self.online_ip.clone(),
            res: self.res.clone(),
            srun_ver: self.srun_ver.clone(),
            st: self.st.clone(),
            server_flag: self.server_flag.clone(),
            services_intf_server_ip: self.services_intf_server_ip.clone(),
            services_intf_server_port: self.services_intf_server_port.clone(),
            access_token: self.access_token.clone(),
            checkout_date: self.checkout_date.clone(),
            poly_msg: self.poly_msg.clone(),
            real_name: self.real_name.clone(),
            remain_flux: self.remain_flux.clone(),
            remain_times: self.remain_times.clone(),
            suc_msg: self.suc_msg.clone(),
            sysver: self.sysver.clone(),
            username: self.username.clone(),
            wallet_balance: self.wallet_balance.clone(),
        };
        json::stringify(obj)
    }
}

#[derive(Debug)]
pub struct SRUNLogoutResponse {
    pub client_ip: String,
    pub ecode: u64,
    pub error: String,
    pub error_msg: String,
    pub online_ip: String,
    pub res: String,
    pub srun_ver: String,
}

impl fmt::Display for SRUNLogoutResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.error == "ok" {
            write!(f, "Logout OK\n")?;
        } else {
            write!(f, "Logout Failed: {}\n", self.error_msg)?;
        }
        Ok(())
    }
}

impl SRUNLogoutResponse {
    pub fn from_string(s: String) -> Result<SRUNLogoutResponse, Box<dyn std::error::Error>> {
        let j = json::parse(&s)?;

        Ok(SRUNLogoutResponse {
            client_ip: j["client_ip"].as_str().unwrap().to_string(),
            ecode: j["ecode"].as_u64().unwrap(),
            error: j["error"].as_str().unwrap().to_string(),
            error_msg: j["error_msg"].as_str().unwrap().to_string(),
            online_ip: j["online_ip"].as_str().unwrap().to_string(),
            res: j["res"].as_str().unwrap().to_string(),
            srun_ver: j["srun_ver"].as_str().unwrap().to_string(),
        })
    }

    pub fn to_json(&self) -> String {
        let obj = object! {
            client_ip: self.client_ip.clone(),
            ecode: self.ecode.clone(),
            error: self.error.clone(),
            error_msg: self.error_msg.clone(),
            online_ip: self.online_ip.clone(),
            res: self.res.clone(),
            srun_ver: self.srun_ver.clone(),
        };
        json::stringify(obj)
    }
}
