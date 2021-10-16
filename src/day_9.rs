/*
data is a string with rainfall records of a few cities for months from January to December. The records of towns are separated by \n. The name of each town is followed by :.

data and towns can be seen in "Your Test Cases:".

Task:
- function: mean(town, strng) should return the average of rainfall for the city town and the strng data or data1 (In R and Julia this function is called avg).
- function: variance(town, strng) should return the variance of rainfall for the city town and the strng data or data1.

Examples:
mean("London", data), 51.19(9999999999996)
variance("London", data), 57.42(833333333374)
*/

use std::collections::HashMap;

pub fn mean(town: &str, strng: &str) -> f64 {
    match get_temperature_record(town, strng) {
        Some((temperature_vec, count)) => {
            temperature_vec.iter().fold(0.0, |mut acc, x| {
                acc += x;
                acc
            }) / count as f64
        }
        None => -1.0,
    }
}

pub fn variance(town: &str, strng: &str) -> f64 {
    let mean = mean(town, strng);

    match get_temperature_record(town, strng) {
        Some((temperature_vec, count)) => {
            temperature_vec.iter().fold(0.0, |mut acc, x| {
                acc += (x - mean).powf(2.0);
                acc
            }) / count as f64
        }
        None => -1.0,
    }
}

fn get_temperature_record(town: &str, strng: &str) -> Option<(Vec<f64>, i32)> {
    let weather_dir =
        strng
            .split("\n")
            .collect::<Vec<_>>()
            .iter()
            .fold(HashMap::new(), |mut map, c| {
                let w = c.split(":").collect::<Vec<_>>();
                if w.len() == 2 {
                    map.insert(w[0], w[1]);
                }
                map
            });

    match weather_dir.get(town) {
        Some(data) => Some(data.split(",").collect::<Vec<_>>().iter().fold(
            (Vec::new(), 0),
            |(mut temp, mut count), data| {
                let d = data.split(" ").collect::<Vec<_>>();
                temp.push(d[1].parse::<f64>().unwrap());
                count += 1;
                (temp, count)
            },
        )),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_float_equals(actual: f64, expected: f64) {
        let merr = 1.0e-2;
        let abs = (actual - expected).abs();
        let res = abs <= merr;
        println!("RES {}, {}. {}. ", res, actual, expected);
        assert!(
            res,
            format!(
                "Expected value must be near: {:e} but was:{:e}",
                expected, actual
            )
        );
    }

    fn dotest(town: &str, strng: &str, exp_mean: f64, exp_variance: f64) -> () {
        let m = mean(town, strng);
        let v = variance(town, strng);
        assert_float_equals(m, exp_mean);
        assert_float_equals(v, exp_variance);
    }

    fn da_ta() -> String {
        // don't move the string below

        let dr0 = r#"Rome:Jan 81.2,Feb 63.2,Mar 70.3,Apr 55.7,May 53.0,Jun 36.4,Jul 17.5,Aug 27.5,Sep 60.9,Oct 117.7,Nov 111.0,Dec 97.9
London:Jan 48.0,Feb 38.9,Mar 39.9,Apr 42.2,May 47.3,Jun 52.1,Jul 59.5,Aug 57.2,Sep 55.4,Oct 62.0,Nov 59.0,Dec 52.9
Paris:Jan 182.3,Feb 120.6,Mar 158.1,Apr 204.9,May 323.1,Jun 300.5,Jul 236.8,Aug 192.9,Sep 66.3,Oct 63.3,Nov 83.2,Dec 154.7
NY:Jan 108.7,Feb 101.8,Mar 131.9,Apr 93.5,May 98.8,Jun 93.6,Jul 102.2,Aug 131.8,Sep 92.0,Oct 82.3,Nov 107.8,Dec 94.2
Vancouver:Jan 145.7,Feb 121.4,Mar 102.3,Apr 69.2,May 55.8,Jun 47.1,Jul 31.3,Aug 37.0,Sep 59.6,Oct 116.3,Nov 154.6,Dec 171.5
Sydney:Jan 103.4,Feb 111.0,Mar 131.3,Apr 129.7,May 123.0,Jun 129.2,Jul 102.8,Aug 80.3,Sep 69.3,Oct 82.6,Nov 81.4,Dec 78.2
Bangkok:Jan 10.6,Feb 28.2,Mar 30.7,Apr 71.8,May 189.4,Jun 151.7,Jul 158.2,Aug 187.0,Sep 319.9,Oct 230.8,Nov 57.3,Dec 9.4
Tokyo:Jan 49.9,Feb 71.5,Mar 106.4,Apr 129.2,May 144.0,Jun 176.0,Jul 135.6,Aug 148.5,Sep 216.4,Oct 194.1,Nov 95.6,Dec 54.4
Beijing:Jan 3.9,Feb 4.7,Mar 8.2,Apr 18.4,May 33.0,Jun 78.1,Jul 224.3,Aug 170.0,Sep 58.4,Oct 18.0,Nov 9.3,Dec 2.7
Lima:Jan 1.2,Feb 0.9,Mar 0.7,Apr 0.4,May 0.6,Jun 1.8,Jul 4.4,Aug 3.1,Sep 3.3,Oct 1.7,Nov 0.5,Dec 0.7
"#;
        return String::from(dr0);
    }

    #[test]
    fn basic_tests() {
        let da_ta = &da_ta();
        dotest("Rome", da_ta, 66.02499999999999, 915.3852083333335);
        dotest("London", da_ta, 51.199999999999996, 57.428333333333335);
        dotest("Paris", da_ta, 173.89166666666665, 6594.412430555556);
        dotest("NY", da_ta, 103.21666666666665, 212.0730555555556);
        dotest("Vancouver", da_ta, 92.64999999999999, 2190.3425);
        dotest("Sydney", da_ta, 101.85000000000001, 483.1075000000001);
        dotest("Bangkok", da_ta, 120.41666666666667, 9238.419722222223);
        dotest("Tokyo", da_ta, 126.8, 2619.256666666667);
        dotest("Beijing", da_ta, 52.416666666666664, 4808.37138888889);
        dotest("Lima", da_ta, 1.6083333333333332, 1.5790972222222222);
        dotest("Delhi", da_ta, -1.0, -1.0);
    }
}
