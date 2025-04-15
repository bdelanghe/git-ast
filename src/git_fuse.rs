use std::ffi::OsStr;
use std::time::{Duration, UNIX_EPOCH};
use std::collections::HashMap;

use fuser::{
    Filesystem,
    Request,
    ReplyData,
    ReplyEntry,
    ReplyAttr,
    ReplyDirectory,
    FileType,
    FileAttr,
    FUSE_ROOT_ID,
    KernelConfig,
};
use git2::{Repository, Oid};
use libc::{ENOENT, EPERM};

const TTL: Duration = Duration::from_secs(1); // 1 second TTL

// Helper function to create basic directory attributes
fn dir_attr(ino: u64) -> FileAttr {
    FileAttr {
        ino,
        size: 0,
        blocks: 0,
        atime: UNIX_EPOCH, // Placeholder
        mtime: UNIX_EPOCH,
        ctime: UNIX_EPOCH,
        crtime: UNIX_EPOCH,
        kind: FileType::Directory,
        perm: 0o755,
        nlink: 2, // Standard for directories (. and ..)
        uid: 501, // TODO: Get actual uid
        gid: 20,  // TODO: Get actual gid
        rdev: 0,
        flags: 0,
        blksize: 512, // Block size
    }
}

// Our filesystem structure
pub struct GitFS {
    repo_path: String,
    repo: Option<Repository>,
    inodes: HashMap<u64, Oid>,
    oids: HashMap<Oid, u64>,
    #[allow(dead_code)] // Will be used later
    next_inode: u64,
}

impl GitFS {
    pub fn new(repo_path: String) -> Self {
        GitFS {
            repo_path,
            repo: None,
            inodes: HashMap::new(),
            oids: HashMap::new(),
            next_inode: FUSE_ROOT_ID + 1,
        }
    }

    // Internal helper for initialization logic, testable without FUSE context
    fn perform_initialization(&mut self) -> Result<(), git2::Error> {
        println!("Performing initialization for repo: {}", self.repo_path);
        let repo = Repository::open(&self.repo_path)?;
        println!("Repository opened successfully.");

        // Get the root tree OID within a separate scope to drop borrows before moving repo
        let root_oid = {
            let head = repo.head()?;
            let commit = head.peel_to_commit()?;
            let tree = commit.tree()?;
            tree.id() // This clones the OID, original tree/commit/head can be dropped
        };
        println!("Root tree OID: {}", root_oid);

        self.inodes.insert(FUSE_ROOT_ID, root_oid);
        self.oids.insert(root_oid, FUSE_ROOT_ID);

        // Now we can safely move the repo, as borrows (head, commit, tree) are dropped
        self.repo = Some(repo);
        println!("Initialization complete.");
        Ok(())
    }

    // Helper to get a reference to the repository
    #[allow(dead_code)] // Will be used later
    fn repo(&self) -> &Repository {
        self.repo.as_ref().expect("Repository not initialized")
    }
}

// Implement the Filesystem trait for GitFS
impl Filesystem for GitFS {
    // `init` is called when the filesystem is mounted.
    // Updated signature to match the trait
    fn init(&mut self, _req: &Request<'_>, _config: &mut KernelConfig) -> Result<(), libc::c_int> {
        // Call the internal helper method
        self.perform_initialization().map_err(|e| {
            eprintln!(
                "Failed to initialize GitFS for repo '{}': {}",
                self.repo_path,
                e
            );
            EPERM // Return permission error on failure
        })
    }

    // `lookup` finds a directory entry by name.
    fn lookup(&mut self, _req: &Request, _parent: u64, _name: &OsStr, reply: ReplyEntry) {
        println!("lookup(parent={}, name={:?})", _parent, _name);
        reply.error(ENOENT); // Default: Not found
    }

    // `getattr` gets file attributes.
    // Updated signature to match the trait
    fn getattr(&mut self, _req: &Request, ino: u64, _fh: Option<u64>, reply: ReplyAttr) {
        println!("getattr(ino={}, fh={:?})", ino, _fh);
        if ino == FUSE_ROOT_ID {
            println!("getattr: Found root inode ({})", ino);
            reply.attr(&TTL, &dir_attr(ino));
        } else {
            println!("getattr: Inode {} not found (yet)", ino);
            reply.error(ENOENT); // Default: Not found
        }
    }

    // `read` reads data from a file.
    fn read(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        _size: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: ReplyData,
    ) {
        println!(
            "read(ino={}, fh={}, offset={}, size={})",
            _ino, _fh, _offset, _size
        );
        reply.error(ENOENT); // Default: Not found or not implemented
    }

    // `readdir` reads entries from a directory.
    fn readdir(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        reply: ReplyDirectory,
    ) {
        println!("readdir(ino={}, fh={}, offset={})", _ino, _fh, _offset);
        reply.error(ENOENT); // Default: Not found or not implemented
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use git2::{Repository, Signature};

    #[test]
    fn test_gitfs_perform_initialization() { // Renamed test
        // 1. Create a temporary directory and initialize a bare Git repo
        let td = tempdir().expect("Failed to create temp dir");
        let repo_path = td.path();
        let repo = Repository::init_bare(repo_path).expect("Failed to init bare repo");

        // 2. Create an initial empty commit to establish HEAD
        {
            let signature = Signature::now("Test User", "test@example.com")
                              .expect("Failed to create signature");
            let tree_id = repo.treebuilder(None).expect("Failed to create treebuilder")
                            .write().expect("Failed to write empty tree");
            let tree = repo.find_tree(tree_id).expect("Failed to find empty tree");
            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                "Initial empty commit",
                &tree,
                &[],
            ).expect("Failed to create initial commit");
        }
        println!("Temporary repo created at: {:?}", repo_path);

        // 3. Instantiate GitFS
        let mut gitfs = GitFS::new(repo_path.to_str().unwrap().to_string());

        // 4. Call the internal initialization method directly
        let init_result = gitfs.perform_initialization();

        // 5. Assert success and internal state
        assert!(init_result.is_ok(), "perform_initialization failed: {:?}", init_result.err());
        assert!(gitfs.repo.is_some(), "Repository should be initialized");
        assert!(gitfs.inodes.contains_key(&FUSE_ROOT_ID), "Root inode should be mapped");
        assert_eq!(gitfs.oids.len(), 1, "Only root OID should be mapped initially");
        assert_eq!(gitfs.inodes.len(), 1, "Only root inode should be mapped initially");
        println!("perform_initialization test completed successfully.");
    }
} 
