use super::*;

#[test]
fn test_open() {
    let disk = Arc::new(Disk {});
    let ext4 = Ext4::open(disk);

    let path = ".";
    let r = ext4.ext4_file_open(path, "r+");
    assert!(r.unwrap() == 2);
    assert!(r.is_ok(), "open directory error {:?}", r.err());

    let path = "./";
    let r = ext4.ext4_file_open(path, "r+");
    assert!(r.unwrap() == 2);
    assert!(r.is_ok(), "open directory error {:?}", r.err());

    let path =
        "test_files/dirtest0/./dirtest1/../dirtest1/../../dirtest0/dirtest1/dirtest2/dirtest3";
    let r = ext4.ext4_file_open(path, "r+");
    assert!(r.is_ok(), "open directory error {:?}", r.err());

    let path = "test_files/dirtest0/./dirtest1/../dirtest1/../../dirtest0/dirtest1/dirtest2/dirtest3/nonexistpath";
    let r = ext4.ext4_file_open(path, "r+");
    assert!(r.is_err());
}

#[test]
fn test_file_write_and_read_random_data() {
    let disk = Arc::new(Disk {});
    let ext4 = Ext4::open(disk);

    use rand::Rng;
    
    // 创建一个 1GB 的文件并写入随机数据
    const FILE_SIZE: usize = 1 * 1024 * 1024 * 1024; // 1G
    let path = "large_file_random.txt";
    let flags = "w+";
    let inode_num = ext4.ext4_file_open(path, flags).unwrap();
    
    let mut rng = rand::thread_rng();
    
    // 写入和验证逐块数据
    for i in 0..(FILE_SIZE / BLOCK_SIZE) {
        // 生成随机数据
        let mut write_data = vec![0u8; BLOCK_SIZE];
        rng.fill(&mut write_data[..]);
    
        // 计算偏移量
        let offset = (i * BLOCK_SIZE) as i64;
    
        // 写入数据
        let write_size = ext4
            .ext4_file_write(inode_num as u64, offset, &write_data)
            .unwrap();
        assert_eq!(
            write_size, BLOCK_SIZE,
            "Failed to write block at offset {} (expected {} bytes, got {})",
            offset, BLOCK_SIZE, write_size
        );
    
        // 读取数据
        let read_data = ext4
            .ext4_file_read(inode_num as u64, BLOCK_SIZE as u32, offset)
            .unwrap();
        assert_eq!(
            read_data, write_data,
            "Data mismatch at block {:x} (offset {:x}).",
            i, offset
        );
    
        if i % 1024 == 0 {
            log::info!(
                "Verified {:?} MB written and read back successfully",
                i * 4 / 1024
            );
        }
    }
}

