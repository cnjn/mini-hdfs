mod inner {
    tonic::include_proto!("file_service");
}

pub use inner::{FileInfo, File, DoubleFile};
impl FileInfo {
    pub fn new(file_name: String, is_dir: bool) -> Self {
        FileInfo {
            file_name,
            is_dir,
            ..Default::default()
        }
    }
}

impl File {
    pub fn new() -> Self {
        File {
            file_info: Some(FileInfo::new("test_file".to_string(), true)),
            content: vec![],
        }
    }
}

use tonic::{Request, Response, Status};
pub use inner::file_service_server::FileService as FileServiceTrait;
use inner::file_service_server::FileServiceServer;
#[derive(Debug, Default)]
pub struct FileService;
impl FileService {
    pub fn new() -> FileServiceServer<Self> {
        FileServiceServer::new(Self::default())
    }
}

// #[tonic::async_trait]
// impl FileServiceTrait for FileService {
//     async fn get_file(&self, request: Request<FileInfo>) -> Result<Response<File>, Status> {
//         let file_info = request.into_inner();
//         Err(Status::unimplemented("Not implemented"))
//     }

//     async fn put_file(&self, request: Request<File>) -> Result<Response<FileInfo>, Status> {
//         let file = request.into_inner();
//         Err(Status::unimplemented("Not implemented"))
//     }

//     async fn delete_file(&self, request: Request<FileInfo>) -> Result<Response<FileInfo>, Status> {
//         let file_info = request.into_inner();
//         Err(Status::unimplemented("Not implemented"))
//     }

//     async fn list_files(&self, request: Request<FileInfo>) -> Result<Response<FileInfo>, Status> {
//         let file_info = request.into_inner();
//         Err(Status::unimplemented("Not implemented"))
//     }
// }
pub use inner::file_service_client::FileServiceClient;

