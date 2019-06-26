# Hello Rust
Make sure you have: 
- Rust [installed][rustInstall]
- Nightly Rust channel
- VS Code with [Rust Extension Pack][vsExtPck]

![crabby](https://mir-s3-cdn-cf.behance.net/project_modules/disp/7df0bd42774743.57ee5f32bd76e.gif)

# Start
From your VSCode terminal:
```sh
$ rustup override set nightly
$ cargo build
     Finished dev [unoptimized + debuginfo] target(s) in 0.36s
$ cargo run
```

# Todo
- [x] Stop parsing odd/even/heck.
- [x] Respond over HTTP.
- [ ] Input list of URLs to crawl.
- [ ] Dynamic parsing, use css-selector technique to effectively find links in any given URL.

[rustInstall]: <https://www.rust-lang.org/tools/install>
[vsExtPck]: <https://marketplace.visualstudio.com/items?itemName=swellaby.rust-pack>
