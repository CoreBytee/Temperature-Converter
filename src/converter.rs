pub fn celcius_to_farenheight(celcius: f64) -> f64 {
    (celcius * 9.0 / 5.0) + 32.0
}

pub fn farenheight_to_celcius(farenheight: f64) -> f64 {
    (farenheight - 32.0) * 5.0 / 9.0
}