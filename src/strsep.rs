pub fn blockcutr(src: &str, sep: &str, blockn: usize) -> Option<String> {
    let blocks: Vec<&str> = src.split(sep).collect();
    if blocks.len() < blockn {
        None
    } else {
        let blk = blocks[blockn].trim();
        Some(blk.to_string())
    }
}

#[test]
fn t1() {
    let src = r#"aaa
    --
    bbb
    --
    ccc"#;
    let src = src.to_string();
    let sep = "--".to_string();
    let expect1 = "aaa".to_string();
    let expect2 = "bbb".to_string();
    let expect3 = "ccc".to_string();
    assert_eq!(expect1, blockcutr(&src, &sep, 0).unwrap());
    assert_eq!(expect2, blockcutr(&src, &sep, 1).unwrap());
    assert_eq!(expect3, blockcutr(&src, &sep, 2).unwrap());
    assert_eq!(None, blockcutr(&src, &sep, 55));
}

#[test]
fn item_name() {
    let name = "改造人間 | お買い得 【当日発送】 | 型番100";
    let sep = "|";
    let pure_name = "改造人間".to_string();
    let serial = "型番100".to_string();
    assert_eq!(pure_name, blockcutr(name, sep, 0).unwrap());
    assert_eq!(serial, blockcutr(name, sep, 2).unwrap());
}

