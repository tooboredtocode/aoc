mod y2024;

use aoc_lib::year::Years;

pub fn years() -> Years {
    let mut years = Years::new();

    years.add_year(y2024::year2024());

    years
}