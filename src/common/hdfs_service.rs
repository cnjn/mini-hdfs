mod inner{
    tonic::include_proto!("hdfs_service");
}

pub use inner::{Block, BlockInfo};