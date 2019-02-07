extern crate reqwest;

fn main() {
    match reqwest::get("http://httpbin.org/ip") {
    	Ok(mut response) => {
    		// check if 200 OK
    		if response.status() == reqwest::StatusCode::Ok {
    			match response.text() {
    				Ok(text) => println!("{}", text),
    				Err(_) => println!("Can't read response text."),
    			}
    		} else {
    			println!("Response was not 200, response was {}", response.status());
    		}
    		// println!("{}", expr);
    	},
    	Err(e) => {
    		println!("Erro: {}", e.to_string());
    	},
    }
}
