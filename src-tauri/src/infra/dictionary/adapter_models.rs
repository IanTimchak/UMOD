use serde::{Deserialize, Serialize};

//
// ------------------- TOKENIZE -------------------------
//

#[derive(Serialize)]
pub struct TokenizeRequest {
    pub text: String,
    #[serde(rename = "scanLength")]
    pub scan_length: u32,
}

// Raw shape from Yomitan (ARRAY)
#[derive(Deserialize, Debug)]
pub struct TokenizeItem {
    pub id: String,
    pub source: String,
    pub dictionary: Option<String>,
    pub content: Vec<Vec<TokenContent>>,
}

#[derive(Deserialize, Debug)]
pub struct TokenContent {
    pub text: String,
    pub reading: String,
}

// Flattened representation used by adapter
#[derive(Deserialize, Debug)]
pub struct TokenizeResponse {
    pub tokens: Vec<TokenInfo>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TokenInfo {
    pub term: String,
}

//
// ------------------- TERM ENTRIES ---------------------
//

#[derive(Serialize)]
pub struct TermEntriesRequest {
    pub term: String,
}

//
// ------------------- KANJI ENTRIES --------------------
//

#[derive(Serialize)]
pub struct KanjiEntriesRequest {
    pub character: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KanjiEntriesResponse {
    pub entries: Vec<KanjiEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KanjiEntry {
    pub r#type: String,
    pub character: String,
    pub onyomi: Vec<String>,
    pub kunyomi: Vec<String>,
    pub definitions: Vec<String>,
}
