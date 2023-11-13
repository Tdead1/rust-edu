use std::net::TcpListener;
use std::thread::{self, JoinHandle};
use std::time::Instant;

const FINALPORT :i16 = 9999;
// Basic non-threaded application to check if we can get any TCP responses from a network device.
fn check_ports()
{
	let mut last_printed_address = 0;
	let mut last_open = false;

	for ip_last_digits in 0..255
	{
		let ip = format!("192.168.0.{}", ip_last_digits);
		let mut output_string = String::new();
		output_string.push_str( format!("Checking IP {} \n", ip).as_str());

		for port in 1..FINALPORT 
		{
			let open = match TcpListener::bind((ip.clone(), port as u16)) {
				Ok(_) => true,
				Err(_) => false,
			};
			if last_open != open 
			{
				if last_printed_address == port - 1 {
					output_string.push_str(format!("Address {} is {}.\n"
					, port - 1
					, if last_open { "open" } else { "closed" }).as_str());
				}
				else {
					output_string.push_str(format!("Address {} to {} addresses are {}.\n"
					, last_printed_address
					, port - 1
					, if last_open { "open" } else { "closed" }).as_str());
				}
				last_printed_address = port;
			}
			last_open = open;
		}

		output_string.push_str(format!("Address {} to {} addresses are {}. \n"
		, last_printed_address
		, FINALPORT
		, if last_open { "open" } else { "closed" }).as_str());

		print!("{}", output_string);
	}
	println!("Finished query.");
}

// This only spawns two threads, which isn't very useful but at least I learned the threading syntax from it.
fn check_ports_threaded()
{
	for ip_last_digits in 0..255
	{
		let thead_handle = thread::spawn(move|| 
		{
			let mut output = String::new();
			let ip = format!("192.168.0.{}", ip_last_digits);
			output.push_str(format!("Checking IP {} \n", ip).as_str());

			for port in 1..FINALPORT 
			{
				let open = match TcpListener::bind((ip.clone(), port as u16)) 
				{
					Ok(_) => true,
					Err(_) => false,
				};
				if open
				{
					output.push_str(format!("Address 192.168.0.{}:{} is {}. \n", ip_last_digits, port, if open { "open" } else { "closed" }).as_str());
				}
			}
			return output;
		});

		print!("{}", thead_handle.join().unwrap());
	}
}

// Spawning more threads for each ip address as well.
fn check_ports_threaded_optimized()
{
	let final_handle = 
	thread::spawn(move||
	{
		let mut open_and_closed_ports = (String::new(), Vec::new());
		for ip_last_digits in 0..255
		{
			println!("Tackling digit {}", ip_last_digits.clone());
			let thead_handle = 
			thread::spawn(move || 
			{
				let ip: String = format!("192.168.0.{}", ip_last_digits);
				let mut open_ports: Vec<i16> = Vec::new();
				open_ports.reserve(1000);

				let mut closed_ports: Vec<i16> = Vec::new();
				closed_ports.reserve(1000);

				for port in 1..FINALPORT 
				{
					if match TcpListener::bind((ip.clone(), port as u16)) { Ok(_) => true, Err(_) => false, }
					{
						open_ports.push(port);
					}
					else 
					{
						closed_ports.push(port);
					};
				}
				return (open_ports, closed_ports);
			});
			open_and_closed_ports.0 = format!("192.168.0.{}", ip_last_digits);
			open_and_closed_ports.1.push(thead_handle.join());
		}
		return open_and_closed_ports; 
	});

	let result = final_handle.join().unwrap();
	let ip = result.0;
	let data = result.1;
	for result in data
	{
		for open_port in result.unwrap().0
		{
			println!("Open port found: {}:{}", ip, open_port);
		}
	}
}

// I misunderstood threads, here is an example of using them correctly:
fn check_ports_threaded_fixed()
{
	for ip_last_digits in 0..255
	{
		println!("Tackling digit {}", ip_last_digits.clone());
		let mut output = String::new();
		let ip = format!("192.168.0.{}", ip_last_digits);
		output.push_str(format!("Checking IP {} \n", ip.clone()).as_str());
		
		let mut threads:Vec<JoinHandle<(i16, bool)>> = Vec::new(); 
		for port in 1..FINALPORT 
		{
			threads.push(thread::spawn(move || {
				return (port, match TcpListener::bind((format!("192.168.0.{}", ip_last_digits), port as u16)){ Ok(_) => true, Err(_) => false, })
			}));
		}
		for i in threads
		{
			let result = i.join().unwrap();
			if result.1
			{
				println!("Open port found: 192.168.0.{}, {}", ip_last_digits, result.0);
			}
		}
	}
}

// This will eventually complete, but uh... Yeah. Made the amounts of ports to check 15, otherwise I'm taxing my pc too much.
// Could potentially be used if I had a near-infinite amount of threads available, like a server botnet or something.
fn check_ports_threaded_fixed_extreme()
{
	let mut base_threads: Vec<JoinHandle<(i32, Vec<JoinHandle<(i16, bool)>>)>> = Vec::new(); 
	
	for ip_last_digits in 0..15
	{
		base_threads.push(thread::spawn(move || 
		{
			println!("Tackling digit {}", ip_last_digits.clone());
			let mut output = String::new();
			let ip = format!("192.168.0.{}", ip_last_digits);
			output.push_str(format!("Checking IP {} \n", ip.clone()).as_str());
			
			let mut threads:Vec<JoinHandle<(i16, bool)>> = Vec::new(); 
			for port in 1..FINALPORT 
			{
				threads.push(thread::spawn(move || {
					return (port, match TcpListener::bind((format!("192.168.0.{}", ip_last_digits), port as u16)){ Ok(_) => true, Err(_) => false, })
				}));
			}
			return (ip_last_digits.clone(), threads);
		}));
	}

	for base_thread in base_threads
	{
		print!("Still alive!\n");
		let base_thread_results = base_thread.join().unwrap();
		for base_thread_result in base_thread_results.1
		{
			print!("Working hard!\n");
			let result = base_thread_result.join().unwrap();
			let open = result.1;
			let port = result.0;
			if open
			{
				println!("Open port found: 192.168.0.{}:{}", base_thread_results.0, port);
			}
		}
	}
}

// For a single pc, by far the best solution. One thread per IP, which checks 9999 ports each.
fn check_ports_ultimate()
{
	let mut base_threads: Vec<JoinHandle<(i32, Vec<(i16, bool)>)>> = Vec::new(); 
	
	for ip_last_digits in 0..255
	{
		base_threads.push(thread::spawn(move || 
		{
			println!("Tackling digit {}", ip_last_digits.clone());
			let mut output = String::new();
			let ip = format!("192.168.0.{}", ip_last_digits);
			output.push_str(format!("Checking IP {} \n", ip.clone()).as_str());
			
			let mut results : Vec<(i16, bool)> = Vec::new(); 
			for port in 1..FINALPORT 
			{
				results.push((port, match TcpListener::bind((format!("192.168.0.{}", ip_last_digits), port as u16)){ Ok(_) => true, Err(_) => false, }));
			}
			return (ip_last_digits.clone(), results);
		}));
	}

	for base_thread in base_threads
	{
		let base_thread_results = base_thread.join().unwrap();
		for vector_result in base_thread_results.1
		{
			let open = vector_result.1;
			let port = vector_result.0;
			if open
			{
				println!("Open port found: 192.168.0.{}:{}", base_thread_results.0, port);
			}
		}
	}
}

fn main() 
{
	let first_check_start = Instant::now();
	check_ports();
	let first_check_duration = first_check_start.elapsed().as_millis();
	print!("{esc}c", esc = 27 as char);
	
	let second_check_start = Instant::now();
	check_ports_threaded();
	let second_check_duration = second_check_start.elapsed().as_millis();
	print!("{esc}c", esc = 27 as char);
	
	let third_check_start = Instant::now();
	check_ports_threaded_optimized();
	let third_check_duration = third_check_start.elapsed().as_millis();
	print!("{esc}c", esc = 27 as char);
	
	let fourth_check_start = Instant::now();
	check_ports_threaded_fixed();
	let fourth_check_duration = fourth_check_start.elapsed().as_millis();
	print!("{esc}c", esc = 27 as char);

	let fifth_check_start = Instant::now();
	check_ports_threaded_fixed_extreme();
	let fifth_check_duration = fifth_check_start.elapsed().as_millis();
	print!("{esc}c", esc = 27 as char);

	let sixth_check_start = Instant::now();
	check_ports_ultimate();
	let sixth_check_duration = sixth_check_start.elapsed().as_millis();
	print!("{esc}c", esc = 27 as char);

	println!("\n\n timings (s):");
	println!("1th duration: {:.4} seconds", (first_check_duration as f64) * 0.001);
	println!("2th duration: {:.4} seconds", (second_check_duration as f64) * 0.001);
	println!("3th duration: {:.4} seconds", (third_check_duration as f64) * 0.001);
	println!("4th duration: {:.4} seconds", (fourth_check_duration as f64) * 0.001);
	println!("5th duration: {:.4} seconds", (fifth_check_duration as f64) * 0.001);
	println!("6th duration: {:.4} seconds", (sixth_check_duration as f64) * 0.001);
}
