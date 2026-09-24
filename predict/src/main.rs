use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct Car {
	km: u32,
	price: u32
}

#[derive(Debug, Deserialize)]
struct NormalizedCar {
	km: f64,
	price: u32
}

fn reader(_file: &str) -> Result<Vec<Car>, csv::Error> {
	let mut rdr = csv::Reader::from_path("../data.csv")?;
	let mut result: Vec<Car> = Vec::new();

	for r in rdr.deserialize() {
		let car: Car = r?;
		result.push(car);
	}

	Ok(result)
}

fn parser(values: Result<Vec<Car>, csv::Error>) -> Result<Vec<NormalizedCar>, csv::Error> {
	let val = values?;
	let mut result: Vec<NormalizedCar> = Vec::new();
	let mut min: u32 = val[0].km;
	let mut max: u32 = val[0].km;

	for car in val.iter().clone() {
		if car.km > max {
			max = car.km;
		}
		else if car.km < min {
			min = car.km;
		}
	}
	
	println!("max: {}, min {}", max, min);

	for car in val {
		let tmp: NormalizedCar = NormalizedCar { km: normalize(car.km, min, max), price: car.price };
		result.push(tmp);
	}
	
	Ok(result)
}

fn normalize(x: u32, min: u32, max: u32) -> f64 {
	let tmp: f64 = x as f64;
	let tmp1: f64 = min as f64;
	let tmp2: f64 = max as f64;
	(tmp-tmp1)/(tmp2-tmp1)
}

fn epoch(values: Result<Vec<NormalizedCar>, csv::Error>, n: u32) -> Result<(f64, f64), csv::Error>{
	let val = values?;

	println!("NormalizedCar: {}", val[2].km);

	let l_rate: f64 = 0.9;
	let mut t0: f64 = 0.0;
	let mut t1: f64 = 0.0;
	let mut i: u32 = 0;

	while i < n {
		let mut pred: Vec<(f64, NormalizedCar)> = Vec::new();

		// calculate predictions
		for car in val.iter().clone() {
			let clone: NormalizedCar = NormalizedCar { km: car.km, price: car.price };
			pred.push((pricing(t0, t1, car.km), clone));
		}

		// calculate errors
		let mut errors: Vec<(f64, f64)> = Vec::new();
		for p in pred.iter().clone() {
			let tmp: f64 = p.1.price as f64;
			errors.push((p.0 - tmp, p.1.km));
		}

		// calculate thetas
		let mut sum0: f64 = 0.0;
		for e in errors.iter().clone() {
			let tmp: f64 = e.0 as f64;
			sum0 += tmp;
		}
		let mut sum1: f64 = 0.0;
		for e in errors.iter().clone() {
			let tmp: f64 = e.0 as f64;
			let mileage: f64 = e.1 as f64;
			sum1 += tmp * mileage;
		}

		// update thetas
		let tmp: f64 = 1.0/val.len() as f64;
		let tmp0 = l_rate * tmp * sum0;
		let tmp1 = l_rate * tmp * sum1;
		t0 -= tmp0;
		t1 -= tmp1;
		i += 1;

		if i%20 == 0 {
			println!("Errors: {}, thetas: {}, {}", sum0, t0, t1);
		}
	}

	Ok((t0, t1))
}

fn pricing(t0: f64, t1: f64, mileage: f64) -> f64 {
	t0 + t1 * mileage
}

fn _writer() {}

fn main() {
    println!("Hello, world!");
	let content: Result<Vec<Car>, csv::Error>  = reader("data.csv");
	let parsed: Result<Vec<NormalizedCar>, csv::Error> = parser(content);
	let _ = epoch(parsed, 1000);
}
