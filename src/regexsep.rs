// 参考 https://stackoverflow.com/questions/56921637/how-do-i-split-a-string-using-a-rust-regex-and-keep-the-delimiters
// https://docs.rs/regex/latest/regex/#grouping-and-flags
use regex::Regex;

pub fn blockcutr(src: &str, sep: &str, blockn: usize) -> Option<String> {
    let sep = Regex::new(sep).expect("Invalid regex");
    let blocks = split_pos_keep(&sep, src);
    if blocks.len() - 1 < blockn {
        None
    } else {
        let blk = blocks[blockn].trim();
        Some(blk.to_string())
    }
}

fn split_pos_keep(r: &Regex, text: &str) -> Vec<String> {
    let mut splits = Vec::new();
    let mut last_index = 0;
    for mat in r.find_iter(text) {
        let index = mat.start();
        if last_index != index {
            splits.push(text[last_index..index].to_string());
        }
        last_index = mat.end();
    }
    if last_index < text.len() {
        splits.push(text[last_index..].to_string());
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
    assert_eq!(splits[0].trim(), "<h1>Harry</h1>".to_string());
    assert_eq!(splits[1].trim(), "<p>met Mary</p>".to_string());
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
