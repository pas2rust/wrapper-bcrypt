use wrapper_bcrypt::wrapper::{BcryptTrait, WrapperBcrypt};
#[test]
fn bcrypt_gen_hash_and_verify_password() {
    let mut wrapper = WrapperBcrypt::new("12345".to_string(), 13, None);
    let hash = wrapper.encode().expect("hash error");
    wrapper.set_hash(Some(hash));
    let verify = wrapper.verify();
    match verify {
        Ok(ok) => assert!(ok),
        Err(err) => {
            wrapper.print_err();
            assert!(false, "{}", err)
        },
    };
}
