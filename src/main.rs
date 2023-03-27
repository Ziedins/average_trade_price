use std::{error::Error, io, process};

fn trades_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());

    let mut value_sum : f64 = 0.;
    let mut qty_sum : f64 = 0.;

    for result in rdr.records() {
        let record = result?;
        let value = record[1].trim().parse::<f64>().unwrap();
        let qty = record[2].trim().parse::<f64>().unwrap();
        qty_sum += qty;
        value_sum += value * qty;
    }

    let avg_price: f64 = value_sum / qty_sum;

    println!("value sum: {}, qty sum : {}", value_sum, qty_sum);
    println!("avg price weighed by qty {}", avg_price);

    Ok(())
}

fn main() {
    if let Err(err) = trades_csv() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
