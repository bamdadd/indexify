use anyhow::{anyhow, Result};

use crate::{
    api::Content,
    internal_api::{self, CoordinateResponse, ExtractResponse},
};

pub struct ExtractorRouter {
    coordinator_addr: String,
}

impl ExtractorRouter {
    pub fn new(coordinator_addr: &str) -> Self {
        Self {
            coordinator_addr: coordinator_addr.into(),
        }
    }

    pub async fn extract_content(
        &self,
        extractor_name: &str,
        content: Content,
    ) -> Result<Vec<Content>, anyhow::Error> {
        let request = internal_api::ExtractRequest {
            content: internal_api::Content {
                content_type: content.content_type,
                source: content.source,
                feature: None,
            },
        };

        let coordinate_request = internal_api::CoordinateRequest {
            extractor_name: extractor_name.to_string(),
        };

        let coordinate_response = reqwest::Client::new()
            .post(&format!("http://{}/coordinates", self.coordinator_addr))
            .json(&coordinate_request)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("unable to get coordinates of extractor: {}", e))?
            .json::<CoordinateResponse>()
            .await?;
        let extractor_addr = coordinate_response
            .content
            .get(0)
            .ok_or(anyhow!("no extractor found"))?;
        let resp = reqwest::Client::new()
            .post(&format!("http://{}/extract", extractor_addr))
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("unable to embed query: {}", e))?
            .json::<ExtractResponse>()
            .await?;
        let content = resp
            .content
            .get(0)
            .ok_or(anyhow!("no content was extracted"))?
            .to_owned();
        Ok(vec![content.into()])
    }
}
