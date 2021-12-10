struct Counter {
	value: i32
}

fn main() {
	let mut counter = Counter { value: 0 };
	let mut thread_vec = vec![];

	for _ in 0..10 {
		let thread = std::thread::spawn(|| {
			for _ in 0..10000 {
				counter.value = counter.value + 1;
			}
		});
		thread_vec.push(thread);
	}

	for thread in thread_vec {
		thread.join().unwrap();
	}

	println!("Value of counter.value: {}", counter.value);
}