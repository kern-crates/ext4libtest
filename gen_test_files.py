import os


if not os.path.exists("test_files"):
    os.mkdir("test_files")

if not os.path.exists("test_files/dirtest0"):
    os.mkdir("test_files/dirtest0")

if not os.path.exists("test_files/dirtest0/dirtest1"):
    os.mkdir("test_files/dirtest0/dirtest1")

if not os.path.exists("test_files/dirtest0/dirtest1/dirtest2"):
    os.mkdir("test_files/dirtest0/dirtest1/dirtest2")

if not os.path.exists("test_files/dirtest0/dirtest1/dirtest2/dirtest3"):
    os.mkdir("test_files/dirtest0/dirtest1/dirtest2/dirtest3")
    
for i in range(3):
    name = "test_files/"+ str(i) + ".txt"
    f = open(name, "w")
    f.write(str(i)  * 0x100000)
    f.close()