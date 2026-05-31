use std::io::{self, Write};

struct Planet {
    rho_o: f32,
    rho_outer: f32,
    r: f32,
    step: f32,
}

struct PlanetParam {
    prompt: &'static str,
    default: f32,
}

struct PlanetParams {
    rho_o: PlanetParam,
    rho_outer: PlanetParam,
    r: PlanetParam,
    step: PlanetParam,
}

enum PlanetParamType {
    RhoO,
    RhoOuter,
    R,
    Step,
}

fn set_value(param_type: &PlanetParamType, params: &PlanetParams) -> f32 {
    let current_type = match param_type {
        PlanetParamType::RhoO => &params.rho_o,
        PlanetParamType::RhoOuter => &params.rho_outer,
        PlanetParamType::R => &params.r,
        PlanetParamType::Step => &params.step,
    };

    let mut l = String::new();
    print!("{} [default={}]", current_type.prompt, current_type.default);
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut l)
        .expect("Invalid input. Reading failed");

    let m: f32 = l.trim().parse().unwrap_or_else(|_| {
        println!("Invalid input. Using default value");
        current_type.default
    });

    match param_type {
        PlanetParamType::RhoO | PlanetParamType::RhoOuter => m * 1.0e12,
        _ => m,
    }
}

fn main() {
    let planet_params = PlanetParams {
        rho_o: PlanetParam {
            prompt: "The core density[g/cm^3].",
            default: 4.5,
        },
        rho_outer: PlanetParam {
            prompt: "The outer density[g/cm^3].",
            default: 2.0,
        },
        r: PlanetParam {
            prompt: "The radius[km].",
            default: 6340.0,
        },
        step: PlanetParam {
            prompt: "The step size[km].",
            default: 1.0,
        },
    };

    // 入力の読み取り
    let planet = Planet {
        rho_o: set_value(&PlanetParamType::RhoO, &planet_params),
        rho_outer: set_value(&PlanetParamType::RhoOuter, &planet_params),
        r: set_value(&PlanetParamType::R, &planet_params),
        step: set_value(&PlanetParamType::Step, &planet_params),
    };
    println!("caluculating ...");

    // let mut grad_input: String = String::new();
    // print!("Step5: The density gradiation: [default: -x] ");
    // io::stdout().flush().unwrap();

    // io::stdin()
    //     .read_line(&mut grad_input)
    //     .expect("Invalid input. Reading failed");

    // let grad: &str = grad_input.trim();

    // ループの回数を定義 端数切り上げ
    let steps: u32 = planet.r as u32 / planet.step as u32 + 1;

    // 密度勾配を関数で定義  s: step, sc: step count
    fn get_density_at_r(planet: &Planet, sc: u32) -> f32 {
        let rho_r: f32 =
            planet.rho_o + (planet.rho_outer - planet.rho_o) / planet.r * planet.step * sc as f32;
        rho_r
    }

    // 体積を求める関数  s: step
    fn get_shell_volume_at_r(planet: &Planet, sc: u32) -> f64 {
        let s: f64 = planet.step as f64;
        let r: f64 = s * sc as f64;
        let v: f64 =
            4.0 / 3.0 * std::f64::consts::PI * (3.0 * r * r * s + 3.0 * r * s * s + s * s * s);
        v
    }

    let mut mass_total: f64 = 0.0;
    for i in 0..steps {
        let density: f32 = get_density_at_r(&planet, i);
        let volume: f64 = get_shell_volume_at_r(&planet, i);
        let mass_r: f64 = density as f64 * volume;
        mass_total += mass_r;
    }
    let earth_mass: f64 = 5.9742e24;
    let mass_ratio: f64 = mass_total / earth_mass;

    println!(
        "=======  caluculated  =======
Total Mass[kg]: {}
Mass Ratio: {}",
        mass_total, mass_ratio
    );
}
