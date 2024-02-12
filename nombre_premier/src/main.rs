fn main() {
	let start = std::time::Instant::now();

	let n: u64 = 10000000;

	let mut n_premier = vec![];

    'big: for nombre in 2..n {
	for x in n_premier.iter() {
		if nombre % x == 0 {
			continue 'big;
		}
	}
        
        println!("{}", nombre);
	n_premier.push(nombre);
    }

	let time = start.elapsed().as_secs_f64();
	println!("A pris {} secondes, vitesse {}", time, n as f64 / time);
}
