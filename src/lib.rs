use std::io::prelude::*;
use std::net::TcpStream;

pub trait Prismatik {
	fn light_count(&mut self) -> usize;
	fn lock(&mut self) -> bool;
	fn unlock(&mut self) -> bool;
	fn set_brightness(&mut self, level: usize);
	fn set_smooth(&mut self, level: usize);
	fn set_color(&mut self, id: usize, r: usize, g: usize, b:usize);
	fn set_on(&mut self, on: bool);
}

pub fn set_all_lights(api: &mut Prismatik, r: usize, g: usize, b: usize) {
	let count = api.light_count();
	for id in 0..count {
		api.set_color(id, r, g, b);
	}
}

struct Dummy;

impl Dummy {
	pub fn new() -> Dummy {
		Dummy{}
	}
}

impl Prismatik for Dummy {
	fn light_count(&mut self) -> usize { 100 }
	fn lock(&mut self) -> bool { true }
	fn unlock(&mut self) -> bool { true }
	fn set_brightness(&mut self, _: usize) {}
	fn set_smooth(&mut self, _: usize) {}
	fn set_color(&mut self, _: usize, _: usize, _: usize, _:usize) {}
	fn set_on(&mut self, _: bool) {}
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

	fn light_count(&mut self) -> usize {
		100
	}

	fn lock(&mut self) -> bool {
		write!(self.stream, "lock\n").is_ok()
	}

	fn unlock(&mut self) -> bool {
		write!(self.stream, "unlock\n").is_ok()
	}

	fn set_brightness(&mut self, level: usize) {
		let brightness_string = format!("setbrightness:{}", level).to_string();
		write!(self.stream, "{}\n", brightness_string);
		self.flush();
	}

	fn set_smooth(&mut self, level: usize) {
		let smooth_string = format!("setsmooth:{}", level).to_string();
		write!(self.stream, "{}\n", smooth_string);
		self.flush();
	}

	fn set_color(&mut self, id: usize, r: usize, g: usize, b:usize) {
		let color_string = format!("setcolor:{}-{},{},{};", id, r, g, b).to_string();
		write!(self.stream, "{}\n", color_string);
		self.flush();
	}

	fn set_on(&mut self, on: bool) {
		let status_string = format!("setstatus:{}", if on { "on" } else { "off" }).to_string();
		write!(self.stream, "{}\n", status_string);
		self.flush();
	}
}
