use std::io::prelude::*;
use std::io::Error;
use std::net::TcpStream;

pub trait Prismatik {
	fn light_count(&mut self) -> Result<usize, Error>;
	fn lock(&mut self) -> Result<(), Error>;
	fn unlock(&mut self) -> Result<(), Error>;
	fn set_brightness(&mut self, level: usize) -> Result<(), Error>;
	fn set_smooth(&mut self, level: usize) -> Result<(), Error>;
	fn set_color(&mut self, id: usize, r: usize, g: usize, b:usize) -> Result<(), Error>;
	fn set_on(&mut self, on: bool) -> Result<(), Error>;
}

pub fn set_all_lights(api: &mut Prismatik, r: usize, g: usize, b: usize) -> Result<(), Error> {
	let count = api.light_count();

	if count.is_err() {
		return Err(count.err().unwrap())
	} else {
		let count = count.unwrap();

		for id in 0..count {
			let r = api.set_color(id, r, g, b);
			if r.is_err() {
				return r;
			}
		}

		Ok(())
	}
}

struct Dummy;

impl Dummy {
	pub fn new() -> Dummy {
		Dummy{}
	}
}

impl Prismatik for Dummy {
	fn light_count(&mut self) -> Result<usize, Error> { Ok(100) }
	fn lock(&mut self) -> Result<(), Error> { Ok(()) }
	fn unlock(&mut self) -> Result<(), Error> { Ok(()) }
	fn set_brightness(&mut self, _: usize) -> Result<(), Error> { Ok(()) }
	fn set_smooth(&mut self, _: usize) -> Result<(), Error> { Ok(()) }
	fn set_color(&mut self, _: usize, _: usize, _: usize, _:usize) -> Result<(), Error> { Ok(())}
	fn set_on(&mut self, _: bool) -> Result<(), Error> { Ok(()) }
}

pub struct CoreApi {
	stream: TcpStream
}

impl CoreApi {

	pub fn new(path: &str, key: &str) -> Option<CoreApi> {

		let out_stream = TcpStream::connect(path);

		if out_stream.is_ok() {

			let mut prism = CoreApi {
				stream: TcpStream::connect(path).unwrap()
			};
			
			prism.send_key(key);

			Some(prism)
		} else {
			None
		}
	}

	pub fn flush(&mut self) {
		self.stream.flush();
	}

	pub fn send_key(&mut self, key: &str) {
		let key_string = format!("apikey:{{{}}}", key).to_string();
		write!(self.stream, "{}\n", key_string);
		self.flush();
	}
}

impl Prismatik for CoreApi {

	fn light_count(&mut self) -> Result<usize, Error> {
		Ok(100)
	}

	fn lock(&mut self) -> Result<(), Error> {
		write!(self.stream, "lock\n")
	}

	fn unlock(&mut self) -> Result<(), Error> {
		write!(self.stream, "unlock\n")
	}

	fn set_brightness(&mut self, level: usize) -> Result<(), Error> {
		write!(self.stream, "setbrightness:{}\n", level)
	}

	fn set_smooth(&mut self, level: usize) -> Result<(), Error> {
		write!(self.stream, "setsmooth:{}\n", level)
	}

	fn set_color(&mut self, id: usize, r: usize, g: usize, b:usize) -> Result<(), Error> {
		write!(self.stream, "setcolor:{}-{},{},{};", id, r, g, b)
	}

	fn set_on(&mut self, on: bool) -> Result<(), Error> {
		write!(self.stream, "setstatus:{}", if on { "on" } else { "off" })
	}
}
