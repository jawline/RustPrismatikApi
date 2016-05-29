use std::io::prelude::*;
use std::net::TcpStream;

trait Prismatik {

	pub fn light_count(&mut self);
	pub fn lock(&mut self);
	pub fn unlock(&mut self);

	pub fn set_brightness(&mut self, level: usize);
	pub fn set_smooth(&mut self, level: usize);
	pub fn set_color(&mut self, id: usize, r: usize, g: usize, b:usize);
	pub fn set_on(&mut self, on: bool);
}

pub fn set_all_lights(&mut api: Prismatik, r: usize, g: usize, b: usize) {
	let count = api.light_count();
	for id in 0..count {
		api.set_color(id, r, g, b);
	}
}

struct Dummy;

impl Primsatik for Dummy {
	pub fn light_count(&mut self) { 100 }
	pub fn lock(&mut self) {}
	pub fn unlock(&mut self) {}
	pub fn set_brightness(&mut self, level: usize) {}
	pub fn set_smooth(&mut self, level: usize) {}
	pub fn set_color(&mut self, id: usize, r: usize, g: usize, b:usize) {}
	pub fn set_on(&mut self, on: bool) {}
}

pub struct CoreApi {
	stream: Option<TcpStream>
}

impl CoreApi {

	pub fn new(path: &str, key: &str) -> Prismatik {

		let mut prism = Prismatik {
			stream: TcpStream::connect(path).unwrap()
		};
		
		prism.send_key(key);
		prism.lock();
		prism
	}

	pub fn flush(&mut self) {
		self.stream.flush();
	}

	pub fn send_key(&mut self, key: &str) {
		let key_string = "apikey:{".to_string() + key + "}";
		write!(self.stream, "{}\n", key_string);
		self.flush();
	}
}

impl Prismatik for CoreApi {

	pub fn light_count(&mut self) -> usize {
		100
	}

	fn lock(&mut self) {
		let lock_string = "lock".to_string();
		write!(self.stream, "{}\n", lock_string);
		self.flush();
	}

	fn set_brightness(&mut self, level: usize) {
		let brightness_string = "setbrightness:".to_string() + &level.to_string();
		write!(self.stream, "{}\n", brightness_string);
		self.flush();
	}

	fn set_smooth(&mut self, level: usize) {
		let smooth_string = "setsmooth:".to_string() + &level.to_string();
		write!(self.stream, "{}\n", smooth_string);
		self.flush();
	}

	fn set_color(&mut self, id: usize, r: usize, g: usize, b:usize) {
		let color_string = "setcolor:".to_string() + &id.to_string() + "-" + &r.to_string() + "," + &g.to_string() + "," + &b.to_string() + ";";
		write!(self.stream, "{}\n", color_string);
		self.flush();
	}

	fn set_on(&mut self, on: bool) {
		let status_string = "setstatus:".to_string() + if on { "on" } else { "off" };
		write!(self.stream, "{}\n", status_string);
		self.flush();
	}
}