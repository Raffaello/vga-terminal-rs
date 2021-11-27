pub mod vt;

#[cfg(test)]
mod tests {
    use crate::vt::vt::vt_version;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_vt_version() {
        let version = vt_version();
        assert_eq!(version, "0.1");
    }
}
