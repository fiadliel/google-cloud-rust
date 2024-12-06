// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.

use gax::error::{Error, HttpError};
use crate::{Credential, Result};
use std::sync::Arc;

// Shared implementation across clients.
struct InnerClient {
    http_client: reqwest::Client,
    cred: Credential,
    endpoint: String,
}

#[derive(serde::Serialize)]
#[allow(dead_code)]
struct NoBody {}

/// An abstract interface that provides location-related information for
/// a service. Service-specific metadata is provided through the
/// [Location.metadata][google.cloud.location.Location.metadata] field.
#[derive(Clone)]
pub struct Locations {
    inner: Arc<InnerClient>,
}

impl std::fmt::Debug for Locations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Locations[{}]", self.inner.endpoint)
    }
}

impl Locations {
    pub async fn new() -> Result<Self> {
        Self::new_with_config(crate::ConfigBuilder::default()).await
    }

    pub async fn new_with_config(conf: crate::ConfigBuilder) -> Result<Self> {
        let inner = InnerClient {
            http_client: conf.client.unwrap_or(crate::ConfigBuilder::default_client()),
            cred: conf
                .cred
                .unwrap_or(crate::ConfigBuilder::default_credential().await?),
            endpoint: conf.endpoint.unwrap_or(crate::DEFAULT_HOST.to_string()),
        };
        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    async fn fetch_token(&self) -> Result<String> {
        let tok = self
            .inner
            .cred
            .access_token()
            .await
            .map_err(Error::authentication)?;
        Ok(tok.value)
    }

    async fn execute<I: serde::ser::Serialize, O: serde::de::DeserializeOwned>(
        access_token: String,
        mut builder: reqwest::RequestBuilder,
        body: Option<I>,
    ) -> Result<O> {
        builder = builder.bearer_auth(access_token);
        if let Some(body) = body {
            builder = builder.json(&body);
        }
        let resp = builder.send().await.map_err(Error::io)?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let headers = gax::error::convert_headers(resp.headers());
            let body = resp.bytes().await.map_err(Error::io)?;
            return Err(HttpError::new(status, headers, Some(body)).into());
        }
        let response = resp.json::<O>().await.map_err(Error::serde)?;
        Ok(response)
    }
}

impl crate::traits::Locations for Locations {
    /// Lists information about the supported locations for this service.
    async fn list_locations(&self, req: crate::model::ListLocationsRequest) -> Result<crate::model::ListLocationsResponse> {
        let inner_client = self.inner.clone();
        let builder = inner_client.http_client
            .get(format!(
               "{}/v1/{}",
               inner_client.endpoint,
               req.name,
            ))
            .query(&[("alt", "json")]);
        let builder = gax::query_parameter::add(builder, "filter", &req.filter).map_err(Error::other)?;
        let builder = gax::query_parameter::add(builder, "pageSize", &req.page_size).map_err(Error::other)?;
        let builder = gax::query_parameter::add(builder, "pageToken", &req.page_token).map_err(Error::other)?;
        let access_token = self.fetch_token().await?;
        Self::execute(access_token, builder, None::<NoBody>).await
    }

    /// Gets information about a location.
    async fn get_location(&self, req: crate::model::GetLocationRequest) -> Result<crate::model::Location> {
        let inner_client = self.inner.clone();
        let builder = inner_client.http_client
            .get(format!(
               "{}/v1/{}",
               inner_client.endpoint,
               req.name,
            ))
            .query(&[("alt", "json")]);
        let access_token = self.fetch_token().await?;
        Self::execute(access_token, builder, None::<NoBody>).await
    }

}

