use reqwest::{header::{HeaderMap, HeaderValue}, Method, Client};
use serde::{Serialize};



pub mod models {
    use std::collections::HashMap;
    use getset::{Getters};

    use serde::{Serialize, Deserialize};
    #[derive(Serialize, Deserialize)]
    struct UnifiMetaResponse {
        rc: String
    }

    #[derive(Debug, Serialize, Deserialize, Getters)]
    #[get = "pub with_prefix"]
    pub struct ListClientsDevice {
        site_id: Option<String>,
        assoc_time: Option<i64>,
        latest_assoc_time: Option<i64>,
        hostname: Option<String>,
        // Begin Wireless Device Options
        _uptime_by_uap: Option<i64>,
        _last_seen_by_uap: Option<i64>,
        _is_guest_by_uap: Option<bool>,
        ap_mac: Option<String>,
        channel: Option<i64>,
        radio: Option<String>,
        radio_name: Option<String>,
        essid: Option<String>,
        bssid: Option<String>,
        powersave_enabled: Option<bool>,
        is_11r: Option<bool>,
        user_group_id_computed: Option<String>,
        anon_client_id: Option<String>,
        ccq: Option<i64>,
        dhcpend_time: Option<i64>,
        idletime: Option<i64>,
        noise: Option<i64>,
        nss: Option<i64>,
        rx_rate: Option<i64>,
        rssi: Option<i64>,
        satisfaction_now: Option<i64>,
        satisfaction_real: Option<i64>,
        satisfaction_reason: Option<i64>,
        signal: Option<i64>,
        tx_mcs: Option<i64>,
        tx_power: Option<i64>,
        tx_rate: Option<i64>,
        vlan: Option<i64>,
        radio_proto: Option<String>,
        tx_bytes: Option<i64>,
        rx_bytes: Option<i64>,
        tx_packets: Option<i64>,
        rx_packets: Option<i64>,
        #[serde(rename = "tx_bytes-r")]
        tx_bytes_r: Option<i64>,
        #[serde(rename = "rx_bytes-r")]
        rx_bytes_r: Option<i64>,
        #[serde(rename = "bytes-r")]
        bytes_r: Option<i64>,
        wlanconf_id: Option<String>,
        disconnect_timestamp: Option<i64>,
        // End Wireless Device Options
        oui: Option<String>,
        user_id: Option<String>,
        _id: Option<String>,
        mac: Option<String>,
        is_guest: Option<bool>,
        first_seen: Option<i64>,
        last_seen: Option<i64>,
        is_wired: Option<bool>,
        usergroup_id: Option<String>,
        name: Option<String>,
        noted: Option<bool>,
        _uptime_by_usw: Option<i64>,
        _last_seen_by_usw: Option<i64>,
        _is_guest_by_usw: Option<bool>,
        sw_mac: Option<String>,
        sw_depth: Option<i64>,
        sw_port: Option<i64>,
        wired_rate_mbps: Option<i64>,
        network: Option<String>,
        network_id: Option<String>,
        anomalies: Option<i64>,
        satisfaction_avg: Option<HashMap<String, i64>>,
        uptime: Option<i64>,
        #[serde(rename = "wired-tx_bytes")]
        wired_tx_bytes: Option<i64>,
        #[serde(rename = "wired-rx_bytes")]
        wired_rx_bytes: Option<i64>,
        #[serde(rename = "wired-tx_packets")]
        wired_tx_packets: Option<i64>,
        #[serde(rename = "wired-rx_packets")]
        wired_rx_packets: Option<i64>,
        #[serde(rename = "wired-tx_bytes-r")]
        wired_tx_bytes_r: Option<i64>,
        #[serde(rename = "wired-rx_bytes-r")]
        wired_rx_bytes_r: Option<i64>,
        ip: Option<String>,
        hostname_source: Option<String>,
        satisfaction: Option<i64>,
        _uptime_by_ugw: Option<i64>,
        _last_seen_by_ugw: Option<i64>,
        _is_guest_by_ugw: Option<bool>,
        gw_mac: Option<String>,
        tx_retries: Option<i64>,
        wifi_tx_attempts: Option<i64>,
        authorized: Option<bool>,
        qos_policy_applied: Option<bool>,
        fingerprint_source: Option<i64>,
        dev_cat: Option<i64>,
        dev_family: Option<i64>,
        dev_vendor: Option<i64>,
        dev_id: Option<i64>,
        device_name: Option<String>,
        fw_version: Option<String>,
        score: Option<i64>,
        fingerprint_engine_version: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize, Getters)]
    #[get = "pub with_prefix"]
    pub struct ListClientsResponse {
        meta: HashMap<String, String>,
        data: Vec<ListClientsDevice>
    }

    #[derive(Serialize, Deserialize)]
    pub struct LoginBody {
        pub username: String,
        pub password: String,
        #[serde(rename = "rememberMe")]
        pub remember_me: bool
    }
}

pub struct UnifiApiClient {
    client: Client,
    base_url: String,
    headers: HeaderMap,
    username: String,
    password: String
}

use self::models::{LoginBody};

impl UnifiApiClient {
    async fn authenticate(&mut self) {
        let response = self.client.post(format!("{}/api/auth/login", self.base_url))
            .json(&LoginBody {
                username: self.username.to_owned(),
                password: self.password.to_owned(),
                remember_me: false
            })
            .send().await;

        self.headers.append("X-CSRF-Token", response.unwrap().headers().get("X-CSRF-Token").unwrap().into());
    }

    async fn request(&mut self, method: Method, path: &str, body: Option<impl Serialize>) -> Result<reqwest::Response, reqwest::Error> {
        self.authenticate().await;
        match &body {
            Some(request_body) => {
                self.client.request(method, format!("{}{}", self.base_url, path))
                    .json(&request_body)
                    .headers(self.headers.clone())
                    .send()
                    .await
            },
            None => {
                self.client.request(method, format!("{}{}", self.base_url, path))
                .headers(self.headers.clone())
                .send()
                .await
            }
        }
    }

    pub async fn list_clients(&mut self) -> Result<reqwest::Response, reqwest::Error> {
        self.request(Method::GET, "/proxy/network/api/s/default/stat/sta", None::<&str>).await
    }

    pub fn new(base_url: String, username: String, password: String) -> UnifiApiClient {
        let mut headers = HeaderMap::new();
        headers.append("Content-Type", HeaderValue::from_static("application/json"));
        let client = Client::builder().danger_accept_invalid_certs(true).cookie_store(true).build().unwrap();
        UnifiApiClient {
            client: client,
            base_url: base_url,
            headers: headers,
            username: username,
            password: password,
        }
    }
}