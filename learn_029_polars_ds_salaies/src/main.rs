use polars::prelude::*;
use std::fs::File;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    let guard = pprof::ProfilerGuard::new(100).unwrap();
    // Lee el archivo csv
    let csv: CsvReader<File> = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("ds_salaries.csv".into()))
        .unwrap();

    // Convierte el archivo csv en un LazyFrame
    let df: LazyFrame = csv.finish().unwrap().lazy();

    println!("{:?}", df.clone().collect());

    let count: DataFrame = df
        .clone()
        .filter(col("employment_type").is_null())
        .select(&[col("employment_type").count().alias("null count")])
        .collect()
        .unwrap();

    println!("{:?}", count);

    let df_group = df
        .clone()
        .filter(
            col("employee_residence")
                .eq(lit("US"))
                .and(col("job_title").eq(lit("Data Scientist"))),
        )
        .group_by([col("company_location"), col("experience_level")])
        .agg([
            col("salary_in_usd").mean().alias("average_salary"),
            col("salary_in_usd").max().alias("max_salary"),
            col("salary_in_usd").min().alias("min_salary"),
        ])
        .sort_by_exprs(vec![col("experience_level")], Default::default());
    println!("{:?}", df_group.explain(false));
    println!("{:?}", df_group.explain(true));
    println!("{:?}", df_group.collect());


    if let Ok(report) = guard.report().build() {
        let file = File::create("flamegraph.svg").unwrap();
        report.flamegraph(file).unwrap();

        println!("report: {:?}", &report);
    };
}
