# ext4libtest

用于测试 ext4 文件系统。

## 测试环境

- WSL2 Ubuntu22.04
- Rust 版本：`rustc 1.77.0-nightly (nightly-2023-12-28)`

## fuse example

```sh
#env
sudo apt install libfuse-dev pkg-config
sudo apt install fuse3 libfuse3-dev
```

```sh
sh gen_img.sh
#cargo run /path/to/mountpoint
cargo run ./foo/
```

```sh
# Run in another terminal.
cd foo
ls
cd test_files
ls
touch test_file_create
rm -rf test_file_create
mkdir test_dir_mk
rm -rf test_dir_mk
echo "AAAAAAAA" > test_write
cat test_write
cat 0.txt
```
