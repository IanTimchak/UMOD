pub mod extractors;
pub mod yomitan_models;

use crate::shared::models::dictionary::*;
use extractors::*;
use yomitan_models::*;

/// Condense a Yomitan termEntries response into a Umod-ready model
pub fn condense_term_entries(query: &str, raw: &YomitanTermEntriesResponse) -> UmodTermEntries {
    let entries: Vec<UmodDictionaryEntry> = raw
        .dictionary_entries
        .iter()
        .filter_map(|dict_entry| {
            /* ---------------------------------------------
             Collect canonical POS from all entries
            --------------------------------------------- */
            let mut pos_tags = Vec::new();
            for def in &dict_entry.definitions {
                for entry in &def.entries {
                    pos_tags.extend(extract_pos_from_entry(entry));
                }
            }
            pos_tags.sort();
            pos_tags.dedup();

            /* ---------------------------------------------
             Headwords
            --------------------------------------------- */
            let headwords = dict_entry
                .headwords
                .iter()
                .map(|h| {
                    let mut classes = h.word_classes.clone();

                    // Inject canonical POS if none were supplied
                    if classes.is_empty() {
                        classes.extend(pos_tags.clone());
                        classes.sort();
                        classes.dedup();
                    }

                    UmodHeadword {
                        term: h.term.clone(),
                        reading: h.reading.clone(),
                        word_classes: classes,
                    }
                })
                .collect();

            /* ---------------------------------------------
             Definitions
            --------------------------------------------- */
            let definitions: Vec<UmodDefinition> = dict_entry
                .definitions
                .iter()
                .filter_map(|def| {
                    let (priority, tags) = extract_priority_and_tags(&def.tags);

                    let mut grammar = UmodGrammar::default();
                    let mut senses = Vec::new();
                    let mut variants = Vec::new();

                    for entry in &def.entries {
                        grammar.merge(extract_grammar_from_entry(entry));
                        senses = merge_senses(senses, extract_senses_from_entry(entry));
                        variants.extend(extract_variants_from_entry(entry));
                    }

                    variants.dedup();

                    if senses.is_empty() {
                        return None;
                    }

                    Some(UmodDefinition {
                        dictionary: def.dictionary.clone(),
                        priority,
                        tags,
                        grammar,
                        senses,
                        variants,
                    })
                })
                .collect();

            if definitions.is_empty() {
                return None;
            }

            /* ---------------------------------------------
             Frequencies
            --------------------------------------------- */
            let frequencies = dict_entry
                .frequencies
                .iter()
                .map(|f| UmodFrequency {
                    dictionary: f.dictionary.clone(),
                    display_value: f.display_value.clone(),
                    frequency: f.frequency,
                })
                .collect();

            Some(UmodDictionaryEntry {
                headwords,
                definitions,
                frequencies,
            })
        })
        .collect();

    UmodTermEntries {
        query: query.to_string(),
        original_text_length: raw.original_text_length,
        entries,
    }
}
