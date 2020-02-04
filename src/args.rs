use std::sync::Arc;

pub struct Args {
    pub str_len: usize,
    pub thread_num: u8,
    pub search_str: Arc<String>,
}
impl Args {
    pub fn new(arg_vec: Vec<String>) -> Result<Args, &'static str> {
        let mut thread_num = 8;
        let mut str_len = 10;
        let mut search_str = String::from("-1");
        let mut i = 1;

        while  i < arg_vec.len(){
            if arg_vec.get(i).unwrap() == "-t" {
                i += 1;
                thread_num = match arg_vec.get(i).unwrap().parse() {
                    Ok(arg) => arg,
                    Err(_)  => return Err("Unable to parse number of threads")
                };
            }
            else if arg_vec.get(i).unwrap() == "-l" {
                i += 1;
                str_len = match arg_vec.get(i).unwrap().parse() {
                    Ok(arg) => arg,
                    Err(_)  => return Err("Unable to parse length of string")
                };
            }
            else {
                search_str = arg_vec.get(i).unwrap().clone();
            }
            i += 1;
        }

        if search_str == "-1" {
            return Err("No search string given")
        }

        let search_str = Arc::new(search_str);

        Ok(Args {str_len, thread_num, search_str})
    }
}