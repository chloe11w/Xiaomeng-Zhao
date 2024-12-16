mod models;
mod utils;
mod analysis;
mod test; 

use analysis::{
    analyze_zipcode_frequency,
    analyze_temporal_distribution,
    analyze_breed_and_age_with_normalization,
    analyze_gender_and_spayneuter,
};
use utils::load_data;

fn main() {
    let file_path = "/opt/app-root/src/project/Dog_Bites_Data.csv"; 
    let records = match load_data(file_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error loading data: {}", e);
            return;
        }
    };

    println!("Total records loaded after filtering: {}", records.len());

    analyze_zipcode_frequency(&records);

    analyze_temporal_distribution(&records);

    analyze_breed_and_age_with_normalization(&records);

    analyze_gender_and_spayneuter(&records);
}
