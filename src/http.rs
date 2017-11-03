use std::str;
use std::string::String;

// going to be used for creating http responses
// and reading http responses
pub enum ContentType{
    Html,
    Plaintext,
}

impl ContentType{
    pub fn to_string(self) -> String{
        let mut content_type = match self{
            ContentType::Html => "text/html",
            ContentType::Plaintext => "text/plaintext",
        };
        content_type.to_string()
    }
}

pub struct HttpMessage{
    http_status: u16,
    http_status_msg: String,
    http_content_type: ContentType,
    message: Vec<u8>,
}

// used to store a request
// passed into server to return the content
// basically for now this only stores the contents the browser asked for
pub struct HttpRequest{
    pub requested_path: String,
}

impl HttpMessage{
    pub fn create_404_response() -> HttpMessage{
        HttpMessage{
            http_status: 404,
            http_status_msg: String::from("Not Found"),
            http_content_type: ContentType::Html,
            message: String::from("<h1>Not Found</h1>").into_bytes()
        }
    }

    // creates a 200 http response for a file
    pub fn create_simple_http_response(file_contents: &str) -> HttpMessage{
        HttpMessage{
            http_status: 200,
            http_status_msg: String::from("OK"),
            http_content_type: ContentType::Html,
            message: String::from(file_contents).into_bytes(),
        }
    }

    // this should pass ownership
    pub fn to_string(self) -> Result<String, String>{
        let mut message = String::from("");        
        // adding the header
        message.push_str(&format!("HTTP/1.1 {0} {1}", self.http_status, self.http_status_msg));
        message.push_str(&format!("\nContent-Type: {0}", self.http_content_type.to_string()));

        // adding the message body
        // Vec<u8> coerces to [u8] it's possible to use this as a parameter to a
        // function that takes a [u8]
        let payload = match str::from_utf8(&self.message){
            Ok(s) => s,
            Err(_e) => panic!("Could not read from u8 vector"),
        };
        message.push_str(&format!("\nContent-Length: {0}", payload.len()));
        message.push_str(&format!("\n\n{0}", payload));

        Ok(message)
    }
}

impl HttpRequest{
    // this will need to be expanded if i want more functionality
    pub fn new_from(request: &str) -> HttpRequest{
        let words: Vec<&str> = request.split(' ').collect(); 

        let path = match words[0]{
            "GET" => words[1],
            _ => panic!("Oh no the request did not split right!"),
        };
       
        HttpRequest{
            requested_path: String::from(path),
        }
    }
}