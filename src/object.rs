use crate::repo::GitRepository;
use std::path::Path;

pub struct GitObject {
    // pub worktree: PathBuf,
    // pub gitdir: PathBuf,
    // pub conf: Ini,
}

impl GitObject {
    pub fn New(&self, data: Option<String>) {
        match data {
            None => self.init(),
            Some(data) => self.deserialize(data),
        }
    }

    pub fn serialize(&self, repo: GitRepository) {
        // TODO
    }

    pub fn deserialize(&self, data: String) {
        //TODO
    }

    pub fn init(&self) {
        // TODO ???5
    }

    ///```
    /// use wyag::repo::GitRepository;
    /// use wyag::object::GitObject;
    /// use std::path::Path;
    /// let test = GitRepository::init(&Path::new("C:/Users/colin/Documents/wyag/wyag-rust"), false);
    /// GitObject::object_read(test.unwrap(), "abcdefghij");
    ///```
    pub fn object_read(repo: GitRepository, sha: &str) {
        let mut tmpPath = Path::new("objects");
        if sha.len() < 3 {
            return;
        }

        let path = repo.repo_file(tmpPath
            .join(sha[0..2].to_string())
            .join(sha[2..].to_string()), false);

        println!("CTH - {}", path.unwrap().display());

        // if !path.is_file() {
        //     return
        // }


    }
}


#[cfg(test)]
mod tests {
    // Import the necessary modules
    use crate::repo::GitRepository;
    use crate::object::GitObject;
    use std::path::Path;
    // This test writes to a file
    #[test]
    fn test_file() {
        let test = GitRepository::init(&Path::new("C:/Users/colin/Documents/wyag/wyag-rust"), false);
        GitObject::object_read(test.unwrap(), "abcdefghij");
    }
}


