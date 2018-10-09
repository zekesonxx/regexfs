extern crate env_logger;
extern crate fuse;
extern crate libc;
extern crate time;
extern crate regex;

use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;
use libc::ENOENT;
use time::Timespec;
use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};

mod regexfs;

fn main() {
    env_logger::init();
    let mountpoint = env::args_os().nth(3).unwrap();
    let options = ["-o", "ro"]
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();
    let mut filesystem = regexfs::RegexFS::new(PathBuf::from(env::args_os().nth(1).unwrap()), env::args_os().nth(2).unwrap().to_string_lossy().as_ref());
    fuse::mount(filesystem, &mountpoint, &options).unwrap();
}