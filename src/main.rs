extern crate getopts;
use getopts::Options;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::thread::{sleep_ms};
use std::env;
use std::process::exit;

fn print_usage(program: String, opts: Options) {
  let brief = format!("Usage: {} [options]", program);
  print!("{}", opts.usage(&brief));
}


fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.reqopt("p", "port", "port to listen to", "PORT");
  opts.optopt("f", "friends", "list of other program to communicate with", "host1:port1,host2:port2");
  opts.optflag("h", "help", "print this help menu");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => { panic!(f.to_string()) }
  };

  if matches.opt_present("h") {
    print_usage(program, opts);
    return;
  }

  let local_port = matches.opt_str("p").unwrap();
  let local = "127.0.0.1:".to_string() + &local_port;

  let listener = thread::spawn(move || { listener(local) });

  sleep_ms(1000);

  for host in matches.opt_strs("f").iter() {
    let host_clone = host.clone();
    let reader = thread::spawn(move || { reader(host_clone) });
  }

  // for stream in listener.incoming() {
  //     match stream {
  //         Ok(mut stream) => {
  //           let mut reading = String::new();
  //           let read = stream.read_to_string(&mut reading);
  //           println!("Read {} caracters", read.unwrap());
  //           println!("{:?}", reading);
  //         }
  //         Err(e) => {
  //           println!("Error while doing stuff : {}", e);
  //         }
  //     }
  // }

  let _ = listener.join();
}

fn listener(host: String) {
  match TcpListener::bind(&*host) {
    Err(e) => println!("Error {:?}", e),
    Ok(stream) => println!("Listening to {}", host)
  }
  sleep_ms(20000);
}

fn reader(host: String) {
  match TcpStream::connect(&*host) {
    Err(e) => println!("Error {}", e),
    Ok(stream) => println!("Connected to {}", host)
  }
}