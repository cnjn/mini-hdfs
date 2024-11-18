pub mod name_node;
pub mod data_node;
pub mod common;

#[cfg(test)]
mod test{
    use std::{io::Write, net::SocketAddr};
    use std::fs::File;

    use super::*;
    use common::file_service::FileService;
    use log::{info, warn, error};
    use tonic::transport::Server;
    use common::utils::Sha256Sum;

    #[test]
    fn test_log4rs(){
        log4rs::init_file("log4rs.yml", Default::default())
        .expect("初始化日志系统失败");
        info!("你好哦");
    }

    #[test]
    fn test_tonic(){
        // Server::builder()
        //     .add_service(FileService::new())
        //     .serve(SocketAddr::from(([127, 0, 0, 1], 50051)));
        common::file_service::FileServiceClient::connect("http://[::1]:50051");
    }

    #[test]
    fn test_tarball_and_sha256(){
        let mut digist = Sha256Sum::new();
        {
            let mut archive = tar::Builder::new(&mut digist);
            archive.append_dir_all("src", "src").unwrap();
            archive.finish().unwrap();
        }
        println!("{}", digist.sum());

        // let f = File::create("src.tar").unwrap();
        // let mut archive = tar::Builder::new(f);
        // archive.append_dir_all("src", "src").unwrap();
        // archive.finish().unwrap();
    }
}