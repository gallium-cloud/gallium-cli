use crate::api::command_v2_api::entities::ApiCmdStatus;
use crate::helpers::cmd::cmd_progress::CommandProgressUpdater;
use crate::helpers::qemu::{ConvertTask, QemuImgConvert};
use crate::task_common::error::TaskError;
use cliclack::{MultiProgress, ProgressBar, multi_progress, progress_bar, spinner};

pub struct TransferProgressUi {
    pub multi: MultiProgress,
    pub spinner_init: ProgressBar,
    pub pb: ProgressBar,
    pub spinner_final: ProgressBar,
}

impl TransferProgressUi {
    pub fn init(task_description: String) -> Self {
        let multi = multi_progress(task_description);
        let spinner_init = multi.add(spinner());
        let pb = multi.add(progress_bar(10000));
        let spinner_final = multi.add(spinner());

        Self {
            multi,
            spinner_init,
            pb,
            spinner_final,
        }
    }
}

pub async fn transfer_progress_ui(
    mut convert_task: ConvertTask,
    progress_updater: CommandProgressUpdater,
    ui: TransferProgressUi,
) -> Result<(), TaskError> {
    let mut ui_tick = tokio::time::interval(tokio::time::Duration::from_millis(100));
    let mut backend_tick = tokio::time::interval(tokio::time::Duration::from_millis(5000));

    // qemu-img can take some time after progress has reached 100% before it will actually terminate.
    // (it is waiting for the file/volume to fsync, among other things)
    // Rather than show a progress bar stuck at 100%, switch to a spinner.
    let mut waiting_for_cmd_to_complete = false;

    loop {
        tokio::select! {
            _ = ui_tick.tick() => {
                if !waiting_for_cmd_to_complete {
                    let p = convert_task.progress.read_progress();
                    ui.pb.set_position(p as u64);
                    if p == 10000 {
                        ui.pb.stop("Sending data");
                        waiting_for_cmd_to_complete = true;
                        ui.spinner_final.start("Waiting for completion");
                    }
                }
            }
            _ = backend_tick.tick() => {
                progress_updater.update_progress(convert_task.progress.read_progress() as f64, 10000.0);
            }
            r = &mut convert_task.handle => {
                return match QemuImgConvert::assert_ok(r) {
                    Ok(_) => {
                        progress_updater.complete(ApiCmdStatus::COMPLETE).await?;
                        ui.spinner_final.stop("Import complete");
                        ui.multi.stop();
                        Ok(())
                    }
                    Err(e) => {
                        progress_updater.complete(ApiCmdStatus::FAILED).await.ok();
                        ui.spinner_final.error("Import failed");
                        ui.multi.stop();

                        Err(TaskError::HelperCommand { source: e })
                    }
                };
            }
        }
    }
}
