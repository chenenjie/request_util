extern crate hyper;
extern crate regex;

use std::io;

use hyper::Client;
use hyper::header::Connection;

    
fn main() {
    let url = "http://hk-bici.net/forum.php";
    
    let client = Client::new();
    
    let mut res = client.get(url)
        .header(Connection::close())
        .send().unwrap();
       
    println!("Response: {}", res.status);
    println!("Headers:\n{}", res.headers);
    io::copy(&mut res, &mut io::stdout()).unwrap();
        
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    
    
    #[test]
    fn testRegex(){
        let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        println!("{:?}",re.is_match("2014-01-01"));
    }
}
