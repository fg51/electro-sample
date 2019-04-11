use std::f64::consts::PI;

fn main() {
    let radius_inn = 1.;
    let radius_out = 1.5;
    let relative_permittivity = 2.3;
    println!("cable inner radius: {0:.02} [mm]", radius_inn);
    println!("cable outer radius: {0:.02} [mm]", radius_out);

    let c = to_capacitance(relative_permittivity, radius_inn, radius_out);
    let msg = format!("capacitance: {0:.02E} [uF/m]", c * 1E+6);
    println!("{}", msg);

    let l = to_inductance(radius_inn, radius_out);
    let msg = format!("inductance : {0:.02E} [mH/m]", l * 1E+3);
    println!("{}", msg);
}

fn to_capacitance(relative_permittivity: f64, radius_inn: f64, radius_out: f64) -> f64 {
    let epsilon0 = 8.853E-12;
    let ratio = radius_out / radius_inn;
    return (2. * PI * relative_permittivity * epsilon0) / ratio.ln();
}

fn to_inductance(radius_inn: f64, radius_out: f64) -> f64 {
    let mu0 = 4. * PI * 1E-7;
    let ratio = radius_out / radius_inn;
    return mu0 / (2. * PI) * (1. / 4. + ratio.ln());
}
