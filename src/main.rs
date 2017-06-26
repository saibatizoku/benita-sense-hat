extern crate chrono;
extern crate sensehat;

use chrono::{DateTime, Utc};
use sensehat::SenseHat;

fn main() {
    let mut sense_hat = SenseHat::new().expect("Couldn't create Sense Hat object");
    loop {
        let dt: DateTime<Utc> = Utc::now();
        let temp = sense_hat
            .get_temperature_from_humidity()
            .expect("Couldn't get temp");
        let press_temp = sense_hat
            .get_temperature_from_pressure()
            .expect("Couldn't get temp");
        let rh = sense_hat.get_humidity().expect("Couldn't get rh");
        let pressure = sense_hat.get_pressure().expect("Couldn't get pressure");
        print!("'{:?}',", dt);
        print!("'HUMIDITY: {} {}',", temp, rh);
        println!("'PRESSURE: {} {}'", press_temp, pressure);
        ::std::thread::sleep(::std::time::Duration::from_millis(60000));
    }
}
