extern crate reqwest;

fn try_request(url: &str) {
	let response_text = reqwest::get(url)
		.expect("couldn't make request")
		.text()
		.expect("Couldn't read response text");
	println!("{}", response_text);

	//LONG WAY
	//
	// match reqwest::get(url) {
 //    	Ok(mut response) => {
 //    		// check if 200 OK
 //    		if response.status() == reqwest::StatusCode::Ok {
 //    			match response.text() {
 //    				Ok(text) => println!("{}", text),
 //    				Err(_) => println!("Can't read response text."),
 //    			}
 //    		} else {
 //    			println!("Response was not 200, response was {}", response.status());
 //    		}
 //    		// println!("{}", expr);
 //    	},
 //    	Err(e) => {
 //    		println!("Erro: {}", e.to_string());
 //    	},
 //    }
}

fn main() {
	println!("http://httpbin.org/ip");
	try_request("http://httpbin.org/ip");
    println!("http://httpbin.org/qwerty");
	try_request("http://httpbin.org/qwerty");
	// println!("http://www.google.com");
	// try_request("http://www.google.com");
	// println!("http://fvhajklldfksl");
	// try_request("http://fvhajklldfksl");

}
