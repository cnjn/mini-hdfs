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

    async fn create_dir(&self, request: Request<FileInfo>) -> Result<Response<()>, Status> {
        let file_info = request.into_inner();
        Err(Status::unimplemented("Not implemented"))
    }

    async fn get_file_info(&self, request: Request<FileInfo>) -> Result<Response<FileInfo>, Status> {
        let file_info = request.into_inner();
        Err(Status::unimplemented("Not implemented"))
    }

    async fn copy_file(&self, request: Request<DoubleFile>) -> Result<Response<()>, Status> {
        let files = request.into_inner();
        Err(Status::unimplemented("Not implemented"))
    }

    async fn move_file(&self, request: Request<DoubleFile>) -> Result<Response<()>, Status> {
        let files = request.into_inner();
        Err(Status::unimplemented("Not implemented"))
    }
}


