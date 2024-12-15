use aoc_lib::year::Years;

pub fn years() -> Years {
    let mut years = Years::new();

    years.add_year(year_2024::year());

    years
}