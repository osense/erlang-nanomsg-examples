### Rust worker

This is a worker written in Rust.


#### Running

```
cargo run
```

Please note that you have to have nanomsg installed on your system. On Arch
this is as easy as pacman -S nanomsg.

#### Todo

When Rust 1.4 moves to stable, replace some of the unwrap()s with expect()s.
