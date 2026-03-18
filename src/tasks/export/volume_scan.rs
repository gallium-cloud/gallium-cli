use crate::api::ApiClient;
use crate::api::cluster_vm_api::entities::{ListVirtualMachinesPathParams, VirtualMachine};
use crate::api::storage_api::entities::{ListVolumesPathParams, Volume};
use crate::task_common::error::TaskError;
use crate::tasks::export::ExportArguments;
use std::collections::HashSet;
use std::sync::Arc;

pub struct ScannedVolume {
    pub vm_name: Option<String>,
    pub kube_name: String,
    pub size_bytes: u64,
    pub used_size_bytes: Option<u64>,
}

pub async fn scan_volumes_for_export(
    api_client: &Arc<ApiClient>,
    args: &ExportArguments,
) -> Result<Vec<ScannedVolume>, TaskError> {
    if args.vm.is_empty() && args.vol.is_empty() {
        return Err(TaskError::RequiredParameterMissing {
            reason: "At least one VM or volume must be specified for export.",
        });
    } else if args.vm.is_empty() == args.vol.is_empty() {
        // this restriction is imposed for now, only to avoid having to deal with the case where
        // a user specifies a volume and the VM that volume is a part of in one command.
        return Err(TaskError::InvalidState {
            reason: "Cannot specify both VMs and volumes in one export command.",
        });
    }
    let storage_api = api_client.storage_api();
    let volume_list = storage_api
        .list_volumes(&ListVolumesPathParams {
            cluster_id: args.source.clone(),
            kube_ns: "default".into(),
        })
        .await?
        .volumes;

    let vm_api = api_client.cluster_vm_api();
    let vm_list = vm_api
        .list_virtual_machines(&ListVirtualMachinesPathParams {
            cluster_id: args.source.clone(),
            kube_ns: "default".into(),
        })
        .await?
        .items;

    let mut scanned_volumes = vec![];

    let mut matched_vms = HashSet::new();

    for vm_arg in args.vm.iter() {
        let vm = match_vm_arg(vm_arg, &vm_list)?;
        if !matched_vms.insert(vm.kube_name.to_string()) {
            return Err(TaskError::UserInputInvalidValueReason {
                val: vm_arg.to_string(),
                field: "vm",
                reason: "Same VM specified more than once",
            });
        }

        for vm_vol in vm.volumes.iter() {
            let vol = volume_list
                .iter()
                .find(|v| v.kube_name.as_str() == vm_vol.volume_kube_name.as_str())
                .ok_or_else(|| TaskError::InvalidState {
                    reason: "Volume present on VM missing from volume list",
                })?;
            scanned_volumes.push(ScannedVolume {
                vm_name: Some(vm.kube_name.clone()),
                kube_name: vol.kube_name.clone(),
                size_bytes: vol.size_bytes,
                used_size_bytes: vol.used_size_bytes,
            });
        }
    }

    let mut matched_vols = HashSet::new();
    for vol_arg in args.vol.iter() {
        let vol = match_vol_arg(vol_arg, &volume_list)?;
        if !matched_vols.insert(vol.kube_name.to_string()) {
            return Err(TaskError::UserInputInvalidValueReason {
                val: vol_arg.to_string(),
                field: "vm",
                reason: "Same volume specified more than once",
            });
        } else {
            scanned_volumes.push(ScannedVolume {
                vm_name: None,
                kube_name: vol.kube_name.clone(),
                size_bytes: vol.size_bytes,
                used_size_bytes: vol.used_size_bytes,
            });
        }
    }

    Ok(scanned_volumes)
}

fn match_vm_arg<'a>(
    vm_arg: &str,
    vm_list: &'a [VirtualMachine],
) -> Result<&'a VirtualMachine, TaskError> {
    let mut matched_vm = None;
    let mut matched_count = 0;
    for vm in vm_list.iter() {
        if vm.kube_name.as_str() == vm_arg {
            return Ok(vm);
        } else if vm.name.as_deref() == Some(vm_arg) {
            matched_vm = Some(vm);
            matched_count += 1;
        }
    }
    if matched_count == 1
        && let Some(vm) = matched_vm
    {
        Ok(vm)
    } else if matched_count == 0 {
        Err(TaskError::UserInputInvalidValueReason {
            val: vm_arg.to_string(),
            field: "vm",
            reason: "Does not match any VMs on deployment",
        })
    } else {
        //TODO: this should log the specific VMs
        Err(TaskError::UserInputInvalidValueReason {
            val: vm_arg.to_string(),
            field: "vm",
            reason: "Ambiguously matches multiple VMs",
        })
    }
}

// Is it worth trying to build a generic matcher? I think we will end up with this pattern repeated more.
fn match_vol_arg<'a>(vol_arg: &str, vol_list: &'a [Volume]) -> Result<&'a Volume, TaskError> {
    let mut matched_vol = None;
    let mut matched_count = 0;
    for vol in vol_list.iter() {
        if vol.kube_name.as_str() == vol_arg {
            return Ok(vol);
        } else if vol.name.as_deref() == Some(vol_arg) {
            matched_vol = Some(vol);
            matched_count += 1;
        }
    }

    if matched_count == 1
        && let Some(vol) = matched_vol
    {
        Ok(vol)
    } else if matched_count == 0 {
        Err(TaskError::UserInputInvalidValueReason {
            val: vol_arg.to_string(),
            field: "vol",
            reason: "Does not match any volumes on deployment",
        })
    } else {
        Err(TaskError::UserInputInvalidValueReason {
            val: vol_arg.to_string(),
            field: "vol",
            reason: "Ambiguously matches multiple volumes",
        })
    }
}
