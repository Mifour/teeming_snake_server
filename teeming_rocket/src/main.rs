#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
extern crate fern;
extern crate log;

use fern::colors::{Color, ColoredLevelConfig};
#[allow(unused_imports)]
use log::{debug, info, trace, warn};
use rocket::http::RawStr;
use rocket::State;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::net::{IpAddr, SocketAddr};
use std::sync::Mutex;

use rocket::request::{self, Request, FromRequest};
use rocket::Outcome;
use rocket::http::Status;



#[macro_use] extern crate rocket;

const MAX_MAP: i64 = 5;

#[allow(dead_code)]
fn setup_logger() -> Result<(), fern::InitError> {
	let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::White)
        .trace(Color::BrightBlack);
	let colors_level = colors_line.clone().info(Color::Green);
	
	fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{date}][{target}][{level}{color_line}] {message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors_line.get_color(&record.level()).to_fg_str()
                ),
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message,
            ));
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        //.chain(fern::log_file("output.log")?)
        .apply()?;

    Ok(())
}


pub trait ModuloSignedExt {
    fn modulo(&self, n: Self) -> Self;
}
macro_rules! modulo_signed_ext_impl {
    ($($t:ty)*) => ($(
        impl ModuloSignedExt for $t {
            #[inline]
            fn modulo(&self, n: Self) -> Self {
                (self % n + n) % n
            }
        }
    )*)
}
modulo_signed_ext_impl! { i8 i16 i32 i64 }

pub trait ChangeBaseExt {
	fn change_base(&self, n: Self) -> VecDeque<i64>;
}
macro_rules! change_base_ext_impl {
	($($t:ty)*) => ($(
		impl ChangeBaseExt for $t{
			#[inline]
			fn change_base(&self, n:Self) -> VecDeque<i64> {
				let mut r: VecDeque<i64> = VecDeque::new();
				let mut q = (*self).clone();
				while q > 0 {
					r.push_front(q.modulo(n) as i64);
					q = q/ n;
				} 
				r 
			}
		}
	)*)
}
change_base_ext_impl! {i8 i16 i32 i64}

macro_rules! map(
	{$($key:expr => $value:expr),+} => {
		{
			let mut m = HashMap::new();
	        $(
	        	m.insert($key, $value);
	        )+
	        m
    	}
	}
);


struct NumberTrack{ n: Mutex<i64>}
struct MapOfCode{ map: Mutex<HashMap<String, String>>}

#[derive(Clone, Debug)]
pub struct RealIP(IpAddr);
impl<'a, 'r> FromRequest<'a, 'r> for RealIP {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.client_ip() {
            Some(ip) => Outcome::Success(RealIP(ip)),
            None => Outcome::Failure((Status::from_code(401).unwrap(), ()))
        }
    }
}

#[get("/generate")]
fn generate(_remote_addr: SocketAddr, ip: RealIP, n: State<NumberTrack>, hashmap: State<MapOfCode>) -> String {
    //ask for a code
    let remote_addr: String = format!("{:?}",ip).to_string();
    
    /*if let Some(current) = req.remote() {
        let ip = req.headers()
            .get_one("X-Real-IP")
            .and_then(|ip| {
                ip.parse()
                    .map_err(|_| warn_!("'X-Real-IP' header is malformed: {}", ip))
                    .ok()
            });

        if let Some(ip) = ip {
            req.set_remote(SocketAddr::new(ip, current.port()));
        }
    }*/

    let mut counter = 0;
    let mut code = n.n.lock().unwrap();
    let mut lock = hashmap.map.lock().unwrap();
    loop {
    	if counter > MAX_MAP {
    		return "service is full".to_string();
    	}
	    *code = (*code+1).modulo(MAX_MAP);
	    if !(*lock).contains_key(&(*code).to_string()){
	    	break;
	    }
	    else {
	    	counter+=1;
	    }
	}
	
	(*lock).insert((*code).to_string(), remote_addr.clone().to_string());

    return  format!("{}{}{}",(*code).to_string(),"@".to_string(), remote_addr.to_string() );
}

#[get("/free/<code>")]
fn free(code: &RawStr, hashmap: State<MapOfCode>){
	// make a code available by removing key
	let mut lock = hashmap.map.lock().unwrap();
	(*lock).remove(code.as_str());
}

#[get("/find/<code>")]
fn find(code: &RawStr, hashmap: State<MapOfCode>) -> String {
	// given a code, return the adress bound to it
	// if the code doesn't link to any adress, return empty String
	let mut address = String::from("address not found");
	let lock = hashmap.map.lock().unwrap();
	if (*lock).contains_key(code.as_str()) {
		address = ((*lock).get(code.as_str()).unwrap_or_else(|| &address)).to_string();
	}
	address.to_string()
}

fn main() {
	// sudo ROCKET_ENV=staging cargo run
	// 7, 6, 5 ....
    rocket::ignite().mount("/", routes![find, free, generate]).manage(
		MapOfCode { map: Mutex::new( 
			map!{"0".to_string() => "127.0.0.1".to_string()} 
		) }
		).manage( NumberTrack { n: Mutex::new(0) }
   	).launch();
    // ignition sequence started!
}
