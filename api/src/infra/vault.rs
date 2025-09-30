use reqwest::Client;
#[derive(Clone)]
pub struct VaultClient {
    pub addr: String,
    pub token: String,
    http: Client,
}
impl VaultClient {
    pub fn new(addr: String, token: String) -> Self {
        Self {
            addr,
            token,
            http: Client::new(),
        }
    }
    pub async fn kv_get(&self, path: &str) -> anyhow::Result<serde_json::Value> {
        let url = format!("{}/v1/kv/data/{}", self.addr, path);
        Ok(self
            .http
            .get(url)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?)
    }
    pub async fn transit_encrypt(&self, key: &str, plaintext_b64: &str) -> anyhow::Result<String> {
        let url = format!("{}/v1/transit/encrypt/{}", self.addr, key);
        let r: serde_json::Value = self
            .http
            .post(url)
            .bearer_auth(&self.token)
            .json(&serde_json::json!({ "plaintext": plaintext_b64 }))
            .send()
            .await?
            .json()
            .await?;
        Ok(r["data"]["ciphertext"].as_str().unwrap_or("").to_string())
    }
    pub async fn transit_decrypt(&self, key: &str, ciphertext: &str) -> anyhow::Result<String> {
        let url = format!("{}/v1/transit/decrypt/{}", self.addr, key);
        let r: serde_json::Value = self
            .http
            .post(url)
            .bearer_auth(&self.token)
            .json(&serde_json::json!({ "ciphertext": ciphertext }))
            .send()
            .await?
            .json()
            .await?;
        Ok(r["data"]["plaintext"].as_str().unwrap_or("").to_string())
    }
}
