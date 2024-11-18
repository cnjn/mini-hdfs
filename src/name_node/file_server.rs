use crate::common::file_service::*;
use tonic::{Request, Response, Status};


pub type MyFileService = FileService;
#[tonic::async_trait]
impl FileServiceTrait for MyFileService {
    async fn get_file(&self, request: Request<FileInfo>) -> Result<Response<File>, Status> {
        let file_info = request.into_inner();
        Err(Status::unimplemented("Not implemented"))
    }

    async fn put_file(&self, request: Request<File>) -> Result<Response<FileInfo>, Status> {
        let file = request.into_inner();
        Err(Status::unimplemented("Not implemented"))
    }

    async fn delete_file(&self, request: Request<FileInfo>) -> Result<Response<FileInfo>, Status> {
        let file_info = request.into_inner();
        Err(Status::unimplemented("Not implemented"))
    }

    async fn list_files(&self, request: Request<FileInfo>) -> Result<Response<FileInfo>, Status> {
        let file_info = request.into_inner();
        Err(Status::unimplemented("尚未实现列文件功能"))
    }
}


