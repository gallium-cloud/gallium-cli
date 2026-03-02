mod types;

pub use types::*;

#[cfg(test)]
mod tests {
    use crate::info::QemuInfo;

    fn roundtrip_json(json_bytes: &[u8]) {
        let as_value: serde_json::Value = serde_json::from_slice(json_bytes).unwrap();
        let as_qemu_info: QemuInfo = serde_json::from_slice(json_bytes).unwrap();

        assert_eq!(as_value, serde_json::to_value(as_qemu_info).unwrap());
    }
    #[test]
    fn roundtrip_qcow2() {
        roundtrip_json(include_bytes!("testdata/qcow2.json"));
    }
    #[test]
    fn roundtrip_vmdk() {
        roundtrip_json(include_bytes!("testdata/vmdk.json"));
    }

    #[test]
    fn roundtrip_raw() {
        roundtrip_json(include_bytes!("testdata/raw1.json"));
    }

    #[test]
    fn roundtrip_vhdx() {
        roundtrip_json(include_bytes!("testdata/vhdx.json"));
    }

    #[test]
    fn roundtrip_vpc() {
        roundtrip_json(include_bytes!("testdata/vpc.json"));
    }
}
