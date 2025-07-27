use reqwest::{Response, Error, Client};

pub trait Messenger {
    async fn send_message_to(&self, recv: User, msg: String) -> Result<Response, Error>;
}

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
}
