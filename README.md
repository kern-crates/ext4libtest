# ext4libtest

用于测试 ext4 文件系统。

## 测试环境

- WSL2 Ubuntu22.04
- Rust 版本：`rustc 1.77.0-nightly (nightly-2023-12-28)`

## 运行

   ```bash
   sh run.sh
   ```

## fuse example

```sh
#env
sudo apt install libfuse-dev pkg-config
sudo apt install fuse3 libfuse3-dev
```

```sh
sh fuser.sh
./ext4libtest foo
```

```sh
# Run in another terminal.
cd foo
ls
cd test_files
ls
```
