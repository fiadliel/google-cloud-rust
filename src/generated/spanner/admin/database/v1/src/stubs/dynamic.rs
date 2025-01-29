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

use std::sync::Arc;

/// A dyn-compatible, crate-private version of [super::DatabaseAdmin].
#[async_trait::async_trait]
pub trait DatabaseAdmin: std::fmt::Debug + Send + Sync {
    async fn list_databases(
        &self,
        req: crate::model::ListDatabasesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDatabasesResponse>;

    async fn create_database(
        &self,
        req: crate::model::CreateDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_database(
        &self,
        req: crate::model::GetDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Database>;

    async fn update_database(
        &self,
        req: crate::model::UpdateDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_database_ddl(
        &self,
        req: crate::model::UpdateDatabaseDdlRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn drop_database(
        &self,
        req: crate::model::DropDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn get_database_ddl(
        &self,
        req: crate::model::GetDatabaseDdlRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GetDatabaseDdlResponse>;

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse>;

    async fn create_backup(
        &self,
        req: crate::model::CreateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn copy_backup(
        &self,
        req: crate::model::CopyBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_backup(
        &self,
        req: crate::model::GetBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Backup>;

    async fn update_backup(
        &self,
        req: crate::model::UpdateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Backup>;

    async fn delete_backup(
        &self,
        req: crate::model::DeleteBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn list_backups(
        &self,
        req: crate::model::ListBackupsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBackupsResponse>;

    async fn restore_database(
        &self,
        req: crate::model::RestoreDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_database_operations(
        &self,
        req: crate::model::ListDatabaseOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDatabaseOperationsResponse>;

    async fn list_backup_operations(
        &self,
        req: crate::model::ListBackupOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBackupOperationsResponse>;

    async fn list_database_roles(
        &self,
        req: crate::model::ListDatabaseRolesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDatabaseRolesResponse>;

    async fn create_backup_schedule(
        &self,
        req: crate::model::CreateBackupScheduleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BackupSchedule>;

    async fn get_backup_schedule(
        &self,
        req: crate::model::GetBackupScheduleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BackupSchedule>;

    async fn update_backup_schedule(
        &self,
        req: crate::model::UpdateBackupScheduleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BackupSchedule>;

    async fn delete_backup_schedule(
        &self,
        req: crate::model::DeleteBackupScheduleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn list_backup_schedules(
        &self,
        req: crate::model::ListBackupSchedulesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBackupSchedulesResponse>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [crate::stubs::DatabaseAdmin] also implement [DatabaseAdmin].
#[async_trait::async_trait]
impl<T: crate::stubs::DatabaseAdmin> DatabaseAdmin for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_databases(
        &self,
        req: crate::model::ListDatabasesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDatabasesResponse> {
        T::list_databases(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_database(
        &self,
        req: crate::model::CreateDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_database(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_database(
        &self,
        req: crate::model::GetDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Database> {
        T::get_database(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_database(
        &self,
        req: crate::model::UpdateDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_database(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_database_ddl(
        &self,
        req: crate::model::UpdateDatabaseDdlRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_database_ddl(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn drop_database(
        &self,
        req: crate::model::DropDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::drop_database(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_database_ddl(
        &self,
        req: crate::model::GetDatabaseDdlRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::GetDatabaseDdlResponse> {
        T::get_database_ddl(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse> {
        T::test_iam_permissions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_backup(
        &self,
        req: crate::model::CreateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_backup(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn copy_backup(
        &self,
        req: crate::model::CopyBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::copy_backup(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_backup(
        &self,
        req: crate::model::GetBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Backup> {
        T::get_backup(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_backup(
        &self,
        req: crate::model::UpdateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Backup> {
        T::update_backup(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_backup(
        &self,
        req: crate::model::DeleteBackupRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_backup(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_backups(
        &self,
        req: crate::model::ListBackupsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBackupsResponse> {
        T::list_backups(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn restore_database(
        &self,
        req: crate::model::RestoreDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::restore_database(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_database_operations(
        &self,
        req: crate::model::ListDatabaseOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDatabaseOperationsResponse> {
        T::list_database_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_backup_operations(
        &self,
        req: crate::model::ListBackupOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBackupOperationsResponse> {
        T::list_backup_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_database_roles(
        &self,
        req: crate::model::ListDatabaseRolesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDatabaseRolesResponse> {
        T::list_database_roles(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_backup_schedule(
        &self,
        req: crate::model::CreateBackupScheduleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BackupSchedule> {
        T::create_backup_schedule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_backup_schedule(
        &self,
        req: crate::model::GetBackupScheduleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BackupSchedule> {
        T::get_backup_schedule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_backup_schedule(
        &self,
        req: crate::model::UpdateBackupScheduleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::BackupSchedule> {
        T::update_backup_schedule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_backup_schedule(
        &self,
        req: crate::model::DeleteBackupScheduleRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_backup_schedule(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_backup_schedules(
        &self,
        req: crate::model::ListBackupSchedulesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListBackupSchedulesResponse> {
        T::list_backup_schedules(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::cancel_operation(self, req, options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        T::get_polling_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}
