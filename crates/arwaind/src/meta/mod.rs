use arwain_proto::meta::{meta_service_server, GetVersionRequest, GetVersionResponse};

#[derive(Debug, Default)]
pub struct MetaService {}

#[tonic::async_trait]
impl meta_service_server::MetaService for MetaService {
    async fn get_version(
        &self,
        _request: tonic::Request<GetVersionRequest>,
    ) -> Result<tonic::Response<GetVersionResponse>, tonic::Status> {
        Ok(tonic::Response::new(GetVersionResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
            platform: format!("{}/{}", std::env::consts::OS, std::env::consts::ARCH),
        }))
    }
}