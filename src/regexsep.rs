// 参考 https://stackoverflow.com/questions/56921637/how-do-i-split-a-string-using-a-rust-regex-and-keep-the-delimiters
// https://docs.rs/regex/latest/regex/#grouping-and-flags
use regex::Regex;

// blockn に指定された部分があればトリムして返す
pub fn blockcutr(src: &str, sep: &str, blockn: usize) -> Option<String> {
    let sep = Regex::new(sep).expect("Invalid regex");
    let blocks = split_pos_keep(&sep, src);
    if blocks.len() - 1 < blockn {
        None
    } else {
        let (spos, epos) = blocks[blockn];
        let s = src[spos..epos].trim().to_string();
        Some(s)
    }
}

// regex にヒットした部分をセパレータにして、
// セパレータの前後の部分(＝セパレータは含まない)の開始位置と終了位置が
// タプルになっているもののベクターを返す
fn split_pos_keep(r: &Regex, text: &str) -> Vec<(usize, usize)> {
    let mut splits = Vec::new();
    let mut last_index = 0;
    for mat in r.find_iter(text) {
        let index = mat.start();
        if last_index != index {
            splits.push((last_index, index));
        }
        last_index = mat.end();
    }
    if last_index < text.len() {
        splits.push((last_index, text.len()));
    }
    splits
}

#[test]
fn split_by_rgx() {
    let sep = Regex::new(r"(?i)<hr[^>]+?>").expect("Invalid regex");
    let text = r#"<h1>Harry</h1>
    <HR class="header" />
    <p>met Mary</p>
    <hr />"#;
    let splits = split_pos_keep(&sep, text);
    let (spos, epos) = splits[0];
    let s = text[spos..epos].trim().to_string();
    assert_eq!(s, "<h1>Harry</h1>".to_string());
    let (spos, epos) = splits[1];
    let s = text[spos..epos].trim().to_string();
    assert_eq!(s, "<p>met Mary</p>".to_string());
}

#[test]
fn get_blocks() {
    let sep = "(?i)<hr[^>]+?>";
    let text = r#"<h1>Harry</h1>
    <HR class="header" />
    <p>met Mary</p>
    <hr />"#;
    let b1 = blockcutr(&text, &sep, 0);
    let b2 = blockcutr(&text, &sep, 1);
    let b3 = blockcutr(&text, &sep, 2);
    assert_eq!(b1, Some("<h1>Harry</h1>".to_string()));
    assert_eq!(b2, Some("<p>met Mary</p>".to_string()));
    assert_eq!(b3, None);
}
