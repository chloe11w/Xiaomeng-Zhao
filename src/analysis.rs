use crate::models::DogBiteRecord;
use crate::utils::{extract_month_and_season, parse_age};
use std::collections::HashMap;

pub fn normalize_breed(breed: &str) -> String {
    let breed_keywords = vec![
        ("Pit Bull", "Pit Bull"),
        ("American Pit Bull Mix", "Pit Bull"),
        ("American Pit Bull Terrier", "Pit Bull"),
        ("Mixed/Other", "Mixed"),
        ("MIXED BREED", "Mixed"),
        ("MIXED", "Mixed"),
    ];

    let breed_upper = breed.to_uppercase();

    for (keyword, normalized) in breed_keywords.iter() {
        if breed_upper.contains(&keyword.to_uppercase()) {
            return normalized.to_string();
        }
    }

    if let Some(first_word) = breed.split_whitespace().next() {
        return first_word.to_string();
    }

    "Unknown".to_string()
}

pub fn analyze_breed_and_age_with_normalization(records: &[DogBiteRecord]) {
    let mut breed_count: HashMap<String, usize> = HashMap::new();
    let mut age_sum: HashMap<String, (f32, usize)> = HashMap::new();

    for r in records {
        if let Some(breed) = &r.breed {
            let normalized_breed = normalize_breed(breed);

            if normalized_breed.to_uppercase() == "UNKNOWN" {
                continue;
            }

            *breed_count.entry(normalized_breed.clone()).or_insert(0) += 1;

            if let Some(age) = parse_age(&r.age) {
                let entry = age_sum.entry(normalized_breed.clone()).or_insert((0.0, 0));
                entry.0 += age;
                entry.1 += 1;
            }
        }
    }

    println!("--- Top 10 Dog Breeds Involved in Bites (Normalized) ---");
    let mut sorted_breeds: Vec<_> = breed_count.into_iter().collect();
    sorted_breeds.sort_by_key(|&(_, count)| std::cmp::Reverse(count));
    for (breed, count) in sorted_breeds.iter().take(10) {
        println!("  {}: {}", breed, count);
    }

    println!("--- Average Age of Top 10 Breeds (Normalized) ---");
    for (breed, _) in sorted_breeds.iter().take(10) {
        if let Some((age_sum, count)) = age_sum.get(breed) {
            let avg_age = age_sum / (*count as f32);
            println!("  {}: {:.2} years", breed, avg_age);
        } else {
            println!("  {}: No age data available", breed);
        }
    }
}

pub fn analyze_zipcode_frequency(records: &[DogBiteRecord]) {
    let mut zip_count: HashMap<String, usize> = HashMap::new();

    for r in records {
        if let Some(zc) = &r.zip_code {
            *zip_count.entry(zc.clone()).or_insert(0) += 1;
        }
    }

    println!("--- ZipCode Frequency Analysis ---");
    println!("Total distinct zip codes: {}", zip_count.len());

    let mut zip_vec: Vec<_> = zip_count.into_iter().collect();
    zip_vec.sort_by_key(|&(_, c)| std::cmp::Reverse(c));

    println!("Top 10 ZipCodes with highest dog bite frequencies:");
    for (zc, count) in zip_vec.iter().take(10) {
        println!("  {}: {}", zc, count);
    }
}

pub fn analyze_temporal_distribution(records: &[DogBiteRecord]) {
    let mut month_count: HashMap<u32, usize> = HashMap::new();
    let mut season_count: HashMap<u32, usize> = HashMap::new();

    for record in records {
        if let Some((month, season)) = extract_month_and_season(&record.date_of_bite) {
            *month_count.entry(month).or_insert(0) += 1;
            *season_count.entry(season).or_insert(0) += 1;
        }
    }

    println!("--- Monthly Distribution ---");
    let mut sorted_months: Vec<_> = month_count.into_iter().collect();
    sorted_months.sort_by_key(|&(month, _)| month);
    for (month, count) in sorted_months {
        println!("  Month {}: {}", month, count);
    }

    println!("--- Seasonal Distribution ---");
    let seasons = ["Winter", "Spring", "Summer", "Autumn"];
    let mut sorted_seasons: Vec<_> = season_count.into_iter().collect();
    sorted_seasons.sort_by_key(|&(season, _)| season);
    for (season, count) in sorted_seasons {
        if season > 0 && (season as usize) <= seasons.len() {
            println!("  {}: {}", seasons[(season - 1) as usize], count);
        }
    }
}

pub fn analyze_gender_and_spayneuter(records: &[DogBiteRecord]) {
    let mut gender_count: HashMap<String, usize> = HashMap::new();
    let mut spay_neuter_count: HashMap<String, usize> = HashMap::new();
    let mut combined_stats: HashMap<(String, String), usize> = HashMap::new();

    for r in records {
        if let Some(gender) = &r.gender {
            *gender_count.entry(gender.clone()).or_insert(0) += 1;

            if let Some(spay_neuter) = &r.spay_neuter {
                *spay_neuter_count.entry(spay_neuter.clone()).or_insert(0) += 1;
                let key = (gender.clone(), spay_neuter.clone());
                *combined_stats.entry(key).or_insert(0) += 1;
            }
        }
    }

    println!("--- Dog Bite Incidents by Gender ---");
    for (gender, count) in gender_count.iter() {
        println!("  {}: {}", gender, count);
    }

    println!("--- Dog Bite Incidents by Spay/Neuter Status ---");
    for (status, count) in spay_neuter_count.iter() {
        println!("  {}: {}", status, count);
    }

    println!("--- Dog Bite Incidents by Gender and Spay/Neuter Status ---");
    for ((gender, status), count) in combined_stats.iter() {
        println!("  Gender: {}, Spay/Neuter: {} -> {}", gender, status, count);
    }
}
