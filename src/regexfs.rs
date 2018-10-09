use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::cmp;
use std::path::PathBuf;
use libc::ENOENT;
use libc::c_int;
use time::Timespec;
use regex::Regex;
use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory, ReplyOpen};


const TTL: Timespec = Timespec { sec: 1, nsec: 0 };                     // 1 second

const CREATE_TIME: Timespec = Timespec { sec: 1381237736, nsec: 0 };    // 2013-10-08 08:56

const HELLO_DIR_ATTR: FileAttr = FileAttr {
    ino: 1,
    size: 0,
    blocks: 0,
    atime: CREATE_TIME,
    mtime: CREATE_TIME,
    ctime: CREATE_TIME,
    crtime: CREATE_TIME,
    kind: FileType::Directory,
    perm: 0o444,
    nlink: 0,
    uid: 0,
    gid: 0,
    rdev: 0,
    flags: 0,
};

pub struct RegexFS {
    pub host_dir: PathBuf,
    pub entries: Vec<(u64, String)>,
    pub regex: Regex
}

impl RegexFS {
    pub fn new(host_dir: PathBuf, regex: &str) -> Self {
        RegexFS {
            host_dir,
            entries: vec![],
            regex: Regex::new(regex).unwrap()
        }
    }
    pub fn ino_exists(&self, ino: u64) -> bool {
        ino as usize <= self.entries.len()
    }
    pub fn real_path_for_ino(&self, ino: u64) -> PathBuf {
        let entry = &self.entries[ino as usize];
        let mut path = self.host_dir.clone();
        path.push(&entry.1);
        path
    }

    pub fn gen_attr_for_entry(&self, entry: usize) -> io::Result<FileAttr> {
        let entry = &self.entries[entry];
        let mut path = self.host_dir.clone();
        path.push(&entry.1);
        let metadata = fs::metadata(&path)?;
        Ok(FileAttr {
            ino: entry.0,
            size: metadata.len(),
            blocks: 0,
            atime: CREATE_TIME,
            mtime: CREATE_TIME,
            ctime: CREATE_TIME,
            crtime: CREATE_TIME,
            kind: FileType::RegularFile,
            perm: 0o444,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags: 0,
        })
    }
}


impl Filesystem for RegexFS {
    fn init(&mut self, _req: &Request)  -> Result<(), c_int> {
        let mut entries= vec![];

        entries.push((1, ".".to_string()));
        entries.push((1, "..".to_string()));

        let mut i = 2;
        for entry in fs::read_dir(&self.host_dir).unwrap() {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_file() {
                if self.regex.is_match(entry.file_name().to_string_lossy().as_ref()) {
                    entries.push((i, entry.file_name().to_string_lossy().to_string()));
                    i += 1;
                }
            }
        }
        println!("made a list of {} files", entries.len());
        self.entries = entries;
        Ok(())
    }
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if parent == 1 {
            let entry = &self.entries.binary_search_by_key(&name, |b|b.1.as_ref());
            match entry {
                Ok(ino) => {
                    reply.entry(&TTL, &self.gen_attr_for_entry(*ino).unwrap(), 0);
                },
                Err(_) => {
                    reply.error(ENOENT);
                }
            }
        } else {
            reply.error(ENOENT);

        }
    }
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        match ino {
            1 => reply.attr(&TTL, &HELLO_DIR_ATTR),
            _ => {
                println!("getaddr({}): {:?}", ino, &self.entries[ino as usize]);
                reply.error(ENOENT);
                if ino as usize > self.entries.len() {
                    //reply.error(ENOENT);
                } else {

//                    reply.attr(&TTL, &FileAttr {
//                        ino: ino,
//                        size: 0,
//                        blocks: 0,
//                        atime: CREATE_TIME,
//                        mtime: CREATE_TIME,
//                        ctime: CREATE_TIME,
//                        crtime: CREATE_TIME,
//                        kind: FileType::Directory,
//                        perm: 0o444,
//                        nlink: 2,
//                        uid: 501,
//                        gid: 20,
//                        rdev: 0,
//                        flags: 0,
//                    })
                }
            },
        }
    }

    fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, size: u32, reply: ReplyData) {
        if self.ino_exists(ino) {
            let data = fs::read(self.real_path_for_ino(ino)).unwrap();

            reply.data(&data[(offset as usize)..cmp::min(data.len(), offset as usize+size as usize)]);
        } else {
            reply.error(ENOENT);
        }
    }

    fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
        if ino != 1 {
            reply.error(ENOENT);
            return;
        }

        // Offset of 0 means no offset.
        // Non-zero offset means the passed offset has already been seen, and we should start after
        // it.
        let to_skip = if offset == 0 { offset } else { offset + 1 } as usize;
        for (i, entry) in (&self.entries).into_iter().enumerate().skip(to_skip) {
            if entry.0 == 1 {
                reply.add(1, i as i64, FileType::Directory, entry.1.clone());
            } else {
                reply.add(entry.0, i as i64, FileType::RegularFile, entry.1.clone());
            }
        }
        reply.ok();
    }
}