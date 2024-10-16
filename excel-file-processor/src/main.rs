mod balance_calculation;
mod convert_excel_to_csv;
mod validation_check_parallal;
mod validation_check_synchornize;

fn main() {
    // convert_excel_to_csv::convert_excel_to_csv();
    // validation_check_parallal::validation_check();
    // validation_check_synchornize::validation_cehck();
    balance_calculation::balance_calculation();
}

// validation chekc parallal 120.79 s 2 minutes
// validation check synchornize way 499.53 s 8.3 minutes
// balance calculation synchornize way 499.53 s 8.3 minutes
