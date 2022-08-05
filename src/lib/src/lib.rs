use ini::Ini;
use uuid::Uuid;
use std::net::{TcpListener, TcpStream};
use std::os::raw::{c_int, c_char};
use std::io::{Write, Read};
use std::{fs, slice, ptr};
use std::cell::RefCell;
use std::error::Error;

#[macro_use]
extern crate error_chain;

mod other_error {
    error_chain! {}
}




thread_local!{
    static LAST_ERROR: RefCell<Option<Box<dyn Error>>> = RefCell::new(None);
}





trait Rust {
    fn render_nm(&self);
    fn start_webserver(&self);
    fn start_rest(&self);
}


#[derive(Debug, Clone)]
pub struct Response {
    pub function_name: String,
    pub return_code: i8,
    pub body: Option<Vec<u8>>,
}






pub fn start_webserver_rust(){
    
}

pub fn webserver_handle(mut stream: TcpStream) {
    let contents = fs::read_to_string("index.html").unwrap();
    let response: String = format!(
        "HTTP/1.1 200 OK\r\nContent-Lenght: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}





pub fn start_rest_rust() -> std::io::Result<()> {  // <E: std::convert::From<std::io::Error>>() -> Result<(), E>
    let listener = TcpListener::bind("127.0.0.8081")?;
        for stream in listener.incoming(){
            rest_handle(stream?)?;
        };
        Ok(())
}

pub fn rest_handle(mut stream: TcpStream) -> std::io::Result<()> {
    //handle json
    println!("Connection from {}", stream.peer_addr()?);
    let mut buffer = [0; 8192];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            return Ok(());
        }

        for i in 0..nbytes {
            print!("{}", buffer[i]);
        }
        println!();

        // Write buffer to socket
        stream.write(&buffer[..nbytes])?;
        stream.flush()?;
    }
}





pub fn update_last_error<E: Error + 'static>(err: E) {
    eprintln!("Setting LAST_ERROR: {}", err);

    {
        // Print a pseudo-backtrace for this error, following back each error's
        // cause until we reach the root error.
        let mut cause = err.source();
        while let Some(parent_err) = cause {
            eprintln!("Caused by: {}", parent_err);
            cause = parent_err.source();
        }
    }

    LAST_ERROR.with(|prev| {
        *prev.borrow_mut() = Some(Box::new(err));
    });
}

// Retrieve the most recent error, clearing it in the process.
pub fn take_last_error() -> Option<Box<dyn Error>> {
    LAST_ERROR.with(|prev| prev.borrow_mut().take())
}





impl dyn Rust {

    #[no_mangle]
    pub extern "C" fn render_nm() {
        //import existing con
        //TODO make all dependencies parameter
   
        //generate random uuid
        let uuid = Uuid::new_v4();
        let mut conf = Ini::new();

        //render config
        conf.with_section(Some("connection"))
            .set("id", "")
            .set("uuid", uuid.hyphenated().encode_lower(&mut Uuid::encode_buffer()))
            .set("type", "wifi")
            .set("interface-name", "wlo1")
            .set("permissions", "");
        conf.with_section(Some("wifi"))
            .set("mac-address-blacklist", "")
            .set("mode", "infrastructure")
            .set("ssid", "WLAN-967C53");
        conf.with_section(Some("wifi-security"))
            .set("auth-alg", "open")
            .set("key-mgmt", "wpa-psk")
            .set("psk", "my-password");
        conf.write_to_file("conf.ini").unwrap();
    }



    #[no_mangle]
    pub extern "C" fn start_webserver() -> std::io::Result<()>{
        let listener = TcpListener::bind("127.0.0.1:8080")?;
        for stream in listener.incoming() {
            webserver_handle(stream?);
        }
        Ok(())
    }



    #[no_mangle]
    pub extern "C" fn start_rest() -> *mut Response {
        let _ = match start_rest_rust() {
            Ok(s) => s,
            Err(e) => {
                //let err = Error::with_chain(e, "Unable to start rest-api");
                update_last_error(e);
                return ptr::null_mut();
            }
        };
        return ptr::null_mut()
    }



    #[no_mangle]
    pub extern "C" fn last_error_length() -> c_int { //for returning error codes to c
        LAST_ERROR.with(|prev| match *prev.borrow() {
        Some(ref err) => err.to_string().len() as c_int + 1,
        None => 0,
    })
    }


    #[no_mangle]
    pub unsafe extern "C" fn last_error_message(buffer: *mut c_char, length: c_int) -> c_int {
        if buffer.is_null() {
            eprintln!("Null pointer passed into last_error_message() as the buffer");
            return -1;
        }

        let last_error = match take_last_error() {
            Some(err) => err,
            None => return 0,
        };

        let error_message = last_error.to_string();
        let buffer = slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

        if error_message.len() >= buffer.len() {
            eprintln!("Buffer provided for writing the last error message is too small.");
            eprintln!(
                "Expected at least {} bytes but got {}",
                error_message.len() + 1,
                buffer.len()
            );
            return -1;
        }

        ptr::copy_nonoverlapping(
            error_message.as_ptr(),
            buffer.as_mut_ptr(),
            error_message.len(),
        );

        buffer[error_message.len()] = 0;
        error_message.len() as c_int
    }

}



#[cfg(test)]
mod tests {

}
