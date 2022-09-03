mod test_keyval {
    use std::borrow::Cow;

    use hyper::Uri;

    use crate::helpers::keyval::KeyVal;

    #[test]
    fn keyval_new_generates_empty_hashmap() {
        let keyval = KeyVal::new();

        let keys = keyval.keys().collect::<Vec<&Cow<'static, str>>>();

        assert_eq!(keys.len(), 0);
    }


    #[test]
    fn add_keyval_adds_keyval() {
        let keyval = KeyVal::new();
        let keys = keyval.keys().collect::<Vec<&Cow<'static, str>>>();
        assert_eq!(keys.len(), 0);

        let new_keyval = keyval.add_keyval(String::from("key"), String::from("val"));
        let new_keys = new_keyval.keys().collect::<Vec<&Cow<'static, str>>>();
        assert_eq!(new_keys.len(), 1);
        assert_eq!(new_keys[0].to_string(), String::from("key"));
    }


    #[test]
    fn new_with_keyval_creates_hashmap_with_content_on_creation() {
        let key = String::from("first_key");
        let value = String::from("first_value");
        let keyval = KeyVal::new_with_keyval(key.clone(), value.clone());

        let keys = &keyval.keys().collect::<Vec<&Cow<'static, str>>>();
        let values = &keyval.values().collect::<Vec<&Cow<'static, str>>>();
        assert_eq!(keys.len(), 1);
        assert_eq!(values.len(), 1);
        assert_eq!(keys[0].to_string(), key);
        assert_eq!(values[0].to_string(), value);
    }


    #[test]
    fn adds_a_list_of_keyvalues() {
        let list_keyval = vec![("first_key".to_string(), "first_val".to_string()), ("Second_key".to_string(), "SecondVal".to_string())];
        let keyvals = KeyVal::new().add_list_keyval(list_keyval.clone());

        let keys = &keyvals.keys().collect::<Vec<&Cow<'static, str>>>();
        // .map(|x| x.to_string()).collect::<Vec<String>>();
        let values = &keyvals.values().collect::<Vec<&Cow<'static, str>>>();

        assert_eq!(keys.len(), 2);
        assert_eq!(values.len(), 2);
        assert!(keys.contains(&&Cow::from(&list_keyval[0].0)));
        assert!(keys.contains(&&Cow::from(&list_keyval[1].0)));
        assert!(!keys.contains(&&Cow::from(&list_keyval[0].1)));
        assert!(values.contains(&&Cow::from(&list_keyval[0].1)));
        assert!(values.contains(&&Cow::from(&list_keyval[1].1)));
    }

    #[test]
    fn converts_queryparams_to_keyval() {
        let uri = "https://www.rust-lang.org/install.html?name=tolu&hobby=programming".parse::<Uri>().unwrap();
        let key_val = KeyVal::query_params_to_keyval(&uri);
        assert!(key_val.is_ok());
        let u_key_val = key_val.unwrap();
        let keys = &u_key_val.keys().collect::<Vec<&Cow<'static, str>>>();
        let values = &u_key_val.values().collect::<Vec<&Cow<'static, str>>>();

        assert_eq!(keys.len(), 2);
        assert_eq!(values.len(), 2);
        assert!(keys.contains(&&Cow::from("name")));
        assert!(keys.contains(&&Cow::from("hobby")));
        assert!(values.contains(&&Cow::from("tolu")));
        assert!(values.contains(&&Cow::from("programming")));
    }

    #[test]
    fn string_to_keyval() {
        let string = String::from("name=tolu&hobby=programming");
        let key_val = KeyVal::string_to_keyval(string);
        assert!(key_val.is_some());
        let key_val = key_val.unwrap();
        let keys = &key_val.keys().collect::<Vec<&Cow<'static, str>>>();
        let values = &key_val.values().collect::<Vec<&Cow<'static, str>>>();

        assert_eq!(keys.len(), 2);
        assert_eq!(values.len(), 2);
        assert!(keys.contains(&&Cow::from("name")));
        assert!(keys.contains(&&Cow::from("hobby")));
        assert!(values.contains(&&Cow::from("tolu")));
        assert!(values.contains(&&Cow::from("programming")));
    }

    #[test]
    fn keyval_converts_to_urlencoded_string() {
        let key_val = KeyVal::new_with_keyval(String::from("key"), String::from("value")).to_urlencode();

        assert!(key_val.contains("value"));
        assert!(key_val.contains("="));
        assert!(!key_val.contains("&"));
    }

    #[test]
    fn validates_if_a_key_is_mapped_to_a_value_in_keyval() {
        let first = (String::from("first_key"), String::from("first_value"));
        let second = (String::from("second_key"), String::from("second_value"));
        let key_val = KeyVal::new().add_list_keyval(vec![first.clone(), second.clone()]);


        assert!(key_val.validate(first.0.clone(), first.1.clone()));
        assert!(key_val.validate(second.0.clone(), second.1.clone()));
        assert!(!key_val.validate(first.0.clone(), second.1.clone()));
        assert!(!key_val.validate(first.1.clone(), second.1.clone()));
    }


    #[test]
    fn verifies_if_all_provided_keys_are_in_keyval() {
        let first = (String::from("first_key"), String::from("first_value"));
        let second = (String::from("second_key"), String::from("second_value"));
        let key_val = KeyVal::new().add_list_keyval(vec![first.clone(), second.clone()]);

        assert!(key_val.every(vec![first.0.clone(), second.0.clone()]).is_some());
        assert!(key_val.every(vec![first.0.clone(), second.0, String::from("non-existing-key")]).is_none());
        assert!(key_val.every(vec![first.0]).is_some());
    }
}