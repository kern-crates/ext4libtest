umount tmp
umount foo
rm -rf foo
mkdir foo
sh gen_img.sh
cargo build
target/debug/ext4libtest ./foo