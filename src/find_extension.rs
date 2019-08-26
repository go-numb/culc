fn main() {
    let filename = "filename.jpeg";
    match find(filename, '.') { // .以降の拡張子を検索する（filename.min.jpegには適用されない）
        None => println!("拡張子が見つかりません."),

        // 返り値.の位置以降を出力
        Some(i) => println!("拡張子: {}", &file_name[i+1..]),
    }
}

// find ファイルネームから.位置を検索しlenght[i]をreturn
fn find(filename: &str, dot: char) -> Option<usize> {
    for (offset, c) in filename.char_indices() {
        if c == dot {
            // 一致
            return Some(offset);
        }
    }

    // 見つからなければNoneを返す
    None
}