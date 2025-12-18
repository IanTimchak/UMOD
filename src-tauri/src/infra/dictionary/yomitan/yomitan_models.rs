use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct YomitanTermEntriesResponse {
    #[serde(rename = "dictionaryEntries")]
    pub dictionary_entries: Vec<YomitanDictionaryEntry>,

    #[serde(rename = "originalTextLength")]
    pub original_text_length: usize,
}

#[derive(Debug, Deserialize)]
pub struct YomitanDictionaryEntry {
    pub headwords: Vec<YomitanHeadword>,
    pub definitions: Vec<YomitanDefinition>,

    #[serde(default)]
    pub frequencies: Vec<YomitanFrequency>,
}

#[derive(Debug, Deserialize)]
pub struct YomitanHeadword {
    pub term: String,

    #[serde(default)]
    pub reading: Option<String>,

    #[serde(default, rename = "wordClasses")]
    pub word_classes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct YomitanDefinition {
    pub dictionary: String,

    #[serde(default)]
    pub tags: Vec<YomitanTag>,

    #[serde(default)]
    pub entries: Vec<YomitanEntry>,
}

#[derive(Debug, Deserialize)]
pub struct YomitanTag {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum YomitanEntry {
    Structured {
        #[serde(rename = "type")]
        entry_type: String,
        content: Value,
    },
    PlainText(String),
}

#[derive(Debug, Deserialize)]
pub struct YomitanFrequency {
    pub dictionary: String,

    #[serde(rename = "displayValue")]
    pub display_value: Option<String>,

    pub frequency: Option<u64>,
}
