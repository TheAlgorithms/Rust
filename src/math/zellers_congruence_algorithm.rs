// returns the day of the week from the Gregorian Date

pub fn zellers_congruence_algorithm(date: i32, month: i32, year: i32, as_string: bool) -> String {
    let q = date;
    let (m, y) = if month < 3 {
        (month + 12, year - 1)
    } else {
        (month, year)
    };
    let day: i32 =
        (q + (26 * (m + 1) / 10) + (y % 100) + ((y % 100) / 4) + ((y / 100) / 4) + (5 * (y / 100)))
            % 7;
    if as_string {
        number_to_day(day)
    } else {
        day.to_string()
    }
    /* Note that the day follows the following guidelines:
    0 = Saturday
    1 = Sunday
    2 = Monday
    3 = Tuesday
    4 = Wednesday
    5 = Thursday
    6 = Friday
    */
}

fn number_to_day(number: i32) -> String {
    let days = [
        "Saturday",
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
    ];
    String::from(days[number as usize])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(zellers_congruence_algorithm(25, 1, 2013, false), "6");
        assert_eq!(zellers_congruence_algorithm(25, 1, 2013, true), "Friday");
        assert_eq!(zellers_congruence_algorithm(16, 4, 2022, false), "0");
        assert_eq!(zellers_congruence_algorithm(16, 4, 2022, true), "Saturday");
        assert_eq!(zellers_congruence_algorithm(14, 12, 1978, false), "5");
        assert_eq!(zellers_congruence_algorithm(15, 6, 2021, false), "3");
    }
}
