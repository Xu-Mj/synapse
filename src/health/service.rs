use std::pin::Pin;

use async_trait::async_trait;
use tonic::codegen::tokio_stream::Stream;
use tonic::{Request, Response, Status};
use tracing::debug;

use crate::pb::health_server::Health;
use crate::pb::ServingStatus;
use crate::pb::{HealthCheckRequest, HealthCheckResponse};

#[derive(Debug, Default)]
pub struct HealthService;

impl HealthService {
    pub fn new() -> Self {
        Self {}
    }
}

/// implement grpc health check
#[async_trait]
impl Health for HealthService {
    async fn check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        debug!("health check request: {:?}", request);
        Ok(Response::new(HealthCheckResponse {
            status: ServingStatus::Serving as i32,
        }))
    }

    type WatchStream =
        Pin<Box<dyn Stream<Item = Result<HealthCheckResponse, Status>> + Send + Sync + 'static>>;

    async fn watch(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        todo!()
    }
}
