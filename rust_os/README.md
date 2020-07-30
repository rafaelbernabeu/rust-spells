# rust-os

```
$ cargo install bootimage
$ cargo build
$ cargo bootimage
```

```
$ qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust_os.bin
```

[Blog_Os](https://github.com/phil-opp/blog_os)