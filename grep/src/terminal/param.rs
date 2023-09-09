

#[derive(Debug)]
pub struct Param<'a>{
    search_value:&'a String,
    search_source:&'a String
}
impl<'a>  Param<'a> {
    pub  fn new (args:&'a Vec<String> ) -> Option<Param<'a>>{ 
        let exec_path = args.get(0);
        let search:&String;
        let search_from:&String;

        match args.get(1) {
            Some(v) => search = v,
            None => {println!("缺少必要参数");return None;},
        }
        match args.get(2) {
            Some(v) => search_from = v,
            None => {println!("缺少必要参数");return None;},
        }
        Option::Some( Param{
            search_value:search,
            search_source:search_from
        })
    }

    pub fn search_value(&self) -> &String {
        self.search_value
    }

    pub fn search_source(&self) -> &String {
        self.search_source
    }
}