use serde::{Deserialize, Serialize};
use serde_json::Value;

// This refers to the json_resume schema of gitconnected

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Resume {
    pub basics: Basics,
    pub work: Vec<Work>,
    pub volunteer: Vec<Volunteer>,
    pub education: Vec<Education>,
    pub awards: Vec<Award>,
    pub certificates: Vec<Certificate>,
    pub publications: Vec<Publication>,
    pub skills: Vec<Skill>,
    pub languages: Vec<Language>,
    pub interests: Vec<Interest>,
    pub references: Vec<Reference>,
    pub projects: Vec<Project>,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Basics {
    pub name: String,
    pub label: String,
    pub image: String,
    pub email: String,
    pub phone: String,
    pub url: String,
    pub summary: String,
    pub location_as_string: String,
    pub profiles: Vec<Profile>,
    pub region: String,
    pub username: String,
    pub headline: String,
    pub years_of_experience: u8,
    // I've never seen this populated so value so I don't get errors
    pub blog: Option<Value>,
    pub karma: i64,
    pub id: String,
    pub followers: i64,
    pub following: i64,
    pub picture: String,
    pub website: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Profile {
    pub network: String,
    pub username: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Work {
    pub name: String,
    pub position: String,
    pub url: String,
    pub start_date: String,
    pub end_date: String,
    pub summary: String,
    pub highlights: Vec<String>,
    pub location: String,
    pub description: String,
    pub is_current_role: bool,
    pub start: ShortDate,
    pub end: ShortDate,
    pub company: String,
    pub website: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Volunteer {
    pub organization: String,
    pub position: String,
    pub url: String,
    pub start_date: String,
    pub end_date: String,
    pub summary: String,
    pub highlights: Vec<String>,
    pub location: String,
    pub start: ShortDate,
    pub end: ShortDate,
    pub website: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Education {
    pub institution: String,
    pub url: String,
    pub area: String,
    pub study_type: String,
    pub start_date: String,
    pub end_date: String,
    pub score: String,
    pub courses: Vec<String>,
    pub description: String,
    pub activities: String,
    pub start: ShortDate,
    pub end: ShortDate,
    pub website: String,
    pub gpa: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Award {
    pub title: String,
    pub date: String,
    pub awarder: String,
    pub summary: String,
    pub full_date: ShortDate,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Certificate {
    pub name: String,
    pub date: String,
    pub issuer: String,
    pub url: String,
    pub summary: String,
    pub full_date: Date,
    pub website: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Publication {
    pub name: String,
    pub publisher: String,
    pub release_date: String,
    pub url: String,
    pub summary: String,
    pub full_release_date: Date,
    pub website: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Skill {
    pub name: String,
    pub level: String,
    pub keywords: Vec<String>,
    pub rating: i64,
    #[serde(rename = "yearsOfExperience")]
    pub years_of_experience: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Language {
    pub language: String,
    pub fluency: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Interest {
    pub name: String,
    pub keywords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Reference {
    pub name: String,
    pub reference: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub name: String,
    pub description: String,
    pub highlights: Vec<String>,
    pub keywords: Vec<String>,
    pub start_date: String,
    pub end_date: String,
    pub url: String,
    // when unset shows up as an array, but when populated is a string
    pub roles: Value,
    pub entity: String,
    #[serde(rename = "type")]
    pub project_type: String,
    pub languages: Vec<String>,
    pub libraries: Vec<String>,
    pub display_name: String,
    pub website: String,
    pub summary: String,
    pub primary_language: String,
    pub github_url: String,
    pub repository_url: String,
    pub start: Date,
    pub end: Date,
    pub images: Vec<Image>,
    pub videos: Vec<Video>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ShortDate {
    pub year: Option<i64>,
    pub month: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Date {
    pub year: Option<i64>,
    pub month: Option<i64>,
    pub day: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Video {
    pub url: String,
    pub source: String,
    #[serde(rename = "sourceId")]
    pub source_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ImageSource {
    pub url: String,
    pub size: i64,
    pub width: i64,
    pub height: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Resolutions {
    pub micro: ImageSource,
    pub thumbnail: ImageSource,
    pub mobile: ImageSource,
    pub desktop: ImageSource,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Image {
    pub resolutions: Resolutions,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Meta {
    pub note: String,
    pub canonical: String,
    pub version: String,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
}
