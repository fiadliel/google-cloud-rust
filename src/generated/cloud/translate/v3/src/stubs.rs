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

//! Traits to mock the clients in this library.
//!
//! Application developers may need to mock the clients in this library to test
//! how their application works with different (and sometimes hard to trigger)
//! client and service behavior. Such test can define mocks implementing the
//! trait(s) defined in this module, initialize the client with an instance of
//! this mock in their tests, and verify their application responds as expected.

#![allow(rustdoc::broken_intra_doc_links)]

use gax::error::Error;
use std::sync::Arc;

pub(crate) mod dynamic;

/// Defines the trait used to implement [crate::client::TranslationService].
///
/// Application developers may need to implement this trait to mock
/// `client::TranslationService`.  In other use-cases, application developers only
/// use `client::TranslationService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait TranslationService: std::fmt::Debug + Send + Sync {
    /// Implements [crate::client::TranslationService::translate_text].
    fn translate_text(
        &self,
        _req: crate::model::TranslateTextRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TranslateTextResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::TranslateTextResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::romanize_text].
    fn romanize_text(
        &self,
        _req: crate::model::RomanizeTextRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::RomanizeTextResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::RomanizeTextResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::detect_language].
    fn detect_language(
        &self,
        _req: crate::model::DetectLanguageRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::DetectLanguageResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::DetectLanguageResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::TranslationService::get_supported_languages].
    fn get_supported_languages(
        &self,
        _req: crate::model::GetSupportedLanguagesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SupportedLanguages>> + Send
    {
        std::future::ready::<crate::Result<crate::model::SupportedLanguages>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::translate_document].
    fn translate_document(
        &self,
        _req: crate::model::TranslateDocumentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TranslateDocumentResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::TranslateDocumentResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::TranslationService::batch_translate_text].
    fn batch_translate_text(
        &self,
        _req: crate::model::BatchTranslateTextRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::batch_translate_document].
    fn batch_translate_document(
        &self,
        _req: crate::model::BatchTranslateDocumentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::create_glossary].
    fn create_glossary(
        &self,
        _req: crate::model::CreateGlossaryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::update_glossary].
    fn update_glossary(
        &self,
        _req: crate::model::UpdateGlossaryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::list_glossaries].
    fn list_glossaries(
        &self,
        _req: crate::model::ListGlossariesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListGlossariesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListGlossariesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::TranslationService::get_glossary].
    fn get_glossary(
        &self,
        _req: crate::model::GetGlossaryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Glossary>> + Send {
        std::future::ready::<crate::Result<crate::model::Glossary>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::delete_glossary].
    fn delete_glossary(
        &self,
        _req: crate::model::DeleteGlossaryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::get_glossary_entry].
    fn get_glossary_entry(
        &self,
        _req: crate::model::GetGlossaryEntryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::GlossaryEntry>> + Send {
        std::future::ready::<crate::Result<crate::model::GlossaryEntry>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::list_glossary_entries].
    fn list_glossary_entries(
        &self,
        _req: crate::model::ListGlossaryEntriesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListGlossaryEntriesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListGlossaryEntriesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::TranslationService::create_glossary_entry].
    fn create_glossary_entry(
        &self,
        _req: crate::model::CreateGlossaryEntryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::GlossaryEntry>> + Send {
        std::future::ready::<crate::Result<crate::model::GlossaryEntry>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::update_glossary_entry].
    fn update_glossary_entry(
        &self,
        _req: crate::model::UpdateGlossaryEntryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::GlossaryEntry>> + Send {
        std::future::ready::<crate::Result<crate::model::GlossaryEntry>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::delete_glossary_entry].
    fn delete_glossary_entry(
        &self,
        _req: crate::model::DeleteGlossaryEntryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::TranslationService::create_dataset].
    fn create_dataset(
        &self,
        _req: crate::model::CreateDatasetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::get_dataset].
    fn get_dataset(
        &self,
        _req: crate::model::GetDatasetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Dataset>> + Send {
        std::future::ready::<crate::Result<crate::model::Dataset>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::list_datasets].
    fn list_datasets(
        &self,
        _req: crate::model::ListDatasetsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListDatasetsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListDatasetsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::delete_dataset].
    fn delete_dataset(
        &self,
        _req: crate::model::DeleteDatasetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::create_adaptive_mt_dataset].
    fn create_adaptive_mt_dataset(
        &self,
        _req: crate::model::CreateAdaptiveMtDatasetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AdaptiveMtDataset>> + Send
    {
        std::future::ready::<crate::Result<crate::model::AdaptiveMtDataset>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::delete_adaptive_mt_dataset].
    fn delete_adaptive_mt_dataset(
        &self,
        _req: crate::model::DeleteAdaptiveMtDatasetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::TranslationService::get_adaptive_mt_dataset].
    fn get_adaptive_mt_dataset(
        &self,
        _req: crate::model::GetAdaptiveMtDatasetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AdaptiveMtDataset>> + Send
    {
        std::future::ready::<crate::Result<crate::model::AdaptiveMtDataset>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::list_adaptive_mt_datasets].
    fn list_adaptive_mt_datasets(
        &self,
        _req: crate::model::ListAdaptiveMtDatasetsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListAdaptiveMtDatasetsResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::ListAdaptiveMtDatasetsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::TranslationService::adaptive_mt_translate].
    fn adaptive_mt_translate(
        &self,
        _req: crate::model::AdaptiveMtTranslateRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AdaptiveMtTranslateResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::AdaptiveMtTranslateResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::TranslationService::get_adaptive_mt_file].
    fn get_adaptive_mt_file(
        &self,
        _req: crate::model::GetAdaptiveMtFileRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AdaptiveMtFile>> + Send {
        std::future::ready::<crate::Result<crate::model::AdaptiveMtFile>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::delete_adaptive_mt_file].
    fn delete_adaptive_mt_file(
        &self,
        _req: crate::model::DeleteAdaptiveMtFileRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::TranslationService::import_adaptive_mt_file].
    fn import_adaptive_mt_file(
        &self,
        _req: crate::model::ImportAdaptiveMtFileRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ImportAdaptiveMtFileResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::ImportAdaptiveMtFileResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::TranslationService::list_adaptive_mt_files].
    fn list_adaptive_mt_files(
        &self,
        _req: crate::model::ListAdaptiveMtFilesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListAdaptiveMtFilesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListAdaptiveMtFilesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::TranslationService::list_adaptive_mt_sentences].
    fn list_adaptive_mt_sentences(
        &self,
        _req: crate::model::ListAdaptiveMtSentencesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListAdaptiveMtSentencesResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListAdaptiveMtSentencesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::TranslationService::import_data].
    fn import_data(
        &self,
        _req: crate::model::ImportDataRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::export_data].
    fn export_data(
        &self,
        _req: crate::model::ExportDataRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::list_examples].
    fn list_examples(
        &self,
        _req: crate::model::ListExamplesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListExamplesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListExamplesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::create_model].
    fn create_model(
        &self,
        _req: crate::model::CreateModelRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::list_models].
    fn list_models(
        &self,
        _req: crate::model::ListModelsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListModelsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListModelsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::get_model].
    fn get_model(
        &self,
        _req: crate::model::GetModelRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Model>> + Send {
        std::future::ready::<crate::Result<crate::model::Model>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::TranslationService::delete_model].
    fn delete_model(
        &self,
        _req: crate::model::DeleteModelRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::ListLocationsResponse>> + Send
    {
        std::future::ready::<crate::Result<location::model::ListLocationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::TranslationService::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::TranslationService::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::TranslationService::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::TranslationService::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::TranslationService::wait_operation].
    fn wait_operation(
        &self,
        _req: longrunning::model::WaitOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Returns the polling policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        Arc::new(gax::polling_policy::Aip194Strict)
    }

    /// Returns the polling backoff policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_backoff_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        Arc::new(gax::exponential_backoff::ExponentialBackoff::default())
    }
}
