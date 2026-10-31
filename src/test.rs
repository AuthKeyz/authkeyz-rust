#[test]
#[ignore]
fn bench2() {
    let mut authkeyz = crate::v1_2::AuthKeyzApi::new("library-development", "EdmsTKiuld", "9f752b6a414455175efd942abfd2183667413d57b1d59d6742d8437c71802b49", "1.0", "https://authkeyz.bond/api/1.2/");
    let _ = authkeyz.init(None);
    let _ = authkeyz.login("demoseller".to_string(), "R9yzxdRyybgY75".to_string(), None);
}

#[test]
#[cfg(feature = "v1_0")]
#[ignore]
fn bench1() {
    let mut authkeyz = crate::v1_0::AuthKeyzApi::new("library-development", "EdmsTKiuld", "9f752b6a414455175efd942abfd2183667413d57b1d59d6742d8437c71802b49", "1.0", "https://authkeyz.bond/api/1.0/");
    let _ = authkeyz.init(None);
    let _ = authkeyz.login("demoseller".to_string(), "R9yzxdRyybgY75".to_string(), None);
}
