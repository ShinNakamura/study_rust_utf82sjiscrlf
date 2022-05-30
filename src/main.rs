// 参考 : https://qiita.com/V_lasergun/items/4926654ab4bd1ebc3d6c
//        https://keens.github.io/blog/2017/10/05/rustdekousokunahyoujunshutsuryoku/
use encoding_rs; // Shift_JIS のファイルへエンコードするのに必要
use std::error::Error;
use std::env;
use std::fs::File;
use std::io::{self, stdout, Write, BufWriter, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;
// type MyResult = Result<(), Box<dyn std::error::Error>>;

// コマンドライン引数からファイルパスを1つだけ読み込んで
// そのファイルが `utf8` である前提で
// `Shift_JIS` に変換し、改行コード CRLF で標準出力へ送る
// ただし、引数のファイルパスが "-" だったら標準入力から読み込む
fn main() -> MyResult<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1]; // ここは参照である必要がある。
    match open(filename) {
        Err(err) => eprintln!("{} : {}", filename, err),
        Ok(mut file) => {
            let out = stdout();
            // lock して排他処理を回避(スピードアップ目的)
            let mut out = BufWriter::new(out.lock());
            let mut line = String::new();
            loop {
                let bytes = file.read_line(&mut line)?;
                if bytes == 0 {
                    break;
                }
                // utf8 の時点で改行コードをLFからCRLFに変更
                line = line.replace("\n", "\r\n");
                // 改行コード変更済みの行データをShift_JISに変換
                let (res, _, _) = encoding_rs::SHIFT_JIS.encode(&line);
                let b = res.into_owned();
                // バイトデータをそもまま出力
                out.write(&b)?;
                line.clear();
            }
            out.flush()?;
        },
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

