cargo build -Zbuild-std --target x86_64-blog_os.json

# build
cargo build --target x86_64-blog_os.jsonn in que

# to run in qume:
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin

# to run in real machine:
dd if=target/x86_64-blog_os/debug/bootimage-blog_os.bin of=/dev/sdX && sync