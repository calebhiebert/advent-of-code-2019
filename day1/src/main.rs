use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let lines: Vec<f64> = buf
        .lines()
        .filter_map(|ln| {
            match ln.parse::<i64>() {
                Ok(n) => Some(n as f64),
                Err(_e) => None
            }
        }).collect();

    let mut fuel_requirements = 0;

    lines.iter().for_each(|m| {
        let fuel = (m / 3.0).floor() as i32 - 2;
        fuel_requirements += fuel + additional_fuel_requirements(fuel);
    });

    println!("Fuel Required {}", fuel_requirements);
}

fn additional_fuel_requirements(val: i32) -> i32 {
    let mut req = (val as f64 / 3.0).floor() as i32 - 2;

    if req > 0 {
        req += additional_fuel_requirements(req);
    }

    req.max(0)
}

#[test]
fn test_additional_fuel_requirements() {
    assert_eq!(additional_fuel_requirements(1969), 966)
}