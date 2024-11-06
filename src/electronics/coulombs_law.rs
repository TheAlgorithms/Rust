///   Apply Coulomb's Law on any three given values. These can be force, charge1,
///   charge2, or distance, and then in a Python dict return name/value pair of
///   the zero value.
///
///   Coulomb's Law states that the magnitude of the electrostatic force of
///   attraction or repulsion between two point charges is directly proportional
///   to the product of the magnitudes of charges and inversely proportional to
///   the square of the distance between them.
const COULOMBS_CONSTANT: f64 = 8.9875517923e9;
#[derive(PartialEq, Eq, Debug)]
pub enum CoulombsLawError {
    ExtraZeroArg(String),
    NegativeDistance(String),
    NoneZeroArg(String),
}
pub fn coulombs_law(
    force: f64,
    charge1: f64,
    charge2: f64,
    distance: f64,
) -> Result<String, CoulombsLawError> {
    let charge_product = (charge1 * charge2).abs();

    if invalid_arguments(force, charge1, charge2, distance) {
        return Err(CoulombsLawError::ExtraZeroArg(String::from(
            "One and only one argument must be 0",
        )));
    }

    if distance < 0.0 {
        return Err(CoulombsLawError::NegativeDistance(String::from(
            "Distance cannot be negative",
        )));
    }

    if force == 0.0 {
        return Ok(format!(
            "force: {}",
            calculate_force(charge_product, distance)
        ));
    } else if charge1 == 0.0 {
        return Ok(format!(
            "charge1: {}",
            calculate_charge(charge2, force, distance)
        ));
    } else if charge2 == 0.0 {
        return Ok(format!(
            "charge2: {}",
            calculate_charge(charge1, force, distance)
        ));
    } else if distance == 0.0 {
        return Ok(format!(
            "distance: {}",
            calculate_distance(charge_product, force)
        ));
    }

    Err(CoulombsLawError::NoneZeroArg(String::from(
        "Exactly one argument must be 0",
    )))
}
fn calculate_distance(charge_product: f64, force: f64) -> f64 {
    (COULOMBS_CONSTANT * charge_product / force.abs()).sqrt()
}
fn calculate_charge(charge: f64, force: f64, distance: f64) -> f64 {
    force.abs() * distance.powi(2) / (COULOMBS_CONSTANT * charge)
}
fn calculate_force(charge_product: f64, distance: f64) -> f64 {
    COULOMBS_CONSTANT * charge_product / distance.powi(2)
}

fn invalid_arguments(force: f64, charge1: f64, charge2: f64, distance: f64) -> bool {
    [force, charge1, charge2, distance]
        .iter()
        .filter(|&&x| x == 0.0)
        .count()
        != 1
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_calculate_force {
       ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let ((charge_product,distance), expected) = $inputs;
                    assert_eq!(calculate_force(charge_product,distance), expected);
                }
            )*
        }
    }

    macro_rules! test_calculate_charge {
       ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let ((charge,force,distance), expected) = $inputs;
                    assert_eq!(calculate_charge(charge,force,distance), expected);
                }
            )*
        }
    }
    macro_rules! test_calculate_distance {
       ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let ((charge_product,force), expected) = $inputs;
                    assert_eq!(calculate_distance(charge_product,force), expected);
                }
            )*
        }
    }

    macro_rules! test_invalid_arguments {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let ((force,charge1,charge2,distance), expected) = $inputs;
                    assert_eq!(invalid_arguments(force,charge1,charge2,distance), expected);
                }
            )*
        }
    }
    macro_rules! test_coulombs_law {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let ((force,charge1,charge2,distance), expected) = $inputs;
                    assert_eq!(coulombs_law(force,charge1,charge2,distance).unwrap(), String::from(expected));
                }
            )*
        }
    }

    macro_rules! test_coulombs_law_err {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let ((force,charge1,charge2,distance), expected) = $inputs;
                    assert_eq!(coulombs_law(force,charge1,charge2,distance).unwrap_err(), expected);
                }
            )*
        }
    }

    test_coulombs_law! {
        general_inputs1:((0.0, 3.0, 5.0, 2000.0), "force: 33703.319221125"),
        general_inputs2:((10.0, 3.0, 5.0, 0.0), "distance: 116109.11974711547"),
        general_inputs3:((10.0, 0.0, 5.0, 2000.0), "charge1: 0.0008901200443544508"),
        general_inputs4:((10.0, 5.0,0.0 , 2000.0), "charge2: 0.0008901200443544508"),
    }

    test_coulombs_law_err! {
       extra_zero_arg_err: ((0.0, 3.0, 0.0, 2000.0), CoulombsLawError::ExtraZeroArg(String::from("One and only one argument must be 0"))),
       negative_distance_err: ((0.0, 3.0, 5.0, -2000.0), CoulombsLawError::NegativeDistance(String::from("Distance cannot be negative"))),
    }

    test_invalid_arguments! {
       valid_argument_input: ((0.0, 3.0, 5.0, 2000.0), false),
       invalid_argument_input: ((0.0, 0.0, 5.0, 2000.0), true),
       all_argument_zero: ((0.0, 0.0, 0.0, 0.0), true),
    }

    test_calculate_force! {
        force_test1: ((15.0,2000.0),33703.319221125),
        force_test2: ((18.0,0.12),11234439740375.0),
    }

    test_calculate_charge! {
        charge_test1: ((15.0,6.0,2000.0),0.00017802400887089018),
        charge_test2: ((18.0,3.0,0.12),2.6703601330633526e-13),
    }

    test_calculate_distance! {
        distance_test1: ((15.0,5.0),164203.09186157244),
        distance_test2: ((18.0,11.0),121272.02040394374),
    }
}
