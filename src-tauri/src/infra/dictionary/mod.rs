use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

/// Configure this once in your app
const BASE_URL: &str = "http://127.0.0.1:19633";

pub struct DictionaryAdapter {
    client: Client,
}

impl DictionaryAdapter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    // -------------------------------------------------
    // TOKENIZE (returns ARRAY)
    // -------------------------------------------------
    pub fn tokenize(&self, text: &str) -> reqwest::Result<TokenizeResponse> {
        let payload = TokenizeRequest {
            text: text.into(),
            scan_length: 15,
        };

        let raw: Vec<TokenizeItem> = self
            .client
            .post(format!("{BASE_URL}/tokenize"))
            .json(&payload)
            .send()?
            .json()?;

        // Convert tokenize structure into a flat list of tokens
        let mut tokens = Vec::new();

        if let Some(first_item) = raw.first() {
            for group in &first_item.content {
                // each group is Vec<TokenContent>
                // many groups contain 1 item; some contain 2 (compound)
                let mut combined = String::new();

                for c in group {
                    combined.push_str(&c.text);
                }

                tokens.push(TokenInfo { term: combined });
            }
        }

        Ok(TokenizeResponse { tokens })
    }

    // -------------------------------------------------
    // TERM ENTRIES (returns OBJECT)
    // -------------------------------------------------
    pub fn term_entries(&self, term: &str) -> reqwest::Result<TermEntriesResponse> {
        let payload = TermEntriesRequest { term: term.into() };

        let resp: TermEntriesResponse = self
            .client
            .post(format!("{BASE_URL}/termEntries"))
            .json(&payload)
            .send()?
            .json()?;

        Ok(resp)
    }

    // -------------------------------------------------
    // KANJI ENTRIES (returns ARRAY)
    // -------------------------------------------------
    pub fn kanji_entries(&self, ch: char) -> reqwest::Result<KanjiEntriesResponse> {
        let payload = KanjiEntriesRequest {
            character: ch.to_string(),
        };

        let entries: Vec<KanjiEntry> = self
            .client
            .post(format!("{BASE_URL}/kanjiEntries"))
            .json(&payload)
            .send()?
            .json()?;

        Ok(KanjiEntriesResponse { entries })
    }

    // -------------------------------------------------
    // PIPELINE
    // 1. tokenize(text)
    // 2. first token = T
    // 3. termEntries(T)
    // 4. kanjiEntries(first char of T)
    // -------------------------------------------------
    pub fn lookup(&self, text: &str) -> Result<LookupResult, LookupError> {
        println!("lookup(text = {text})");

        // Step 1: Tokenize
        let tokenize = self.tokenize(text).map_err(LookupError::Reqwest)?;
        let first = tokenize.tokens.first().ok_or(LookupError::NoTokens)?;
        let term = &first.term;

        // Step 2: term entries
        let term_entries = self.term_entries(term).map_err(LookupError::Reqwest)?;

        // Step 3: Kanji entries (first character of term)
        let first_char = term.chars().next().ok_or(LookupError::NoCharacters)?;
        let kanji_entries = self
            .kanji_entries(first_char)
            .map_err(LookupError::Reqwest)?;

        Ok(LookupResult {
            token: first.clone(),
            term_entries,
            kanji_entries,
        })
    }
}

//
// ------------------------------------------------------
// DATA MODELS (corrected to match real Yomitan JSON)
// ------------------------------------------------------
//

// ------------------- TOKENIZE -------------------------

#[derive(Serialize)]
struct TokenizeRequest {
    text: String,
    #[serde(rename = "scanLength")]
    scan_length: u32,
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

// Our flattened representation:
#[derive(Deserialize, Debug)]
pub struct TokenizeResponse {
    pub tokens: Vec<TokenInfo>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TokenInfo {
    pub term: String,
}

// ------------------- TERM ENTRIES ---------------------

#[derive(Serialize)]
struct TermEntriesRequest {
    term: String,
}

#[derive(Deserialize, Debug)]
pub struct TermEntriesResponse {
    #[serde(rename = "dictionaryEntries")]
    pub dictionary_entries: Vec<serde_json::Value>,

    #[serde(rename = "originalTextLength")]
    pub original_text_length: usize,
}

// ------------------- KANJI ENTRIES --------------------

#[derive(Serialize)]
struct KanjiEntriesRequest {
    character: String,
}

// Kanji returns ARRAY, so wrap it:
#[derive(Deserialize, Debug)]
pub struct KanjiEntriesResponse {
    pub entries: Vec<KanjiEntry>,
}

#[derive(Deserialize, Debug)]
pub struct KanjiEntry {
    pub r#type: String,
    pub character: String,
    pub onyomi: Vec<String>,
    pub kunyomi: Vec<String>,
    pub definitions: Vec<String>,
}

// ------------------- LOOKUP RESULT --------------------

#[derive(Debug)]
pub enum LookupError {
    Reqwest(reqwest::Error),
    NoTokens,
    NoCharacters,
}

#[derive(Debug)]
pub struct LookupResult {
    pub token: TokenInfo,
    pub term_entries: TermEntriesResponse,
    pub kanji_entries: KanjiEntriesResponse,
}
