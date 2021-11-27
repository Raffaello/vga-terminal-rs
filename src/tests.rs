#[cfg(test)]
mod tests {
    use crate::vt::vt::vt_version;

    #[test]
    fn test_vt_version() {
        let version = vt_version();
        const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
        assert_eq!(version, PKG_VERSION);
    }
}
