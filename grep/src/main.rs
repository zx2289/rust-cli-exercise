use std::env;
mod terminal;

use terminal::param::{Param};

fn main() {
    let args:Vec<String> = env::args().collect();
    //获取到参数之后 按照 搜索内容，文件  的格式保存参数
    let param_option = Param::new(&args);
    let param:Param;
    match param_option {
        Some(some) => {println!("参数解析成功");
            param = some},
        None => return,
    }

    println!("参数解析结果{:?}",param);
}
