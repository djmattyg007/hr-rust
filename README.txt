hr-rust

This is a port of hr to rust, as a learning exercise for rust.

You can find my C version of hr here:
https://github.com/djmattyg007/hr

This is a very simple program, so just using 'cargo build' is enough to build
it - no flags are necessary. If you're building for packaging purposes, I
recommend the following command:

cargo build --release --locked

However for packaging you probably want this command:

cargo install --no-track --locked --all-features --root "${pkgdir}/usr/" --path .

This program is released into the public domain without any warranty.
