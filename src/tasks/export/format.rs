use clap::ValueEnum;

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum ExportFormat {
    Qcow2,
    Raw,
    Vmdk,
    Vhdx,
}
impl ExportFormat {
    pub fn as_qemu_img_fmt(&self) -> &'static str {
        match self {
            ExportFormat::Qcow2 => "qcow2",
            ExportFormat::Raw => "raw",
            ExportFormat::Vmdk => "vmdk",
            ExportFormat::Vhdx => "vhdx",
        }
    }

    pub fn as_ext(&self) -> &'static str {
        match self {
            ExportFormat::Qcow2 => ".qcow2",
            ExportFormat::Raw => ".img",
            ExportFormat::Vmdk => ".vmdk",
            ExportFormat::Vhdx => ".vhdx",
        }
    }
}
