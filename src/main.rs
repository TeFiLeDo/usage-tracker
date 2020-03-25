fn main() {
    // setup human panic crate
    human_panic::setup_panic!(human_panic::Metadata {
        authors: env!("CARGO_PKG_AUTHORS").into(),
        homepage: env!("CARGO_PKG_HOMEPAGE").into(),
        name: env!("CARGO_PKG_NAME").into(),
        version: env!("CARGO_PKG_VERSION").into(),
    });

    panic!("Nothing implemented yet!");
}
