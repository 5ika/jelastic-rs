use serde_json::json;

const APP_ID: &str = "1dd8d191d38fff45e62564fcf67fdcd6";

pub struct Api {
    pub host: String,
    pub token: String,
}

impl Api {
    async fn request(
        &self,
        endpoint: &str,
        content: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let client = reqwest::Client::new();
        let params = json!(content);
        let auth_params = json!({
            "session": &self.token,
            "appid": APP_ID
        });
        let response = client
            .post(String::from("https://") + &self.host + endpoint)
            .query(&params)
            .query(&auth_params)
            .send()
            .await?;

        let body: serde_json::Value = response.json().await?;
        Ok(body)
    }

    pub async fn get_envs(&self) -> Result<serde_json::Value, reqwest::Error> {
        Api::request(&self, "/1.0/environment/control/rest/getenvs", None).await
    }

    pub async fn get_env_info(&self, env_name: &str) -> Result<serde_json::Value, reqwest::Error> {
        let params = json!({ "envName": env_name });
        Api::request(
            &self,
            "/1.0/environment/control/rest/getenvinfo",
            Some(params),
        )
        .await
    }

    pub async fn get_logs(
        &self,
        env_name: &str,
        node_id: u32,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let params = json!({ "envName": env_name, "nodeId": node_id });
        Api::request(&self, "/1.0/environment/control/rest/getlogs", Some(params)).await
    }

    pub async fn start_env(&self, env_name: &str) -> Result<serde_json::Value, reqwest::Error> {
        let params = json!({ "envName": env_name });
        Api::request(
            &self,
            "/1.0/environment/control/rest/startenv",
            Some(params),
        )
        .await
    }

    pub async fn stop_env(&self, env_name: &str) -> Result<serde_json::Value, reqwest::Error> {
        let params = json!({ "envName": env_name });
        Api::request(&self, "/1.0/environment/control/rest/stopenv", Some(params)).await
    }
}
