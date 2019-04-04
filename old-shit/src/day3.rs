use csv::Writer;

#[derive(Serialize)]
struct Movie {
    title: String,
    bad_guy: String,
    pub_year: usize
}

pub fn write_csv() {
    let dollar_films = vec![
        Movie{
            title: "A Fistful of Dollars".to_string(), 
            bad_guy: "Rojo".to_string(), 
            pub_year: 1964
            },
        Movie {
            title: "For a Few Dollars More".to_string(), 
            bad_guy: "El Indio".to_string(), 
            pub_year: 1965
        },
        Movie {
            title: "The Good, the Bad and the Ugly".to_string(), 
            bad_guy: "Tuco".to_string(), 
            pub_year: 1966
            },
    ];

    let path = "westerns.csv";
    let mut writer = Writer::from_path(path).unwrap();
    for row in dollar_films {
        // writer.write_record(&row).expect("CSV writer error");
        // This is how we handle errors here 
        writer.serialize(row).expect("CSV serialization error");
    }
    writer.flush().expect("CSV flushing error");
}
