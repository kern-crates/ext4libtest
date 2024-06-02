use ext4_rs::*;

extern crate alloc;
pub use alloc::sync::Arc;
use clap::{crate_version, Arg, ArgAction, Command};
use fuser::{
    FileAttr, FileType, Filesystem, MountOption, ReplyAttr, ReplyData, ReplyDirectory, ReplyEmpty,
    ReplyEntry, ReplyWrite, Request,
};
use log::{Level, LevelFilter, Metadata, Record};

use std::ffi::OsStr;
use std::time::{Duration, UNIX_EPOCH};

macro_rules! with_color {
    ($color_code:expr, $($arg:tt)*) => {{
        format_args!("\u{1B}[{}m{}\u{1B}[m", $color_code as u8, format_args!($($arg)*))
    }};
}

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        let level = record.level();
        let args_color = match level {
            Level::Error => ColorCode::Red,
            Level::Warn => ColorCode::Yellow,
            Level::Info => ColorCode::Green,
            Level::Debug => ColorCode::Cyan,
            Level::Trace => ColorCode::BrightBlack,
        };

        if self.enabled(record.metadata()) {
            println!(
                "{} - {}",
                record.level(),
                with_color!(args_color, "{}", record.args())
            );
        }
    }

    fn flush(&self) {}
}

#[repr(u8)]
enum ColorCode {
    Red = 31,
    Green = 32,
    Yellow = 33,
    Cyan = 36,
    BrightBlack = 90,
}

pub const EPERM: i32 = 1;
pub const ENOENT: i32 = 2;
pub const ESRCH: i32 = 3;
pub const EINTR: i32 = 4;
pub const EIO: i32 = 5;
pub const ENXIO: i32 = 6;
pub const E2BIG: i32 = 7;
pub const ENOEXEC: i32 = 8;
pub const EBADF: i32 = 9;
pub const ECHILD: i32 = 10;
pub const EAGAIN: i32 = 11;
pub const ENOMEM: i32 = 12;
pub const EACCES: i32 = 13;
pub const EFAULT: i32 = 14;
pub const ENOTBLK: i32 = 15;
pub const EBUSY: i32 = 16;
pub const EEXIST: i32 = 17;
pub const EXDEV: i32 = 18;
pub const ENODEV: i32 = 19;
pub const ENOTDIR: i32 = 20;
pub const EISDIR: i32 = 21;
pub const EINVAL: i32 = 22;
pub const ENFILE: i32 = 23;
pub const EMFILE: i32 = 24;
pub const ENOTTY: i32 = 25;
pub const ETXTBSY: i32 = 26;
pub const EFBIG: i32 = 27;
pub const ENOSPC: i32 = 28;
pub const ESPIPE: i32 = 29;
pub const EROFS: i32 = 30;
pub const EMLINK: i32 = 31;
pub const EPIPE: i32 = 32;
pub const EDOM: i32 = 33;
pub const ERANGE: i32 = 34;
pub const EWOULDBLOCK: i32 = EAGAIN;

const TTL: Duration = Duration::from_secs(1); // 1 second

#[derive(Debug)]
pub struct Disk {}

impl BlockDevice for Disk {
    fn read_offset(&self, offset: usize) -> Vec<u8> {
        // log::info!("read_offset: {:x?}", offset);
        use std::fs::OpenOptions;
        use std::io::{Read, Seek};
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("ex4.img")
            .unwrap();
        let mut buf = vec![0u8; BLOCK_SIZE as usize];
        let r = file.seek(std::io::SeekFrom::Start(offset as u64));
        let r = file.read_exact(&mut buf);

        buf
    }

    fn write_offset(&self, offset: usize, data: &[u8]) {
        use std::fs::OpenOptions;
        use std::io::{Read, Seek, Write};
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("ex4.img")
            .unwrap();

        let r = file.seek(std::io::SeekFrom::Start(offset as u64));
        let r = file.write_all(&data);
    }
}

struct Ext4Fuse {
    ext4: Arc<Ext4>,
}

impl Ext4Fuse {
    pub fn new(ext4: Arc<Ext4>) -> Self {
        Self { ext4: ext4 }
    }
}

impl Filesystem for Ext4Fuse {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        let path = name.to_str().unwrap();
        log::info!("lookup: {:?}", path);
        let mut file = Ext4File::new();
        let result = self.ext4.ext4_open(&mut file, path, "r", false);

        // let mut inode_ref = Ext4InodeRef::get_inode_ref(self.ext4.self_ref.clone(), file.inode as u32);
        // let mode = inode_ref.inner.inode.mode;
        // let inode_type = InodeMode::from_bits(mode & EXT4_INODE_MODE_TYPE_MASK as u16).unwrap();

        // let file_type = match inode_type {
        //     InodeMode::S_IFDIR => {
        //         FileType::Directory
        //     }
        //     InodeMode::S_IFREG => {
        //         FileType::RegularFile
        //     }
        //     /* Reset blocks array. For inode which is not directory or file, just
        //      * fill in blocks with 0 */
        //     _ => {
        //         FileType::RegularFile
        //     }
        // };

        match result {
            Ok(_) => {

                log::info!("open success: {:x?}", file.inode);
                let attr = FileAttr {
                    ino: file.inode as u64,
                    size: file.fsize,
                    blocks: file.fsize / BLOCK_SIZE as u64,
                    atime: UNIX_EPOCH,
                    mtime: UNIX_EPOCH,
                    ctime: UNIX_EPOCH,
                    crtime: UNIX_EPOCH,
                    // fix me
                    kind: FileType::Directory,
                    perm: 0o644,
                    nlink: 1,
                    uid: 501,
                    gid: 20,
                    rdev: 0,
                    flags: 0,
                    blksize: BLOCK_SIZE as u32,
                };
                reply.entry(&TTL, &attr, 0);
            }
            Err(_) => reply.error(ENOENT),
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, _fh: Option<u64>, reply: ReplyAttr) {
        let inode = match ino {
            // root
            1 => 2,
            _ => ino,
        };

        log::info!("getattr: {:x?}", inode);
        let mut inode_ref = Ext4InodeRef::get_inode_ref(self.ext4.self_ref.clone(), inode as u32);
        let link_cnt = inode_ref.inner.inode.ext4_inode_get_links_cnt() as u32;
        let mode = inode_ref.inner.inode.mode;
        let inode_type = InodeMode::from_bits(mode & EXT4_INODE_MODE_TYPE_MASK as u16).unwrap();
        let file_type = match inode_type {
            InodeMode::S_IFDIR => {
                FileType::Directory
            }
            InodeMode::S_IFREG => {
                FileType::RegularFile
            }
            /* Reset blocks array. For inode which is not directory or file, just
             * fill in blocks with 0 */
            _ => {
                FileType::RegularFile
            }
        };

        let attr = FileAttr {
            ino: inode,
            size: inode_ref.inner.inode.inode_get_size() as u64,
            blocks: inode_ref.inner.inode.inode_get_size() / BLOCK_SIZE as u64,
            atime: UNIX_EPOCH, // Example static time, adjust accordingly
            mtime: UNIX_EPOCH,
            ctime: UNIX_EPOCH,
            crtime: UNIX_EPOCH,
            kind: file_type, // Adjust according to inode type
            perm: 0o777,               // Need a method to translate inode perms to Unix perms
            nlink: link_cnt,
            uid: 501,
            gid: 20,
            rdev: 0, // Device nodes not covered here
            flags: 0,
            blksize: BLOCK_SIZE as u32, 
        };
        reply.attr(&TTL, &attr);
    }

    fn read(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        size: u32,
        _flags: i32,
        _lock: Option<u64>,
        reply: ReplyData,
    ) {
        log::info!("-----------read-----------");
        let mut file = Ext4File::new();
        file.inode = ino as u32;
        file.fpos = offset as usize;

        let mut data = vec![0u8; size as usize];
        let mut read_cnt = 0;
        let result = self
            .ext4
            .ext4_file_read(&mut file, &mut data, size as usize, &mut read_cnt);

        match result {
            Ok(_) => reply.data(&data),
            Err(_) => reply.error(ENOENT), // Adjust error handling as needed
        }
    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        let inode = match ino {
            // root
            1 => 2,
            _ => ino,
        };
        log::info!("-----------readdir-----------inode {:x?}", ino);
        let entries = self.ext4.read_dir_entry(inode);
        for (i, entry) in entries.iter().enumerate().skip(offset as usize) {
            let name = get_name(entry.name, entry.name_len as usize).unwrap();
            let detype = entry.get_de_type();
            let kind = match detype {
                1 => FileType::RegularFile,
                2 => FileType::Directory,
                _ => FileType::RegularFile,
            };
            reply.add(entry.inode as u64, (i + 1) as i64, kind, &name);
        }
        reply.ok();
    }

    fn write(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        data: &[u8],
        write_flags: u32,
        flags: i32,
        lock_owner: Option<u64>,
        reply: ReplyWrite,
    ) {
        let mut file = Ext4File::new();
        file.inode = ino as u32;
        file.fpos = offset as usize;

        self.ext4.ext4_file_write(&mut file, data, data.len());
        reply.written(data.len() as u32)
    }

    /// Remove a file.
    fn unlink(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        let path = name.to_str().unwrap_or_default();
        let mut parent_ref = Ext4InodeRef::get_inode_ref(self.ext4.self_ref.clone(), parent as u32);
        let mut child = Ext4File::new();
        let open_result = self.ext4.ext4_open(&mut child, path, "r", false);

        if let Ok(_) = open_result {
            let mut child_ref =
                Ext4InodeRef::get_inode_ref(self.ext4.self_ref.clone(), child.inode);
            let result =
                self.ext4
                    .ext4_unlink(&mut parent_ref, &mut child_ref, path, path.len() as u32);
            match result {
                EOK => reply.ok(),
                _ => reply.error(EIO),
            }
        } else {
            reply.error(ENOENT);
        }
    }
}

fn main() {
    log::set_logger(&SimpleLogger).unwrap();
    log::set_max_level(LevelFilter::Info);

    let disk = Arc::new(Disk {});
    let ext4 = Ext4::open(disk);
    let ext4_fuse = Ext4Fuse::new(ext4);
    let mountpoint = "foo";
    let mut options = vec![
        MountOption::RW,
        MountOption::FSName("ext4_test".to_string()),
    ];
    fuser::mount2(ext4_fuse, mountpoint, &options).unwrap();
}
