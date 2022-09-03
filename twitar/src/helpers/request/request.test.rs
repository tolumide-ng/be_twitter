mod test_req_query {
    use crate::helpers::request::req_query;

    #[test]
    pub fn should_return_some_when_the_key_exists() {
        let query = Some("name=testing");
        let key = "name";

        assert!(req_query(query, key).is_some());
    }

    #[test]
    pub fn should_return_none_when_the_key_does_not_exist() {
        let query = Some("name=testing");
        let key = "no-name";

        assert!(req_query(query, key).is_none());
    }


    #[test]
    pub fn should_return_none_when_key_does_not_exist_and_query_is_an_empty_some() {
        let query = Some("");
        let key = "no-name";

        assert!(req_query(query, key).is_none());
    }
}