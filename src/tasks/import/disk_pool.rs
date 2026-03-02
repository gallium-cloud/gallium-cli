use crate::api::storage_api::StorageApi;
use crate::api::storage_api::entities::{DiskPoolSummary, ListDiskPoolsPathParams};
use crate::task_common::error::TaskError;
use crate::tasks::import::ImportArguments;

#[derive(Debug, Clone)]
pub struct DiskPoolDetermination {
    pub kube_name: String,
    #[allow(unused)]
    pub display_name: Option<String>,
    #[allow(unused)]
    pub reason: &'static str,
}
impl DiskPoolDetermination {
    fn from_summary(pool: &DiskPoolSummary, reason: &'static str) -> Self {
        DiskPoolDetermination {
            kube_name: pool.kube_name.clone(),
            display_name: pool.name.clone(),
            reason,
        }
    }
}
pub async fn determine_disk_pool(
    storage_api: &StorageApi,
    args: &ImportArguments,
) -> Result<DiskPoolDetermination, TaskError> {
    let disk_pools = storage_api
        .list_disk_pools(&ListDiskPoolsPathParams {
            cluster_id: args.target.clone(),
        })
        .await?
        .pools;

    if let Some(pool_arg) = args.pool.as_deref() {
        if let Some(pool) = disk_pools.iter().find(|p| p.kube_name.as_str() == pool_arg) {
            Ok(DiskPoolDetermination::from_summary(
                pool,
                "ID matches parameter",
            ))
        } else if let Some(pool) = disk_pools
            .iter()
            .find(|p| p.name.as_deref() == Some(pool_arg))
        {
            Ok(DiskPoolDetermination::from_summary(
                pool,
                "Name matches parameter",
            ))
        } else {
            Err(TaskError::UserInputInvalidValueReason {
                val: pool_arg.to_string(),
                field: "pool",
                reason: "Matching pool does not exist",
            })
        }
    } else if let Some(pool) = disk_pools.iter().find(|p| p.is_default) {
        Ok(DiskPoolDetermination::from_summary(pool, "Pool is default"))
    } else {
        Err(TaskError::InvalidState {
            reason: "No default pool in deployment, and pool not specified.",
        })
    }
}
