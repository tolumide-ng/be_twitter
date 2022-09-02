mod test_pkce {
    use crate::helpers::gen_pkce::Pkce;

    #[test]
    fn generates_base_63_on_new() {
        let url = Pkce::new().to_string();
        let url = url.as_str();

        assert!(!url.to_string().contains("/"));
        assert!(!url.to_string().contains("+"));
    }
}