use std::fs::File;
use std::io::Write;
use rand::Rng;
use std::iter;

fn main() -> std::io::Result<()> {
    let file_size_lines = 1_0000_000;  //该值表示文件的行数
    let line_length = 1001;  //该值代表每行的字符数（不包括换行符）

    let mut file = File::create("big_file.txt")?;

    let mut rng = rand::thread_rng();

    for _ in 0..file_size_lines {
        let line: String = iter::repeat(())
            .map(|_| rng.sample(rand::distributions::Alphanumeric).to_string())
            .take(line_length).collect();


        writeln!(file, "{}", line)?;
    }

    Ok(())
}