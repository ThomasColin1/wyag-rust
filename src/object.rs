use crate::repo::GitRepository;
use crate::repo::WyagError;
use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::stdout;
use std::path::Path;

pub struct GitObject {

}

impl GitObject {
    pub fn new(&self, data: Option<String>) {
        match data {
            None => self.init(),
            Some(data) => self.deserialize(data),
        }
    }

    pub fn serialize(&self, _repo: GitRepository) {
        // TODO
    }

    pub fn deserialize(&self, _data: String) {
        //TODO
    }

    pub fn init(&self) {
        // TODO
    }

    pub fn object_read(repo: GitRepository, sha: &str) -> Result<(), WyagError> {
        let tmp_path = Path::new("objects");
        if sha.len() < 3 {
            return Err(WyagError::TODOERROR());
        }

        let path = repo.repo_file(
            tmp_path
                .join(sha[0..2].to_string())
                .join(sha[2..].to_string()),
            false,
        )?;

        if !path.is_file() {
            return Err(WyagError::TODOERROR());
        }

        let file = File::open(path)?;
        let mut decoder = ZlibDecoder::new(file);
        let mut raw: Vec<u8> = Vec::new();
        println!("CTH - 1 ?");
        stdout().flush().unwrap();
        decoder.read_to_end(&mut raw)?;

        
        
        let x: usize = raw
            .iter()
            .position(|&b| b == b' ')
            .ok_or_else(|| WyagError::MalformedObject(sha.to_string()))?;

        let fmt = &raw[0..x];

        let y = raw[x..]
            .iter()
            .position(|&b| b == b'\x00')
            .map(|pos| x + pos)
            .ok_or_else(|| WyagError::MalformedObject(sha.to_string()))?;

        let size_str = std::str::from_utf8(&raw[x + 1..y])?;
        let size: usize = size_str.parse()?;

        let payload = &raw[y + 1..];
        if size != payload.len() {
            return Err(WyagError::MalformedObject(sha.to_string()));
        }

        let _object = match fmt {
            b"commit" => println!("commit"), // TODO
            b"tree" => println!("tree"),
            b"tag" => println!("tag"),
            b"blob" => println!("blob"),
            _ => {
                let type_str = String::from_utf8_lossy(fmt);
                return Err(WyagError::UnknownObjectType(
                    type_str.to_string(),
                    sha.to_string(),
                ));
            }
        };

        return Ok(()); // TODO : return object()
    }
}

#[cfg(test)]
mod tests {
    // Import the necessary modules
    use crate::object::GitObject;
    use crate::repo::GitRepository;
    use flate2::Compression;
    use flate2::write::ZlibEncoder;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    #[test]
    fn test_file() {
        let test = GitRepository::init(&Path::new("/home/colin/Documents/test"), false);
        let _ = GitObject::object_read(test.unwrap(), "00382a8572452980a7f74f0a44d0665a63d0d17a")
            .unwrap();
    }

    /*   #[test]
    fn test_write() -> Result<(), Box<dyn std::error::Error>>  {
        let data = b"Hello, world! This is compressed data using zlib in Rust.";

        // Create the output file
        let file = File::create("output.txt.zlib")?;

        // Wrap the file inside a ZlibEncoder with the desired compression level
        let mut encoder = ZlibEncoder::new(file, Compression::default());

        // Write data to the encoder
        encoder.write_all(data)?;

        // Finish compressing and write out remaining bytes
        encoder.finish()?;
        Ok(())
    } */
}
