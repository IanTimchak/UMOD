use serde_json::Value;
use std::collections::{BTreeMap, HashSet};

use super::yomitan_models::{YomitanEntry, YomitanTag};
use crate::shared::models::dictionary::*;

/* =====================================================
Generic JSON walker
===================================================== */

fn walk_objects(v: &Value, f: &mut dyn FnMut(&serde_json::Map<String, Value>)) {
    match v {
        Value::Object(map) => {
            f(map);
            for v in map.values() {
                walk_objects(v, f);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                walk_objects(v, f);
            }
        }
        _ => {}
    }
}

/* =====================================================
Tags / priority
===================================================== */

pub fn extract_priority_and_tags(tags: &[YomitanTag]) -> (bool, Vec<String>) {
    let mut priority = false;
    let mut out = Vec::new();

    for t in tags {
        if t.name.contains("priority") {
            priority = true;
        }
        if t.name.trim() == "★" || t.name.contains("priority") {
            out.push(t.name.clone());
        }
    }

    (priority, out)
}

/* =====================================================
POS / word classes (canonicalized)
===================================================== */

pub fn extract_pos_from_entry(entry: &YomitanEntry) -> Vec<String> {
    let YomitanEntry::Structured { content, .. } = entry else {
        return Vec::new();
    };

    let mut out = Vec::new();

    walk_objects(content, &mut |obj| {
        if obj.get("tag").and_then(Value::as_str) != Some("span") {
            return;
        }

        let Some(code) = obj
            .get("data")
            .and_then(|d| d.get("code"))
            .and_then(Value::as_str)
        else {
            return;
        };

        // Grammar-only flags handled elsewhere
        if matches!(code, "vi" | "vt" | "uk") {
            return;
        }

        if let Some(norm) = normalize_pos(code) {
            out.push(norm.to_string());
        }
    });

    out.sort();
    out.dedup();
    out
}

fn normalize_pos(code: &str) -> Option<&'static str> {
    match code {
        // nouns
        "n" | "pn" => Some("noun"),

        // verbs
        "v1" => Some("ichidan"),
        "v5" => Some("godan"),
        "vs" => Some("suru-verb"),
        "vk" => Some("kuru-verb"),

        // adjectives
        "adj-i" => Some("i-adjective"),
        "adj-no" | "adj-na" => Some("na-adjective"),
        "adj-pn" => Some("prenominal-adjective"),

        // modifiers / function words
        "adv" => Some("adverb"),
        "prt" => Some("particle"),
        "conj" => Some("conjunction"),
        "int" => Some("interjection"),

        // affixes
        "pref" => Some("prefix"),
        "suf" => Some("suffix"),

        // misc
        "exp" => Some("expression"),
        "num" => Some("numeric"),
        "ctr" => Some("counter"),

        _ => None,
    }
}

/* =====================================================
Grammar (structured entries only)
===================================================== */

pub fn extract_grammar_from_entry(entry: &YomitanEntry) -> UmodGrammar {
    let mut g = UmodGrammar::default();

    let YomitanEntry::Structured { content, .. } = entry else {
        return g;
    };

    walk_objects(content, &mut |obj| {
        if obj.get("tag").and_then(Value::as_str) != Some("span") {
            return;
        }

        let code = obj
            .get("data")
            .and_then(|d| d.get("code"))
            .and_then(Value::as_str);

        match code {
            Some("vi") => g.transitivity = Some("intransitive".into()),
            Some("vt") => g.transitivity = Some("transitive".into()),
            Some("uk") => g.kana_only = true,
            _ => {}
        }
    });

    g
}

/* =====================================================
Senses
===================================================== */

pub fn extract_senses_from_entry(entry: &YomitanEntry) -> Vec<UmodSense> {
    match entry {
        YomitanEntry::Structured { content, .. } => {
            let numbered = extract_structured_senses(content);
            if !numbered.is_empty() {
                return numbered;
            }

            let glosses = extract_glossary_without_numbers(content);
            if !glosses.is_empty() {
                return vec![UmodSense {
                    number: 1,
                    glosses,
                }];
            }

            Vec::new()
        }

        YomitanEntry::PlainText(text) => vec![UmodSense {
            number: 1,
            glosses: vec![text.clone()],
        }],
    }
}

fn extract_structured_senses(content: &Value) -> Vec<UmodSense> {
    let mut map: BTreeMap<usize, Vec<String>> = BTreeMap::new();

    walk_objects(content, &mut |obj| {
        let Some(num) = obj
            .get("data")
            .and_then(|d| d.get("sense-number"))
            .and_then(Value::as_str)
            .and_then(|s| s.parse::<usize>().ok())
        else {
            return;
        };

        walk_objects(&Value::Object(obj.clone()), &mut |inner| {
            let is_glossary = inner
                .get("data")
                .and_then(|d| d.get("content"))
                .and_then(Value::as_str)
                == Some("glossary");

            if !is_glossary {
                return;
            }

            walk_objects(&Value::Object(inner.clone()), &mut |li| {
                if li.get("tag").and_then(Value::as_str) == Some("li") {
                    if let Some(text) = li.get("content").and_then(Value::as_str) {
                        map.entry(num).or_default().push(text.to_string());
                    }
                }
            });
        });
    });

    map.into_iter()
        .map(|(number, mut glosses)| {
            glosses.dedup();
            UmodSense { number, glosses }
        })
        .collect()
}

pub fn merge_senses(a: Vec<UmodSense>, b: Vec<UmodSense>) -> Vec<UmodSense> {
    let mut map: BTreeMap<usize, Vec<String>> = BTreeMap::new();

    for s in a.into_iter().chain(b) {
        map.entry(s.number).or_default().extend(s.glosses);
    }

    map.into_iter()
        .map(|(number, mut glosses)| {
            let mut seen = HashSet::new();
            glosses.retain(|g| seen.insert(g.clone()));
            UmodSense { number, glosses }
        })
        .collect()
}

/* =====================================================
Variants (structured entries only)
===================================================== */

pub fn extract_variants_from_entry(entry: &YomitanEntry) -> Vec<String> {
    let YomitanEntry::Structured { content, .. } = entry else {
        return Vec::new();
    };

    let mut out = Vec::new();

    walk_objects(content, &mut |obj| {
        let is_forms = obj
            .get("data")
            .and_then(|d| d.get("content"))
            .and_then(Value::as_str)
            == Some("forms");

        if !is_forms {
            return;
        }

        walk_objects(&Value::Object(obj.clone()), &mut |li| {
            if li.get("tag").and_then(Value::as_str) == Some("li") {
                if let Some(text) = li.get("content").and_then(Value::as_str) {
                    out.push(text.to_string());
                }
            }
        });
    });

    out
}

/* =====================================================
Glossary-only fallback
===================================================== */

fn extract_glossary_without_numbers(content: &Value) -> Vec<String> {
    let mut glosses = Vec::new();

    walk_objects(content, &mut |obj| {
        let is_glossary = obj
            .get("data")
            .and_then(|d| d.get("content"))
            .and_then(Value::as_str)
            == Some("glossary");

        if !is_glossary {
            return;
        }

        walk_objects(&Value::Object(obj.clone()), &mut |li| {
            if li.get("tag").and_then(Value::as_str) == Some("li") {
                if let Some(text) = li.get("content").and_then(Value::as_str) {
                    let t = text.trim();
                    if !t.is_empty() {
                        glosses.push(t.to_string());
                    }
                }
            }
        });
    });

    glosses
}
