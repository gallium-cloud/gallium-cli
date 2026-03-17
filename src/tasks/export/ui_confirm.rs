use crate::task_common::error::TaskError;
use crate::tasks::export::ExportArguments;
use crate::tasks::export::volume_scan::ScannedVolume;
use cliclack::{confirm, log};
use fs2::available_space;
use humansize::{BINARY, format_size};
use snafu::ResultExt;
use std::collections::BTreeMap;

pub fn confirm_export(
    export_vols: &[ScannedVolume],
    args: &ExportArguments,
) -> Result<bool, TaskError> {
    if export_vols.is_empty() {
        log::warning("Nothing to export.")
            .whatever_context::<_, TaskError>("writing to terminal")?;
        return Ok(false);
    }

    let mut grouped: BTreeMap<Option<&str>, Vec<&ScannedVolume>> = BTreeMap::new();
    for vol in export_vols {
        grouped.entry(vol.vm_name.as_deref()).or_default().push(vol);
    }

    let mut lines = Vec::new();
    let mut total_bytes: u64 = 0;

    for (vm_name, vols) in &grouped {
        if let Some(name) = vm_name {
            lines.push(format!("  VM: {}", name));
        }
        for v in vols {
            let size_str = match v.used_size_bytes {
                Some(used) => {
                    total_bytes += used;
                    format_size(used, BINARY).to_string()
                }
                None => {
                    total_bytes += v.size_bytes;
                    format!("{} (allocated)", format_size(v.size_bytes, BINARY))
                }
            };
            let indent = if vm_name.is_some() { "    " } else { "  " };
            lines.push(format!(
                "{}{} (estimated: {})",
                indent, v.kube_name, size_str
            ));
        }
    }

    lines.push(format!(
        "\n  Total export size: {}",
        format_size(total_bytes, BINARY)
    ));

    log::info(format!(
        "The following {} will be exported:\n{}",
        if export_vols.len() == 1 {
            "volume"
        } else {
            "volumes"
        },
        lines.join("\n"),
    ))
    .whatever_context::<_, TaskError>("writing to terminal")?;

    maybe_print_disk_space_warning(total_bytes)?;

    if !args.yes {
        let proceed = confirm("Proceed with export?")
            .initial_value(true)
            .interact()
            .whatever_context::<_, TaskError>("reading confirmation")?;
        if !proceed {
            log::warning("Export cancelled.")
                .whatever_context::<_, TaskError>("writing to terminal")?;
            return Ok(false);
        }
    }

    Ok(true)
}

fn maybe_print_disk_space_warning(total_bytes: u64) -> Result<(), TaskError> {
    let free_space_bytes =
        available_space(".").whatever_context::<_, TaskError>("checking available space")?;

    if free_space_bytes < total_bytes {
        log::warning(format!(
            "Insufficient free disk space: {} available, estimated {} required",
            format_size(free_space_bytes, BINARY),
            format_size(total_bytes, BINARY),
        ))
        .whatever_context::<_, TaskError>("writing to terminal")?;
    }

    Ok(())
}
