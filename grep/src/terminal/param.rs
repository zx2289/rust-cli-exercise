use std::collections::HashMap;

#[derive(Debug)]
pub struct Param<'a> {
    search_value: &'a String,
    search_source: &'a String,
    instruct: Box<Vec<HashMap<&'a String, &'a String>>>,
}

impl<'a> Param<'a> {
    pub fn new(args: &'a Vec<String>) -> Option<Param<'a>> {
        let exec_path = args.get(0);
        let mut search: Option<&String> = None;
        let mut search_from: Option<&String> = None;
        let mut instruct_vec: Vec<HashMap<&'a String, &'a String>> = Vec::new();
        let mut dealed_index:usize = 0;
        for (i, instruct) in args.iter().enumerate() {
            if i == 0 || dealed_index == i {
                continue;
            }
            if instruct.starts_with("-") {
                let uppercase_v = instruct.to_uppercase();
                // 这个是一个指令参数 对于不同的指令参数进行不同的处理
                if uppercase_v == "-C" || uppercase_v == "-A" {
                    let c_param = args.get(i + 1);
                    match c_param {
                        Some(v) => {
                            let mut map: HashMap<&'a String, &'a String> = HashMap::new();
                            map.insert(instruct, v);
                            instruct_vec.push(map);
                            dealed_index = i+1;
                        }
                        None => {
                            println!("{:?} 指令缺少必要参数", instruct);
                            return None;
                        }
                    }
                }
            } else {
                if let Some(v) = search {
                    search_from = Option::from(instruct);
                } else {
                    search = Option::from(instruct);
                }
            }
        }

        Option::Some(Param {
            search_value: search.unwrap(),
            search_source: search_from.unwrap(),
            instruct: Box::new(instruct_vec),
        })
    }

    pub fn search_value(&self) -> &String {
        self.search_value
    }

    pub fn search_source(&self) -> &String {
        self.search_source
    }
}