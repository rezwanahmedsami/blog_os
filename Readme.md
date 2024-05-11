cargo build -Zbuild-std --target x86_64-blog_os.json
cargo build --target x86_64-blog_os.json

# to run in real machine:
dd if=target/x86_64-blog_os/debug/bootimage-blog_os.bin of=/dev/sdX && sync