use std::time::Duration;
use bytes::Bytes;
use futures::Stream;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use crate::{
    config::AppConfig,
    error::{AppError, AppResult},
    models::{
        EmbeddingRequest, EmbeddingResponse, UpstreamChatCompletionRequest,
        UpstreamChatCompletionResponse,
    },
};

pub struct OmniRouteClient {
    client: reqwest::Client,
    base_url: String,
    api_key: Option<String>,
}

impl OmniRouteClient {
    pub fn new(config: &AppConfig, client: reqwest::Client) -> Self {
        Self {
            client,
            base_url: config.omniroute_base_url.clone(),
            api_key: config.omniroute_api_key.clone(),
        }
    }

    fn headers(&self) -> AppResult<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        if let Some(key) = &self.api_key {
            let auth_val = format!("Bearer {}", key);
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&auth_val)
                    .map_err(|e| AppError::Internal(format!("Invalid auth header: {}", e)))?,
            );
        }
        Ok(headers)
    }

    pub async fn chat_completion(
        &self,
        request: &UpstreamChatCompletionRequest,
    ) -> AppResult<UpstreamChatCompletionResponse> {
        let url = format!("{}/chat/completions", self.base_url);
        let resp = self
            .client
            .post(&url)
            .headers(self.headers()?)
            .json(request)
            .timeout(Duration::from_secs(120))
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "OmniRoute chat error [HTTP {}]: {}",
                status, body
            )));
        }

        let result = resp.json::<UpstreamChatCompletionResponse>().await?;
        Ok(result)
    }

    pub async fn chat_completion_stream(
        &self,
        request: &UpstreamChatCompletionRequest,
    ) -> AppResult<impl Stream<Item = Result<Bytes, reqwest::Error>>> {
        let url = format!("{}/chat/completions", self.base_url);
        let resp = self
            .client
            .post(&url)
            .headers(self.headers()?)
            .json(request)
            .timeout(Duration::from_secs(120))
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "OmniRoute stream error [HTTP {}]: {}",
                status, body
            )));
        }

        Ok(resp.bytes_stream())
    }

    pub async fn create_embeddings(
        &self,
        texts: Vec<String>,
        model: &str,
    ) -> AppResult<Vec<Vec<f32>>> {
        let url = format!("{}/embeddings", self.base_url);
        let payload = EmbeddingRequest {
            input: texts,
            model: model.to_string(),
        };

        let resp = self
            .client
            .post(&url)
            .headers(self.headers()?)
            .json(&payload)
            .timeout(Duration::from_secs(60))
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "OmniRoute embeddings error [HTTP {}]: {}",
                status, body
            )));
        }

        let res = resp.json::<EmbeddingResponse>().await?;
        let embeddings = res.data.into_iter().map(|item| item.embedding).collect();
        Ok(embeddings)
    }
}
