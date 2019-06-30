# Hello Rust
Make sure you have: 
- Rust [installed][rustInstall]
- Nightly Rust channel
- VS Code with [Rust Extension Pack][vsExtPck]

Optional:
- [Docker](https://hub.docker.com/)

![crabby](https://mir-s3-cdn-cf.behance.net/project_modules/disp/7df0bd42774743.57ee5f32bd76e.gif)

## Start

From your VSCode terminal:

```sh
$ rustup override set nightly
$ cargo build
     Finished dev [unoptimized + debuginfo] target(s) in 0.36s
$ cargo run
```

## Dockerized version

### Build

Using make:

```make
make build-docker-latest
```

or manual:

```sh
docker build -t hello-rust:latest .
```

### Run

Using make:

```make
make run-docker-latest
```

or manual:

```sh
docker run -p 8000:8000 hello-rust
````

Then navigate to <http://localhost:8000> to access the app.

## Todo

- [x] Dockerize app.
- [x] Stop parsing odd/even/heck and start doing random programming questions from [Cracking The Coding Interview][interviewcrack]
- [x] Respond over HTTP.
- [ ] Input list of URLs to crawl.
- [ ] Dynamic parsing, use css-selector technique to effectively find links in any given URL.

[rustInstall]: <https://www.rust-lang.org/tools/install>
[vsExtPck]: <https://marketplace.visualstudio.com/items?itemName=swellaby.rust-pack>
[interviewcrack]: <https://leonmercanti.com/books/personal-development/Cracking%20the%20Coding%20Interview%20189%20Programming%20Questions%20and%20Solutions.pdf>
