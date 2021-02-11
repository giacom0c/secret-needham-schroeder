# Description

This is an experimental implementation of the Needham-Schroeder protocol using
[Secret Network](https://github.com/enigmampc/SecretNetwork), written in Rust.

## How to run

Assuming you have a recent version of rust and cargo installed (via [rustup](https://rustup.rs/)),
then the following should get you a new repo to start a contract:

First, install
[cargo-generate](https://github.com/ashleygwilliams/cargo-generate).
Unless you did that before, run this line now:

```sh
cargo install cargo-generate --features vendored-openssl
```

Now, download the project.
Go to the folder in which you want to place it and run:

```sh
cargo generate --git https://github.com/enigmampc/secret-template.git --name YOUR_NAME_HERE
```

You will now have a new folder called `YOUR_NAME_HERE` (I hope you changed that to something else)
containing the project.
You will also need docker to run locally the Secret Contract.
