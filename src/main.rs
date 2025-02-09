mod enhancement {
    tonic::include_proto!("enhancement");
}

use enhancement::enhancement_service_server::{EnhancementService, EnhancementServiceServer};
use enhancement::{EnhanceItemRequest, EnhanceItemResponse};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct MyEnhancementService {
    // Add any necessary fields here
}

#[tonic::async_trait]
impl EnhancementService for MyEnhancementService {
    async fn enhance_item(
        &self,
        request: Request<EnhanceItemRequest>,
    ) -> Result<Response<EnhanceItemResponse>, Status> {
        let req = request.into_inner();
        // Implement your logic here
        let response = EnhanceItemResponse {
            item: req.item,
            enhanced_item: format!("Enhanced {}", req.item),
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let enhancement_service = MyEnhancementService::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(EnhancementServiceServer::new(enhancement_service))
        .serve(addr)
        .await?;

    Ok(())
}
