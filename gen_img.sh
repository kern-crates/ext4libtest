rm -rf test_files
rm -rf tmp
mkdir tmp
mkdir foo
python3 gen_test_files.py
rm -rf ex4.img
dd if=/dev/zero of=ex4.img bs=1M count=8192
mkfs.ext4 ./ex4.img

## create link
cd test_files
sudo ln -s ./1.txt ./linktest
cd ..

## copy files to image
sudo mount ./ex4.img ./tmp/
cd tmp
sudo mkdir -p test_files
sudo cp -r ../test_files/* ./test_files/
cd ../
sudo umount tmp