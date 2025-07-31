use reqwest::{Response, Error, Client};
use serde::{Deserialize, Serialize};

pub trait Messenger {
    async fn send_message_to(&self, recv: User, msg: String) -> Result<Response, Error>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub player_count: u32,
    pub status: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    status: String,
    message: String,
}

impl Default for ServerInfo {
    fn default() -> Self {
        Self {
            player_count: 0,
            status: "offline".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    username: String,
    addr: String,
    client: Client,
}

impl Messenger for User {
    async fn send_message_to(&self, recv: User, msg: String) -> Result<Response, Error> {
        Ok(self.client.post(recv.addr).body(msg).send().await?)
    }
}

impl User {
    pub fn new(username: String, addr: String) -> Self {
        Self {
            username,
            addr,
            client: reqwest::Client::new(),
        }
    }

    pub async fn yeehaw_partner(&self) -> Result<Response, Error> {
        Ok(self.client.get("194.163.183.44:8000/api/playercount").send().await?)
    }

    pub async fn get_server_info(&self) -> ServerInfo {
        match self.client.get("http://194.163.183.44:8000/api/playercount").send().await {
            Ok(response) => {
                if response.status().is_success() {
                    if let Ok(api_response) = response.json::<ApiResponse>().await {
                        let player_count = self.extract_player_count(&api_response.message);
                        let status = if api_response.message.contains("online") {
                            "online".to_string()
                        } else {
                            "offline".to_string()
                        };
                        
                        return ServerInfo {
                            player_count,
                            status,
                        };
                    }
                }
                ServerInfo {
                    player_count: 0,
                    status: "error".to_string(),
                }
            }
            Err(_) => ServerInfo::default(),
        }
    }

    fn extract_player_count(&self, message: &str) -> u32 {
        if let Some(start) = message.find("currently has ") {
            let after_has = &message[start + 14..];
            if let Some(end) = after_has.find(" players") {
                let count_str = &after_has[..end];
                return count_str.parse().unwrap_or(0);
            }
        }
        0
    }
}
