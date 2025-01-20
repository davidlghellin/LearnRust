use csv::Reader;
use std::{fs::File, sync::Arc};

#[derive(Debug, serde::Deserialize)]
struct SalairesCSV {
    id: i32,
    work_year: i32,
    experience_level: String,
    employment_type: String,
    job_title: String,
    salary: i32,
    salary_currency: String,
    salary_in_usd: i32,
    employee_residence: String,
    remote_ratio: i32,
    company_location: String,
    company_size: String,
}

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    let guard = pprof::ProfilerGuard::new(100).unwrap();

    print_csv(csv::Reader::from_path("ds_salaries.csv").unwrap());

    let mut rdr = csv::Reader::from_path("ds_salaries.csv").unwrap();
    let mut salarios: Vec<SalairesCSV> = Vec::new();
    for result in rdr.deserialize() {
        let record: SalairesCSV = result.unwrap();
        salarios.push(record);
    }
    println!("{:?}", salarios.len());
    //println!("{:?}", suma_id(salarios));

    let arc_salarios: Arc<[SalairesCSV]> = salarios.into();
    // let arc_salarios2: Arc<[SalairesCSV]> = Arc::from(salarios.into_boxed_slice());
    println!("{:?}", suma_id_arc(arc_salarios));

    if let Ok(report) = guard.report().build() {
        let file = File::create("flamegraph_manual.svg").unwrap();
        report.flamegraph(file).unwrap();

        println!("report: {:?}", &report);
    };
}

fn print_csv(mut rdr: Reader<File>) {
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: SalairesCSV = result.unwrap();
        println!("{:?}", record);
    }
}

fn suma_id(salarios: Vec<SalairesCSV>) -> i32 {
    let mut sum = 0;
    for salario in salarios {
        sum += salario.id;
    }
    sum
}

fn suma_id_arc(salarios: Arc<[SalairesCSV]>) -> i32 {
    let mut sum = 0;
    for salario in salarios.iter() {
        sum += salario.id;
    }
    sum
}
