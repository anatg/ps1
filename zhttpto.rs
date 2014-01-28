    //
    // zhttpto.rs
    // 
    // University of Virginia - cs4414 Spring 2014
    // Weilin Xu and David Evans
    // Version 0.3




    #[feature(globs)];
    #[feature(managed_boxes)];
    use std::io::*;
    use std::io::net::ip::{SocketAddr};
    use std::io::net::tcp::TcpListener;
    use std::{str, os};
    use std::repr;

    static IP: &'static str = "127.0.0.1";
    static PORT:        int = 4414;
    static mut count:  int = 0;

    fn main() {
        let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
        let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
        
        println(format!("Listening on [{:s}] ...", addr.to_str()));
        
        for stream in acceptor.incoming() {
            // Spawn a task to handle the connection
            do spawn {
                let mut stream = stream;
                
                match stream {
                    Some(ref mut s) => {
                                 match s.peer_name() {
                                    Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                    None => ()
                                 }
                               },
                    None => ()
                }
                
                let mut buf = [0, ..500];
                stream.read(buf);
                let request_str = str::from_utf8(buf);
                println(format!("Received request :\n{:s}", request_str));
                unsafe{count+=1;}

                let mut lines: ~[&str] = request_str.split_str(" ").collect();
                let mut path = lines.remove(1).clone();
                let response: ~str;
                if path == "/"{
                    let response: ~str = format!(
                               "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                                <doctype !html><html><head><title>Hello, Rust!</title>
                                <style>body \\{ background-color: \\#111; color: \\#ff4acf \\}
                                       h1 \\{ font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red\\}
                                       h2 \\{ font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green\\}
                                </style></head>
                                <body>
                                <h1>Greetings, Krusty!</h1>
                                <h2>Visitor cout: {} </h2>
                                </body></html>\r\n", unsafe{count}); 
                            stream.write(response.as_bytes());
                    }
                else{
                    let mut fp = Path::new(path.clone().slice_from(1));
                    //println(path.clone().slice_from(1));
                    match (File::open(&fp)){
                        Some(mut file)=> {
                            if fp.clone().ends_with(".html"){
                                let file_data: ~[u8] = file.read_to_end();
                                stream.write(file_data);
                            }
                            else{
                                let response: ~str = format!(
                                    "HTTP/1.1 403 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                                <doctype !html><html><head><h1>HTTP Error 403 Forbidden</h1></html>");
                                stream.write(response.as_bytes());
                            }
                        }
                        None => {
                            println("failed to read");
                        }
                    }
                }
            }
            println!("Connection terminates.");
                
            }
        }

