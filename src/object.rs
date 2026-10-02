use crate::repo::GitRepository;
use crate::repo::WyagError;
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub trait GitObject: Sized {
    const FMT: &'static [u8];
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Result<Self, WyagError>;
}

pub struct GitBlob {
    pub data: Vec<u8>,
}
pub struct GitCommit {
    pub data: Vec<u8>,
}
pub struct GitTree {
    pub data: Vec<u8>,
}
pub struct GitTag {
    pub data: Vec<u8>,
}

impl GitObject for GitBlob {
    const FMT: &'static [u8] = b"blob";

    fn serialize(&self) -> Vec<u8> {
        self.data.clone()
    }

    fn deserialize(data: &[u8]) -> Result<Self, WyagError> {
        Ok(GitBlob {
            data: data.to_vec(),
        })
    }
}

impl GitObject for GitCommit {
    const FMT: &'static [u8] = b"commit";

    fn serialize(&self) -> Vec<u8> {
        todo!()
    }

    fn deserialize(_data: &[u8]) -> Result<Self, WyagError> {
        todo!()
    }
}

impl GitObject for GitTree {
    const FMT: &'static [u8] = b"tree";

    fn serialize(&self) -> Vec<u8> {
        todo!()
    }

    fn deserialize(_data: &[u8]) -> Result<Self, WyagError> {
        todo!()
    }
}

impl GitObject for GitTag {
    const FMT: &'static [u8] = b"tag";

    fn serialize(&self) -> Vec<u8> {
        todo!()
    }

    fn deserialize(_data: &[u8]) -> Result<Self, WyagError> {
        todo!()
    }
}

pub enum Object {
    Blob(GitBlob),
    Commit(GitCommit),
    Tree(GitTree),
    Tag(GitTag),
}

impl Object {
    pub fn fmt(&self) -> &'static [u8] {
        match self {
            Object::Blob(_) => GitBlob::FMT,
            Object::Commit(_) => GitCommit::FMT,
            Object::Tree(_) => GitTree::FMT,
            Object::Tag(_) => GitTag::FMT,
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            Object::Blob(o) => o.serialize(),
            Object::Commit(o) => o.serialize(),
            Object::Tree(o) => o.serialize(),
            Object::Tag(o) => o.serialize(),
        }
    }
}

pub fn object_read(repo: &GitRepository, sha: &str) -> Result<Object, WyagError> {
    let tmp_path = Path::new("objects");
    if sha.len() < 3 {
        return Err(WyagError::TODOERROR());
    }

    let path = repo.repo_file(tmp_path.join(&sha[0..2]).join(&sha[2..]), false)?;

    if !path.is_file() {
        return Err(WyagError::TODOERROR());
    }

    let file = File::open(path)?;
    let mut decoder = ZlibDecoder::new(file);
    let mut raw: Vec<u8> = Vec::new();

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

    Ok(match fmt {
        b"blob" => Object::Blob(GitBlob::deserialize(payload)?),
        b"commit" => Object::Commit(GitCommit::deserialize(payload)?),
        b"tree" => Object::Tree(GitTree::deserialize(payload)?),
        b"tag" => Object::Tag(GitTag::deserialize(payload)?),
        _ => {
            return Err(WyagError::UnknownObjectType(
                String::from_utf8_lossy(fmt).into_owned(),
                sha.to_string(),
            ));
        }
    })
}

pub fn object_write(obj: Object, repo: Option<&GitRepository>) -> Result<String, WyagError> {
    let data = obj.serialize();

    let mut result = Vec::new();
    result.extend_from_slice(obj.fmt());
    result.push(b' ');
    result.extend_from_slice(data.len().to_string().as_bytes());
    result.push(0);
    result.extend_from_slice(&data);

    let sha = hex::encode(Sha1::digest(&result));

    if let Some(repo) = repo {
        let path = repo.repo_file(Path::new("objects").join(&sha[..2]).join(&sha[2..]), true)?;
        if !path.exists() {
            let mut file = ZlibEncoder::new(File::create(path)?, Compression::default());
            file.write_all(&result)?;
            file.finish()?;
        }
    }

    Ok(sha)
}

#[cfg(test)]
mod tests {
    use crate::object::{GitBlob, Object, object_read, object_write};
    use crate::repo::GitRepository;
    #[test]
    fn test_file() {
        let dir = tempfile::tempdir().unwrap();
        let repo = GitRepository::repo_create(dir.path()).unwrap();

        let obj = Object::Blob(GitBlob {
            data: b"le test (le test) (c'est le texte de test) LETEST.test".to_vec(),
        });
        let sha = object_write(obj, Some(&repo)).unwrap();
        assert_eq!(sha, "1ebe67504c5749a4b9dd6ed8e28fbd469ff4798b");

        match object_read(&repo, &sha).unwrap() {
            Object::Blob(b) => assert_eq!(
                b.data,
                b"le test (le test) (c'est le texte de test) LETEST.test"
            ),
            _ => panic!("wrong type"),
        }
    }
}
