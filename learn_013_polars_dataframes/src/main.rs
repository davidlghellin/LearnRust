use polars::prelude::*;
//use polars::lazy::dsl::*;
//use polars_core::prelude::*;

fn main() {
    escribir_ejemplo_csv();

    let s1 = Column::new("Fruit".into(), ["Apple", "Apple", "Pear"]);
    let s2 = Column::new("Color".into(), ["Red", "Yellow", "Green"]);

    let df: PolarsResult<DataFrame> = DataFrame::new(vec![s1.clone(), s2.clone()]);
    println!("{:?}", df);

    let a = UInt32Chunked::new("a".into(), &[1, 2, 3]).into_column();
    let b = Float64Chunked::new("b".into(), &[10., 8., 6.]).into_column();

    let df: DataFrame = DataFrame::new(vec![a, b, s1, s2]).unwrap();
    println!("{:?}", &df);

    let idx = IdxCa::new("idx".into(), [0, 1]);
    println!("{:?}", &df.take(&idx));

    println!("{:?}", example(&df));
    println!("{:?}", &df.select(["a", "b"]));

    let csv_result: Result<CsvReader<std::fs::File>, PolarsError> = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("path.csv".into()));
    let csv: DataFrame = csv_result.unwrap().finish().unwrap();

    println!("{:?}", &csv);
    let other_csv = &csv.sort(["foo"], Default::default());
    println!("{:?}", other_csv);

    let lazy_csv = &csv
        .lazy()
        .sort(["foo"], Default::default())
        .drop(["agg"])
        .rename(vec!["bar"], vec!["new_bar"], true);
    //println!("{:?}", &lazy_csv.explain(true));
    let final_csv_df = lazy_csv.clone().collect();
    println!("{:?}", final_csv_df);
}

fn example(df: &DataFrame) -> PolarsResult<DataFrame> {
    df.select(["a", "b"])
}

fn escribir_ejemplo_csv() {
    let mut df = df!(
        "foo" => &[1, 2, 3],
        "bar" => &[None, Some("bak"), Some("baz")],
        "agg" => &[true, false, true]
    )
    .unwrap();

    let mut file = std::fs::File::create("path.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap()
}
