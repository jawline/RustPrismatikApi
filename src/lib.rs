use std::io::prelude::*;
use std::net::TcpStream;

pub struct Prismatik {
	stream: TcpStream
}

impl Prismatik {
	pub fn new(path: &str, key: &str) -> Prismatik {
		let mut prism = Prismatik {
			stream: TcpStream::connect(path).unwrap()
		};
		prism.send_key(key);
		prism.lock();
		prism
	}

	pub fn light_count(&mut self) -> usize {
		100
	}

	pub fn flush(&mut self) {
		self.stream.flush();
	}

	pub fn send_key(&mut self, key: &str) {
		let key_string = format!("apikey:\{{}\}", key).to_string();
		write!(self.stream, "{}\n", key_string);
		self.flush();
	}

	pub fn lock(&mut self) {
		let lock_string = "lock".to_string();
		write!(self.stream, "{}\n", lock_string);
		self.flush();
	}

	pub fn set_brightness(&mut self, level: usize) {
		let brightness_string = format!("setbrightness:{}", level).to_string();
		write!(self.stream, "{}\n", brightness_string);
		self.flush();
	}

	pub fn set_smooth(&mut self, level: usize) {
		let smooth_string = format!("setsmooth:{}", level).to_string();
		write!(self.stream, "{}\n", smooth_string);
		self.flush();
	}

	pub fn set_color(&mut self, id: usize, r: usize, g: usize, b:usize) {
		let color_string = format!("setcolor:{}-{},{},{};", id, r, g, b).to_string();
		write!(self.stream, "{}\n", color_string);
		self.flush();
	}

	pub fn set_on(&mut self, on: bool) {
		let status_string = "setstatus:".to_string() + if on { "on" } else { "off" };
		write!(self.stream, "{}\n", status_string);
		self.flush();
	}

	pub fn set_all_lights(&mut self, r: usize, g: usize, b: usize) {
		let count = self.light_count();
		for id in 0..count {
			self.set_color(id, r, g, b);
		}
	}
}
