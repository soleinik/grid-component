mod data_table;
mod data_value;

pub use data_table::TableData;
pub use data_value::DataValue;

#[cfg(test)]
mod tests {

    use fake::{Dummy, Fake, Faker};
    use indexmap::IndexMap;

    use super::*;

    #[test]
    fn it_works() {
        let data = random_test_data();
        println!("rows:{}", data.len());

        let mut table = TableData::new(data.len(), table_meta());

        for (row, data) in data.iter().enumerate() {
            table.set_value(row, 0, data.strike);
            table.set_value(row, 1, data.last_price);
            table.set_value(row, 2, data.change);
            table.set_value(row, 3, data.percent_change);
            table.set_value(row, 4, data.volume);
            table.set_value(row, 5, data.open_interest);
            table.set_value(row, 6, data.bid);
            table.set_value(row, 7, data.ask);

            table.set_value(row, 8, data.expiration);
            table.set_value(row, 9, data.last_trade_date);
            table.set_value(row, 10, data.implied_volatility);
            table.set_value(row, 11, data.in_the_money);
        }

        println!("{}", table);
    }

    fn table_meta() -> IndexMap<String, usize> {
        IndexMap::from([
            ("strike123456789".to_string(), 6),
            ("last $".to_string(), 6),
            ("change".to_string(), 6),
            ("last %".to_string(), 6),
            ("volume".to_string(), 6),
            ("  OI  ".to_string(), 6),
            (" bid  ".to_string(), 6),
            (" ask  ".to_string(), 6),
            ("expiry".to_string(), 6),
            ("trdate".to_string(), 6),
            ("  IV  ".to_string(), 6),
            (" ITM  ".to_string(), 6),
        ])
    }

    fn random_test_data() -> Vec<TestData> {
        (10..20)
            //.step_by(10)
            .map(|x| random_test_row(x as f64))
            .collect()
    }

    fn random_test_row(strike: f64) -> TestData {
        let mut data = Faker.fake::<TestData>();
        data.strike = strike;
        data
    }
    #[derive(Debug, Dummy, Default)]
    struct TestData {
        pub strike: f64,
        #[dummy(faker = "10.0..20.0")]
        pub last_price: f64,
        #[dummy(faker = "0.0..5.0")]
        pub change: f64,
        #[dummy(faker = "-1.0..1.0")]
        pub percent_change: f64,
        #[dummy(faker = "1000..2000")]
        pub volume: u64,
        #[dummy(faker = "1000..2000")]
        pub open_interest: u64,
        #[dummy(faker = "10.0..20.0")]
        pub bid: f64,
        #[dummy(faker = "10.0..20.0")]
        pub ask: f64,
        #[dummy(faker = "1000..2000")]
        pub expiration: u64,
        #[dummy(faker = "1000..2000")]
        pub last_trade_date: u64,
        #[dummy(faker = "10.0..300.0")]
        pub implied_volatility: f64,
        #[dummy(faker = "true")]
        pub in_the_money: bool,
    }
}
