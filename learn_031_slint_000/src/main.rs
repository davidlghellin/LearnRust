slint::include_modules!();

const TAXPER: f64 = 0.21;

fn main() -> Result<(), slint::PlatformError> {
    let main_window: MainWindow = MainWindow::new()?;
    let ui_handler: slint::Weak<MainWindow> = main_window.as_weak();

    main_window.on_divide_income(move |string| {
        let ui: MainWindow = ui_handler.unwrap();
        let num: f64 = string.trim().parse().unwrap();

        let tax: f64 = num * TAXPER;
        let result: String = format!("Impuestos:{}\nGanancia: {}", tax, (num - tax));
        ui.set_results(result.into());
    });

    main_window.run()
}
