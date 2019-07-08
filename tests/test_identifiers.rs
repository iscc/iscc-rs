use iscc_rs::{content_id_image, content_id_mixed, content_id_text, data_id, instance_id, meta_id};

#[test]
fn test_meta_id() {
    // TODO: Also test argument `extra`
    let (mid1, _, _) = meta_id("ISCC Content Identifiers", "");
    assert_eq!(mid1, "CCDFPFc87MhdT");

    let (mid1, title, extra) = meta_id("Die Unendliche Geschichte", "");
    assert_eq!(mid1, "CCAKevDpE1eEL");
    assert_eq!(title, "die unendliche geschichte");
    assert_eq!(extra, "");

    let (mid2, _, _) = meta_id(" Die unéndlíche,  Geschichte ", "");
    assert_eq!(mid1, mid2);

    let (mid, title, extra) = meta_id("Iñtërnâtiônàlizætiøn☃", "");
    assert_eq!(mid, "CCj3TQrYcgaox");
    assert_eq!(title, "internationalizætiøn☃");
    assert_eq!(extra, "")
}

#[test]
fn test_content_id_text() {
    let cid_t_np = content_id_text("", false);
    assert_eq!(cid_t_np, "CT7A4zpmccuEv");

    let cid_t_p = content_id_text("", true);
    assert_eq!(cid_t_p, "Ct7A4zpmccuEv");
}

#[test]
fn test_content_id_image() {
    let cid_i = content_id_image("tests/test_data/lenna.jpg", false).unwrap();
    assert_eq!(cid_i, "CYmLoqBRgV32u");

    let cid_i = content_id_image("tests/test_data/lenna.jpg", true).unwrap();
    assert_eq!(cid_i, "CimLoqBRgV32u");
}

#[test]
fn test_content_id_mixed() {
    let cid_t_1 = content_id_text("Some Text", false);
    let cid_t_2 = content_id_text("Another Text", false);

    let cid_m = content_id_mixed(&[&cid_t_1], false);
    assert_eq!(cid_m, "CM3k9pp7JS7nP".to_string());

    let cid_m = content_id_mixed(&[&cid_t_1, &cid_t_2], false);
    assert_eq!(cid_m, "CM3kHkNRGvnhB".to_string());

    let cid_i = content_id_image("tests/test_data/lenna.jpg", false).unwrap();
    let cid_m = content_id_mixed(&[&cid_t_1, &cid_t_2, &cid_i], false);
    assert_eq!(cid_m, "CM3hswzATv9d3".to_string());
}

#[test]
fn test_data_id() {
    assert_eq!(
        data_id("tests/test_data/cat.jpg").unwrap(),
        "CDC7Lg4oHA8DC".to_string()
    );
    assert_eq!(
        data_id("tests/test_data/cat.png").unwrap(),
        "CDCx1AzhDGcT7".to_string()
    );
    assert_eq!(
        data_id("tests/test_data/cat.gif").unwrap(),
        "CDcLVF7es2AEP".to_string()
    );
}

#[test]
fn test_instance_id() {
    assert_eq!(
        instance_id("tests/test_data/cat.jpg").unwrap(),
        (
            "CRLdd9g4BSUyY".to_string(),
            "f8e5e94f953709ae8930220da8bada303a370a46157a5cdd50ad2476a7f51e42".to_string()
        )
    );
    assert_eq!(
        instance_id("tests/test_data/cat.png").unwrap(),
        (
            "CR6xpnrJkvQDH".to_string(),
            "23a5b78a044143b612a0d033384a5ebf95343ec812c3e62caff393852d0328a0".to_string()
        )
    );
    assert_eq!(
        instance_id("tests/test_data/cat.gif").unwrap(),
        (
            "CR167E86HPsZV".to_string(),
            "424a89d97aa978363b76071510949bf3b3424f3c6d394ccde93d7fa62ced066a".to_string()
        )
    );
}
