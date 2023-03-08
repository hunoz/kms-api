## Documentation
This package is called KMS, short for Kubernetes Management System. This package will allow automatic provisioning of a new Kubernetes machine. It will store each device in a DynamoDB table, which your choice of automation can pick up and use (e.g. a Kickstart server that creates its files dynamically based on this information).

To run this application with hot-reloading, run `cargo install cargo-watch && cargo watch -x run`
To include logging, prepend the above command with `RUST_LOG=debug` where debug is one of `trace`, `debug`, `info`, `warn`, or `error`


## Compiling
To compile from Mac to Linux x86-64, perform the following commands:
1. rustup target add x86_64-unknown-linux-gnu
2. brew tap SergioBenitez/osxct
3. brew install x86_64-unknown-linux-gnu
4. export CC_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-gcc 
5. export CXX_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-g++
6. export AR_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-ar
7. export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc
8. cargo build -r --target x86_64-unknown-linux-gnu


If you wish to test execution in a Linux environment, perform the above section and then use Docker to run it with the following commands:
`docker run --rm -v $(pwd)/config:/tmp/config -v $(pwd)/target/x86_64-unknown-linux-gnu/release/kms:/tmp/kms -p 8080:8080 fedora bash -c "cd /tmp && RUST_LOG=debug ./kms"`
`curl http://127.0.0.1:8080` with the route you wish to test