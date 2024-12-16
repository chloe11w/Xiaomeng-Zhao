use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DogBiteRecord {
    #[serde(rename = "UniqueID")]
    pub unique_id: Option<String>,
    #[serde(rename = "DateOfBite")]
    pub date_of_bite: Option<String>,
    #[serde(rename = "Species")]
    pub species: Option<String>,
    #[serde(rename = "Breed")]
    pub breed: Option<String>,
    #[serde(rename = "Age")]
    pub age: Option<String>,
    #[serde(rename = "Gender")]
    pub gender: Option<String>,
    #[serde(rename = "SpayNeuter")]
    pub spay_neuter: Option<String>,
    #[serde(rename = "Borough")]
    pub borough: Option<String>,
    #[serde(rename = "ZipCode")]
    pub zip_code: Option<String>,
}