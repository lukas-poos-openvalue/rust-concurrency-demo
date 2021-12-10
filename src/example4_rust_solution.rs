use std::sync::*;

fn main() {
	let value = 0;
	let value = Mutex::new(value);
	let value = Arc::new(value);
	let mut thread_vec = vec![];

	for _ in 0..10 {
		let thread_value = value.clone();
		let thread = std::thread::spawn(move || {
			for _ in 0..10000 {
				if let Ok(mut val) = thread_value.lock() {
					*val = *val + 1;
				}
			}
		});
		thread_vec.push(thread);
	}

	for thread in thread_vec {
		thread.join().unwrap();
	}

	println!("Value of counter.value: {}", value.lock().unwrap());
}

