use reqwest::blocking::Client;
use serde::Serialize;
mod adapter_models;
mod yomitan;

use crate::shared::models::dictionary::UmodTermEntries;
use adapter_models::*;
use yomitan::{condense_term_entries, yomitan_models::YomitanTermEntriesResponse};

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
    // TOKENIZE
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

        let mut tokens = Vec::new();

        if let Some(first_item) = raw.first() {
            for group in &first_item.content {
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
    // TERM ENTRIES (RAW)
    // -------------------------------------------------
    pub fn term_entries_raw(&self, term: &str) -> reqwest::Result<YomitanTermEntriesResponse> {
        let payload = TermEntriesRequest { term: term.into() };

        let resp: YomitanTermEntriesResponse = self
            .client
            .post(format!("{BASE_URL}/termEntries"))
            .json(&payload)
            .send()?
            .json()?;

        Ok(resp)
    }

    // -------------------------------------------------
    // TERM ENTRIES (Umod)
    // -------------------------------------------------
    pub fn term_entries_umod(&self, term: &str) -> reqwest::Result<UmodTermEntries> {
        let raw = self.term_entries_raw(term)?;
        Ok(condense_term_entries(term, &raw))
    }

    // -------------------------------------------------
    // KANJI ENTRIES
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
    // -------------------------------------------------
    pub fn lookup(&self, text: &str) -> Result<LookupResult, LookupError> {
        let tokenize = self.tokenize(text).map_err(LookupError::Reqwest)?;
        let first = tokenize.tokens.first().ok_or(LookupError::NoTokens)?;
        let term = &first.term;

        let term_entries = self.term_entries_umod(term).map_err(LookupError::Reqwest)?;

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

/* ---------------- Errors / Results ---------------- */

#[derive(Debug)]
pub enum LookupError {
    Reqwest(reqwest::Error),
    NoTokens,
    NoCharacters,
}

#[derive(Debug, Serialize)]
pub struct LookupResult {
    pub token: TokenInfo,
    pub term_entries: UmodTermEntries,
    pub kanji_entries: KanjiEntriesResponse,
}

//Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create adapter once
    fn adapter() -> DictionaryAdapter {
        DictionaryAdapter::new()
    }

    #[test]
    fn tokenize_basic() {
        let adapter = adapter();
        let result = adapter.tokenize("分かる").expect("tokenize failed");

        println!("TOKENS: {:#?}", result);

        assert!(!result.tokens.is_empty(), "tokenize returned no tokens");
    }

    #[test]
    fn term_entries_raw_smoke() {
        let adapter = adapter();
        let raw = adapter
            .term_entries_raw("分かる")
            .expect("term_entries_raw failed");

        println!(
            "RAW TERM ENTRIES:\n{} entries, originalTextLength={}",
            raw.dictionary_entries.len(),
            raw.original_text_length
        );

        assert!(
            !raw.dictionary_entries.is_empty(),
            "no dictionaryEntries returned"
        );
    }

    #[test]
    fn term_entries_umod_shape() {
        let adapter = adapter();
        let umod = adapter
            .term_entries_umod("分かる")
            .expect("term_entries_umod failed");

        println!("UMOD TERM ENTRIES:\n{:#?}", umod);

        assert!(!umod.entries.is_empty(), "UMOD entries is empty");

        let first_entry = &umod.entries[0];
        assert!(
            !first_entry.headwords.is_empty(),
            "first entry has no headwords"
        );

        let first_def = &first_entry.definitions[0];
        assert!(!first_def.senses.is_empty(), "definition has no senses");

        let first_sense = &first_def.senses[0];
        assert!(!first_sense.glosses.is_empty(), "sense has no glosses");
    }

    #[test]
    fn kanji_entries_basic() {
        let adapter = adapter();
        let result = adapter.kanji_entries('分').expect("kanji_entries failed");

        println!("KANJI ENTRIES:\n{:#?}", result);

        assert!(!result.entries.is_empty(), "kanji entries empty");
    }

    #[test]
    fn full_lookup_pipeline() {
        let adapter = adapter();
        let lookup = adapter.lookup("分かる").expect("lookup pipeline failed");

        println!("LOOKUP RESULT:\n{:#?}", lookup);

        assert_eq!(lookup.token.term, "分かる", "token term mismatch");

        assert!(
            !lookup.term_entries.entries.is_empty(),
            "lookup returned no term entries"
        );

        assert!(
            !lookup.kanji_entries.entries.is_empty(),
            "lookup returned no kanji entries"
        );
    }
}
