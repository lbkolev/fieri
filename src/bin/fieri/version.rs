/// The long version information for fieri.
///
/// - The latest version from Cargo.toml
/// - The long SHA of the latest commit.
/// - The build datetime
/// - The build features
///
/// # Example:
///
/// ```text
/// Version: 0.1.0
/// Commit SHA: defa64b2
/// Build Timestamp: 2023-05-19T01:47:19.815651705Z
/// Build Features: jemalloc
/// ```
pub(crate) const LONG_VERSION: &str = const_str::concat!(
    "Version: ",
    env!("CARGO_PKG_VERSION"),
    " | Commit SHA: ",
    env!("VERGEN_GIT_SHA"),
    " | Build Timestamp: ",
    env!("VERGEN_BUILD_TIMESTAMP"),
    " | Build Profile: ",
    build_profile_name()
);

const fn build_profile_name() -> &'static str {
    // Derived from https://stackoverflow.com/questions/73595435/how-to-get-profile-from-cargo-toml-in-build-rs-or-at-runtime
    // We split on the path separator of the *host* machine, which may be different from
    // `std::path::MAIN_SEPARATOR_STR`.
    const OUT_DIR: &str = env!("OUT_DIR");
    const SEP: char = if const_str::contains!(OUT_DIR, "/") {
        '/'
    } else {
        '\\'
    };
    let parts = const_str::split!(OUT_DIR, SEP);
    parts[parts.len() - 4]
}
