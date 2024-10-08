use bitstring_trees::map::Map;
use cidr::{
	AnyIpCidr,
	IpCidr,
};
use clap::Parser;
use std::{
	io::{
		self,
		BufRead,
	},
	str::FromStr,
};

fn split_line(line: &str) -> (&str, &str) {
	let line = line.trim();
	match line.find(char::is_whitespace) {
		Some(pos) => {
			let key = &line[..pos];
			let rem = &line[pos + 1..];
			for (pos2, c) in rem.char_indices() {
				if !c.is_whitespace() {
					return (key, &rem[pos2..]);
				}
			}
			unreachable!();
		},
		None => (line, ""),
	}
}

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
	/// Group into smallest number of prefixes (default)
	#[arg(long, short)]
	prefix: bool, // value unused, just for "conflict with"

	/// Group into smallest number of a-b ranges
	#[arg(conflicts_with("prefix"))]
	#[arg(long, short)]
	range: bool,

	/// also show IPs with no value (only for prefix output)
	#[arg(conflicts_with("range"))]
	#[arg(long, short)]
	unset: bool,
}

fn main() {
	let args = Args::parse();

	let mut map = Map::<AnyIpCidr, String>::new();
	let stdin = io::stdin();
	let input = stdin.lock();
	for line in input.lines() {
		let line = line.unwrap();
		let line = line.as_str().trim();
		if line.is_empty() || line.starts_with('#') {
			continue;
		}
		let (cidr, value) = split_line(line);
		let cidr = match AnyIpCidr::from_str(cidr) {
			Ok(cidr) => cidr,
			Err(e) => panic!("Failed to parse {:?} as CIDR: {}", cidr, e),
		};
		map.insert(cidr, value.to_string());
	}

	if args.range {
		let mut prev = None;
		for (key, value) in map.iter_full() {
			let key: IpCidr = match key.into() {
				None => {
					// any
					if let Some(value) = value {
						println!("{} => {}", key, value);
					}
					continue;
				},
				Some(key) => key,
			};
			prev = match prev {
				None => Some((key, key.last_address(), value)),
				Some((first_range, last, prev_value)) => {
					if prev_value == value && first_range.family() == key.family() {
						Some((first_range, key.last_address(), prev_value))
					} else {
						if let Some(prev_value) = prev_value {
							println!("{}-{}\t{}", first_range.first_address(), last, prev_value);
						}
						Some((key, key.last_address(), value))
					}
				},
			}
		}
		if let Some((first_range, last, Some(prev_value))) = prev {
			println!("{}-{}\t{}", first_range.first_address(), last, prev_value);
		}
	} else {
		for (key, value) in map.iter_full() {
			match value {
				Some(value) => {
					println!("{}\t{}", key, value);
				},
				None if args.unset => {
					println!("{}", key);
				},
				_ => (),
			}
		}
	}
}
