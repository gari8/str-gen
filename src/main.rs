extern crate core;

use std::env;
use getopts::Options;
use rand::Rng;

fn main() {
   match parse_options() {
       None => eprintln!("NG"),
       Some(length) => {
           let int_len = match length.parse::<u64>() {
               Ok(res) => res,
               Err(err) => panic!("{}", err.to_string())
           };
           match create_str(int_len) {
               Some(res) => println!("{}", res),
               None => eprintln!("please input larger number than 0")
           }
       },
   }
}

fn parse_options() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("l", "length", "set output uuid length", "LENGTH");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => return None
    };
    return matches.opt_str("l");
}

fn create_str(length: u64) -> Option<String> {
    let mut rng = rand::thread_rng();
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-+".chars().collect::<Vec<char>>();
    let str_list = (0..length).map(|_| letters[rng.gen_range(1..letters.len())]).collect::<Vec<_>>();
    if str_list.iter().collect::<String>().len() != 0 {
        return Some(str_list.iter().collect::<String>());
    }
    return None;
}

#[cfg(test)]
mod tests {
    extern crate std;
    use crate::create_str;

    #[test]
    fn test_create_str() {
        for i in 1..1000 {
            match create_str(i) {
                None => eprintln!("test failed"),
                Some(res) => assert_eq!(i as usize, res.len())
            };
        }
    }
}
