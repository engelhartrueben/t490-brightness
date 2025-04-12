<div align="center">
<b>T490-Brightness</b>

*A Rust built, simple controller for your T490 brightness*
</div>

> [!WARNING]
> T490-Brightness is still very much in its early stages. There are no stable releases at the moment.

## About
**T490-Brightness** is designed with a explicit distinction between policy and mechanism. This **program** is a mechanism, and the command line arguments the user passes are the **policy**.

This is used in my Arch set-up, and has not been tested on any other flavor of linux.

## Set-up
1. Clone this repoitory
2. Run `cd /t490-brightness`
3. Build by running `cargo build --release`
4. Change permissions of binary via `chown root:root /target/release/t490-brightness` and `chmod u+s /target/release/t490-brightness`
5. From here, I have it added to by i3 configurage file, where I have these set:
```terminal
bindsym XF86AudioRaiseVolume exec <path_to_t490-brightness_binary> -u 2000 -ma 24242
bindsym XF86AudioLowerVolume exec <path_to_t490-brightness_binary> -d 2000 -mi 4000
```
6. Remember to refresh you i3

## Current todo items
- Add functionality for `-maf` flag that takes the max brightness specified in you `/sys/` directory. 
