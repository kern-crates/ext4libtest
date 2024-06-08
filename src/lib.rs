#[allow(unused_imports)]
use ext4_rs::*;

use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

extern crate alloc;

use alloc::vec;

#[cfg(test)]
mod tests {
    use super::*;
    use log::{Level, LevelFilter, Metadata, Record};
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::os::unix::fs::DirBuilderExt;

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

    #[derive(Debug)]
    pub struct Disk {}

    impl BlockDevice for Disk {
        fn read_offset(&self, offset: usize) -> Vec<u8> {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .open("ex4.img")
                .unwrap();
            let mut buf = vec![0u8; BLOCK_SIZE as usize];
            let _r = file.seek(std::io::SeekFrom::Start(offset as u64));
            let _r = file.read_exact(&mut buf);

            buf
        }

        fn write_offset(&self, offset: usize, data: &[u8]) {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .open("ex4.img")
                .unwrap();

            let _r = file.seek(std::io::SeekFrom::Start(offset as u64));
            let _r = file.write_all(&data);
        }
    }

    // #[test]
    // fn test_open() {

    //     let disk = Arc::new(Disk {});
    //     let ext4 = Ext4::open(disk);

    //     let path = "";
    //     let mut ext4_file = Ext4File::new();
    //     let r = ext4.ext4_open_new(&mut ext4_file, path, "r+", false);
    //     assert!(ext4_file.inode == 2);
    //     assert!(r.is_ok(), "open directory error {:?}", r.err());

    //     let path = ".";
    //     let mut ext4_file = Ext4File::new();
    //     let r = ext4.ext4_open_new(&mut ext4_file, path, "r+", false);
    //     assert!(ext4_file.inode == 2);
    //     assert!(r.is_ok(), "open directory error {:?}", r.err());

    //     let path = "./";
    //     let mut ext4_file = Ext4File::new();
    //     let r = ext4.ext4_open_new(&mut ext4_file, path, "r+", false);
    //     assert!(ext4_file.inode == 2);
    //     assert!(r.is_ok(), "open directory error {:?}", r.err());

    //     let path =
    //         "test_files/dirtest0/./dirtest1/../dirtest1/../../dirtest0/dirtest1/dirtest2/dirtest3";
    //     let mut ext4_file = Ext4File::new();
    //     let r = ext4.ext4_open_new(&mut ext4_file, path, "r+", false);
    //     assert!(r.is_ok(), "open directory error {:?}", r.err());

    //     let path = "test_files/dirtest0/./dirtest1/../dirtest1/../../dirtest0/dirtest1/dirtest2/dirtest3/nonexistpath";
    //     let mut ext4_file = Ext4File::new();
    //     let r = ext4.ext4_open_new(&mut ext4_file, path, "r+", false);
    //     assert!(r.is_err());

    // }

    // #[test]
    // fn test_read_file() {

    //     let disk = Arc::new(Disk {});
    //     let ext4 = Ext4::open(disk);

    //     // Test reading the file in ext4_rs
    //     let file_path_str = "test_files/1.txt";
    //     let mut ext4_file = Ext4File::new();
    //     let r = ext4.ext4_open(&mut ext4_file, file_path_str, "r+", false);
    //     assert!(r.is_ok(), "open file error {:?}", r.err());

    //     let mut read_buf = vec![0u8; 0x100000];
    //     let mut read_cnt = 0;
    //     let r = ext4.ext4_file_read(&mut ext4_file, &mut read_buf, 0x100000, &mut read_cnt);
    //     assert!(r.is_ok(), "open file error {:?}", r.err());
    //     let data = [0x31u8; 0x100000];
    //     assert!(read_buf == data);


    // }

    // #[test]
    // fn test_write_file() {

        
    //     let disk = Arc::new(Disk {});
    //     let ext4 = Ext4::open(disk);

    //     // dir
    //     log::info!("----mkdir----");
    //     for i in 0..10 {
    //         let path = format!("dirtest{}", i);
    //         let path = path.as_str();
    //         let r = ext4.ext4_dir_mk(&path);
    //         assert!(r.is_ok(), "dir make error {:?}", r.err());
    //     }

    //     // write test
    //     // file
    //     log::info!("----write file in dir----");
    //     for i in 0..10 {
    //         const WRITE_SIZE: usize = 0x400000;
    //         let path = format!("dirtest{}/write_{}.txt", i, i);
    //         let path = path.as_str();
    //         let mut ext4_file = Ext4File::new();
    //         let r = ext4.ext4_open(&mut ext4_file, path, "w+", true);
    //         assert!(r.is_ok(), "open file error {:?}", r.err());

    //         let write_data = vec![0x41 + i as u8; WRITE_SIZE];
    //         ext4.ext4_file_write(&mut ext4_file, &write_data, WRITE_SIZE);

    //         // test
    //         let r = ext4.ext4_open(&mut ext4_file, path, "r+", false);
    //         assert!(r.is_ok(), "open file error {:?}", r.err());

    //         let mut read_buf = vec![0u8; WRITE_SIZE];
    //         let mut read_cnt = 0;
    //         let r = ext4.ext4_file_read(&mut ext4_file, &mut read_buf, WRITE_SIZE, &mut read_cnt);
    //         assert!(r.is_ok(), "open file error {:?}", r.err());
    //         assert_eq!(write_data, read_buf);
    //     }
    // }
}
