use mini_hdfs::name_node::file_server::*;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    Server::builder()
        .add_service(MyFileService::new())
        .serve(addr)
        .await?;
    Ok(())
}