use std::io::Write;
use sha2::{Sha256, Digest};



// 计算tar文件的sha256
pub struct Sha256Sum {
    hasher: Sha256,
}
impl Sha256Sum {
    pub fn new() -> Self {
        Sha256Sum {
            hasher: Sha256::new(),
        }
    }

    pub fn sum(self) -> String {
        format!("{:x}", self.hasher.finalize())
    }
}

impl Write for Sha256Sum {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.hasher.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}