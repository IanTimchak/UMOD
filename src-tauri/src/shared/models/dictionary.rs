/* infra/dictionary/yomitan/umod_models.rs
Contains the canonical data-object representation of the term entries formatted for UMOD representation.
This is project focused code which is allowed to populate upwards. Changes to the dictionary adapter should maintain
this format.
*/
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UmodTermEntries {
    pub query: String,
    pub original_text_length: usize,
    pub entries: Vec<UmodDictionaryEntry>,
}

#[derive(Debug, Serialize)]
pub struct UmodDictionaryEntry {
    pub headwords: Vec<UmodHeadword>,
    pub definitions: Vec<UmodDefinition>,
    pub frequencies: Vec<UmodFrequency>,
}

#[derive(Debug, Serialize)]
pub struct UmodHeadword {
    pub term: String,
    pub reading: Option<String>,
    pub word_classes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct UmodDefinition {
    pub dictionary: String,
    pub priority: bool,
    pub tags: Vec<String>,
    pub grammar: UmodGrammar,
    pub senses: Vec<UmodSense>,
    pub variants: Vec<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct UmodGrammar {
    pub transitivity: Option<String>,
    pub kana_only: bool,
}

impl UmodGrammar {
    pub fn merge(&mut self, other: UmodGrammar) {
        if self.transitivity.is_none() {
            self.transitivity = other.transitivity;
        }
        self.kana_only |= other.kana_only;
    }
}

#[derive(Debug, Serialize)]
pub struct UmodSense {
    pub number: usize,
    pub glosses: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct UmodFrequency {
    pub dictionary: String,
    pub display_value: Option<String>,
    pub frequency: Option<u64>,
}
