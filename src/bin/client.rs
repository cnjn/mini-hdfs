use mini_hdfs::common::file_service::*;
use tonic::{Request};
use tonic::metadata::KeyAndValueRef::Ascii;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FileServiceClient::connect("http://[::1]:50051").await?;

    println!("client = {:?}", client);

    let resp = client.list_files(Request::new(FileInfo::new("test".to_string(), false))).await?;
    resp.metadata().iter().for_each(|k| {
        match k {
            Ascii(k,v) => println!("{}: {:?}", k, v),
            _ => println!("Unknown metadata type")           
        }
    });

    Ok(())
}