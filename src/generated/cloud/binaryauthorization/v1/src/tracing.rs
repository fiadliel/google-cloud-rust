// Copyright 2025 Google LLC
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
use crate::Result;

/// Implements a [BinauthzManagementServiceV1](crate::stubs::BinauthzManagementServiceV1) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct BinauthzManagementServiceV1<T>
where
    T: crate::stubs::BinauthzManagementServiceV1 + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> BinauthzManagementServiceV1<T>
where
    T: crate::stubs::BinauthzManagementServiceV1 + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::BinauthzManagementServiceV1 for BinauthzManagementServiceV1<T>
where
    T: crate::stubs::BinauthzManagementServiceV1 + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn get_policy(
        &self,
        req: crate::model::GetPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Policy> {
        self.inner.get_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_policy(
        &self,
        req: crate::model::UpdatePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Policy> {
        self.inner.update_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_attestor(
        &self,
        req: crate::model::CreateAttestorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Attestor> {
        self.inner.create_attestor(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_attestor(
        &self,
        req: crate::model::GetAttestorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Attestor> {
        self.inner.get_attestor(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_attestor(
        &self,
        req: crate::model::UpdateAttestorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Attestor> {
        self.inner.update_attestor(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_attestors(
        &self,
        req: crate::model::ListAttestorsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAttestorsResponse> {
        self.inner.list_attestors(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_attestor(
        &self,
        req: crate::model::DeleteAttestorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_attestor(req, options).await
    }
}

/// Implements a [SystemPolicyV1](crate::stubs::SystemPolicyV1) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct SystemPolicyV1<T>
where
    T: crate::stubs::SystemPolicyV1 + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> SystemPolicyV1<T>
where
    T: crate::stubs::SystemPolicyV1 + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::SystemPolicyV1 for SystemPolicyV1<T>
where
    T: crate::stubs::SystemPolicyV1 + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn get_system_policy(
        &self,
        req: crate::model::GetSystemPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Policy> {
        self.inner.get_system_policy(req, options).await
    }
}

/// Implements a [ValidationHelperV1](crate::stubs::ValidationHelperV1) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct ValidationHelperV1<T>
where
    T: crate::stubs::ValidationHelperV1 + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> ValidationHelperV1<T>
where
    T: crate::stubs::ValidationHelperV1 + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::ValidationHelperV1 for ValidationHelperV1<T>
where
    T: crate::stubs::ValidationHelperV1 + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn validate_attestation_occurrence(
        &self,
        req: crate::model::ValidateAttestationOccurrenceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ValidateAttestationOccurrenceResponse> {
        self.inner
            .validate_attestation_occurrence(req, options)
            .await
    }
}
