use crate::api::command_v2_api::CommandApi;
use crate::api::storage_api::entities::CmdSubmitResponse;
use crate::helpers::cmd::cmd_response::poll_for_cmd_response_type;
use crate::helpers::helper_cmd_error::HelperCommandError;
use crate::helpers::mtls::init_mtls_cmd::InitMtlsCredentialsCmdResponse;
use crate::task_common::error::TaskError;
use base64::Engine;
use base64::engine::GeneralPurpose;
use rcgen::{CertificateParams, DnType, ExtendedKeyUsagePurpose, KeyPair};
use snafu::ResultExt;
use std::path::Path;
use tempfile::TempDir;

mod init_mtls_cmd;

const BASE64: GeneralPurpose = base64::engine::general_purpose::STANDARD;

pub struct MtlsCredentialHelper {
    keypair: KeyPair,
}

pub struct MtlsIssuedCredentialHelper {
    temp_dir: TempDir,
    keypair: KeyPair,
    init_response: InitMtlsCredentialsCmdResponse,
}
impl MtlsCredentialHelper {
    pub fn new() -> Result<Self, HelperCommandError> {
        let keypair =
            KeyPair::generate().whatever_context::<_, HelperCommandError>("generate keypair")?;

        Ok(Self { keypair })
    }

    pub fn get_csr_base64(&self) -> Result<String, HelperCommandError> {
        let mut cert_params = CertificateParams::new(vec![])
            .whatever_context::<_, HelperCommandError>("build cert_params")?;
        cert_params
            .distinguished_name
            .push(DnType::CommonName, "cli-client");
        cert_params
            .extended_key_usages
            .push(ExtendedKeyUsagePurpose::ClientAuth);

        let csr = cert_params
            .serialize_request(&self.keypair)
            .whatever_context::<_, HelperCommandError>("serialise CSR")?;

        Ok(BASE64.encode(csr.der()))
    }

    pub async fn poll_for_credentials(
        self,
        cmd_api: &CommandApi,
        submit_resp: &CmdSubmitResponse,
    ) -> Result<MtlsIssuedCredentialHelper, TaskError> {
        let init_response: InitMtlsCredentialsCmdResponse =
            poll_for_cmd_response_type(cmd_api, submit_resp, "INIT_MTLS_CREDENTIALS").await?;

        Ok(MtlsIssuedCredentialHelper::build(self.keypair, init_response).await?)
    }
}

impl MtlsIssuedCredentialHelper {
    pub async fn build(
        keypair: KeyPair,
        init_response: InitMtlsCredentialsCmdResponse,
    ) -> Result<Self, HelperCommandError> {
        let temp_dir = TempDir::new()?;

        Ok(Self {
            temp_dir,
            keypair,
            init_response,
        })
    }

    pub fn read_server_cert_hostname(&self) -> Result<String, HelperCommandError> {
        self.init_response.read_server_cert_hostname()
    }

    pub async fn write_credentials(&self) -> Result<&Path, HelperCommandError> {
        tokio::fs::write(
            self.temp_dir.path().join("client-key.pem"),
            self.keypair.serialize_pem(),
        )
        .await
        .whatever_context::<_, HelperCommandError>("write client_key pem")?;
        self.init_response
            .write_certificates(self.temp_dir.path())
            .await?;

        Ok(self.temp_dir.path())
    }
}
