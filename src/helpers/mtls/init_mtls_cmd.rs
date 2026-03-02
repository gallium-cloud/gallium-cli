use rustls::pki_types::CertificateDer;

use crate::helpers::helper_cmd_error::HelperCommandError;
use base64::Engine;
use base64::engine::GeneralPurpose;
use pem::Pem;
use serde::Deserialize;
use snafu::ResultExt;
use std::path::Path;
use tokio::fs;
use x509_parser::prelude::{FromDer, X509Certificate};

const BASE64: GeneralPurpose = base64::engine::general_purpose::STANDARD;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InitMtlsCredentialsCmdResponse {
    pub ca_cert_base64: String,
    pub client_cert_base64: String,
    pub server_cert_base64: String,
}

impl InitMtlsCredentialsCmdResponse {
    pub fn read_server_cert_hostname(&self) -> Result<String, HelperCommandError> {
        let cert = CertificateDer::from(
            BASE64
                .decode(&self.server_cert_base64)
                .whatever_context::<_, HelperCommandError>("server_cert decode base64")?,
        );

        let (_, parsed) = X509Certificate::from_der(cert.as_ref())
            .whatever_context::<_, HelperCommandError>("parse server_cert der")?;

        let attribute = parsed.subject().iter_common_name().next().ok_or_else(|| {
            HelperCommandError::InvalidResponse {
                reason: "server_cert hostname missing",
            }
        })?;

        attribute
            .as_str()
            .map(String::from)
            .whatever_context::<_, HelperCommandError>("server_cert read hostname common_name attr")
    }

    pub async fn write_certificates(&self, client_dir: &Path) -> Result<(), HelperCommandError> {
        let ca_cert_pem = Pem::new(
            "CERTIFICATE",
            BASE64
                .decode(&self.ca_cert_base64)
                .whatever_context::<_, HelperCommandError>("decode ca_cert_base64")?,
        );

        let client_cert_pem = Pem::new(
            "CERTIFICATE",
            BASE64
                .decode(&self.client_cert_base64)
                .whatever_context::<_, HelperCommandError>("decode client_cert_base64")?,
        );

        fs::write(client_dir.join("ca-cert.pem"), ca_cert_pem.to_string()).await?;
        fs::write(
            client_dir.join("client-cert.pem"),
            client_cert_pem.to_string(),
        )
        .await?;

        Ok(())
    }
}
