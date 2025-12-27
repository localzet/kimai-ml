use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

pub mod ml_proto {
    tonic::include_proto!("kimai.ml.v1");
}

use ml_proto::ml_processor_server::{MlProcessor, MlProcessorServer};
use ml_proto::{InferRequest, InferResponse};

#[derive(Clone)]
pub struct GrpcServer {}

#[tonic::async_trait]
impl MlProcessor for GrpcServer {
    async fn infer(&self, request: Request<InferRequest>) -> Result<Response<InferResponse>, Status> {
        let req = request.into_inner();
        // Proxy to local HTTP predict endpoint
        let url = format!("http://127.0.0.1:8000/api/predict");
        let client = reqwest::Client::new();
        // Minimal payload: pass options and empty data, ML service can fetch data by user if needed
        let body = serde_json::json!({
            "weeks": [],
            "timesheets": [],
            "settings": {},
            "options": req.options,
        });

        match client.post(&url).json(&body).send().await {
            Ok(resp) => {
                let txt = resp.text().await.unwrap_or_default();
                let out = InferResponse { status: "ok".into(), result_json: txt };
                Ok(Response::new(out))
            }
            Err(e) => Err(Status::internal(format!("proxy error: {}", e))),
        }
    }
}

pub async fn start_grpc_server(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let svc = GrpcServer {};
    Server::builder()
        .add_service(MlProcessorServer::new(svc))
        .serve(addr)
        .await?;
    Ok(())
}
