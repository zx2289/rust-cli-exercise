use super::param::Param;
use std::fs;
use std::io;
use std::io::BufRead;
use colored::*;
pub fn search(param: &Param) {
    let source = param.search_source();
    let value = param.search_value();
    let file_reader = open_file(source);
    match file_reader {
        Ok(file_reader) => {
            for ele in file_reader.lines() {
                match ele {
                    Ok(text) => {
                        if text.contains(value) {
                            let highlighted_text = text.replace(value, &value.yellow().to_string());
                            println!("{}", highlighted_text);
                        }
                    }
                    Err(_) => return,
                }
            }
        }
        Err(err) => {println!("{}",err)}
    }
}

// 打开文件 返回打开文件的句柄,做异常处理
fn open_file(path: &String) ->  Result<io::BufReader<fs::File>,String> {
    let source_file = fs::File::open(path);
    match source_file {
        Ok(file) => {
            Ok(io::BufReader::new(file))
        }
        Err(_) => {Err(String::from("被搜索文件不存在"))}
    }

}