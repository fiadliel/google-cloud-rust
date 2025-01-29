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

/// A dyn-compatible, crate-private version of [super::CloudScheduler].
#[async_trait::async_trait]
pub trait CloudScheduler: std::fmt::Debug + Send + Sync {
    async fn list_jobs(
        &self,
        req: crate::model::ListJobsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListJobsResponse>;

    async fn get_job(
        &self,
        req: crate::model::GetJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Job>;

    async fn create_job(
        &self,
        req: crate::model::CreateJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Job>;

    async fn update_job(
        &self,
        req: crate::model::UpdateJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Job>;

    async fn delete_job(
        &self,
        req: crate::model::DeleteJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn pause_job(
        &self,
        req: crate::model::PauseJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Job>;

    async fn resume_job(
        &self,
        req: crate::model::ResumeJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Job>;

    async fn run_job(
        &self,
        req: crate::model::RunJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Job>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location>;
}

/// All implementations of [crate::stubs::CloudScheduler] also implement [CloudScheduler].
#[async_trait::async_trait]
impl<T: crate::stubs::CloudScheduler> CloudScheduler for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_jobs(
        &self,
        req: crate::model::ListJobsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListJobsResponse> {
        T::list_jobs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_job(
        &self,
        req: crate::model::GetJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Job> {
        T::get_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_job(
        &self,
        req: crate::model::CreateJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Job> {
        T::create_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_job(
        &self,
        req: crate::model::UpdateJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Job> {
        T::update_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_job(
        &self,
        req: crate::model::DeleteJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn pause_job(
        &self,
        req: crate::model::PauseJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Job> {
        T::pause_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn resume_job(
        &self,
        req: crate::model::ResumeJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Job> {
        T::resume_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn run_job(
        &self,
        req: crate::model::RunJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Job> {
        T::run_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location> {
        T::get_location(self, req, options).await
    }
}
