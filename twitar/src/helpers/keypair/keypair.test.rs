mod test_keypair {
    use crate::helpers::keypair::KeyPair;

    #[test]
    fn create_a_new_keypair_with_a_secret_value() {
        let key = String::from("key");
        let value = String::from("value");

        let  kp = KeyPair::new(key.clone(), value.clone());
        assert_eq!(kp.key, key);
        assert_eq!(kp.secret, value);
    }

    #[test]
    fn create_an_empty_keypair() {
        let kp = KeyPair::empty();


        assert_eq!(kp.key, String::from(""));
        assert_eq!(kp.secret, String::from(""));
    }
}