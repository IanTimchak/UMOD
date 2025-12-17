// dictionary/lookup.js

const tauri = window.__TAURI__;
const getCurrentWindow = tauri?.window?.getCurrentWindow;

function el(tag, className, text) {
  const n = document.createElement(tag);
  if (className) n.className = className;
  if (text !== undefined && text !== null) n.textContent = String(text);
  return n;
}

function uniq(arr) {
  return [...new Set((arr || []).filter(Boolean))];
}

function renderHeader(root, lookup) {
  const header = document.getElementById("header");
  header.innerHTML = "";

  const firstEntry = lookup?.term_entries?.entries?.[0];
  const firstHeadword = firstEntry?.headwords?.[0];

  const termBlock = el("div", "term-block");
  termBlock.appendChild(
    el("span", "term", firstHeadword?.term ?? lookup?.term_entries?.query ?? "")
  );
  if (firstHeadword?.reading)
    termBlock.appendChild(el("span", "reading", firstHeadword.reading));

  const badges = el("div", "badges");
  // optional: show token to help debugging / selection correctness
  badges.appendChild(
    el("span", "badge source", `token: ${lookup?.token?.term ?? ""}`)
  );

  header.appendChild(termBlock);
  header.appendChild(badges);
}

function renderDefinition(def) {
  const wrap = el("article", "definition");

  // title row: dictionary + badges
  const top = el("div", "definition-top");
  top.appendChild(el("div", "dict-name", def.dictionary));

  const b = el("div", "badges");
  if (def.priority) b.appendChild(el("span", "badge priority", "★"));
  for (const t of def.tags || []) b.appendChild(el("span", "badge", t));
  top.appendChild(b);
  wrap.appendChild(top);

  // grammar tags
  const grammarRow = el("div", "grammar");
  const g = def.grammar || {};
  if (g.transitivity) grammarRow.appendChild(el("span", "tag", g.transitivity));
  if (g.kana_only) grammarRow.appendChild(el("span", "tag kana", "kana"));
  // headword word-classes (v5, n, etc.) are added by caller (so we can show them once)
  wrap.appendChild(grammarRow);

  // senses
  const meanings = el("div", "meanings");

  const ol = el("ol", "sense-list");
  for (const s of def.senses) {
    const li = el("li", "sense");
    li.dataset.sense = String(s.number);

    // If this is “plain text fallback” (one giant string), render as pre for readability
    const isPlainFallback =
      (s.glosses || []).length === 1 && (s.glosses[0] || "").includes("\n");

    if (isPlainFallback) {
      li.appendChild(el("pre", "plain", s.glosses[0]));
    } else {
      const ul = el("ul", "gloss-list");
      for (const gloss of s.glosses || []) ul.appendChild(el("li", "", gloss));
      li.appendChild(ul);
    }

    ol.appendChild(li);
  }
  meanings.appendChild(ol);

  wrap.appendChild(meanings);

  // variants
  const variants = uniq(def.variants);
  if (variants.length) {
    const sec = el("div", "variants");
    sec.appendChild(el("span", "variants-label", "Forms:"));
    const ul = el("ul", "variants-list");
    for (const v of variants) ul.appendChild(el("li", "", v));
    sec.appendChild(ul);
    wrap.appendChild(sec);
  }

  return wrap;
}

function renderEntry(entry) {
  const section = el("section", "entry");

  // headwords row
  const hw = el("div", "headwords");
  for (const h of entry.headwords || []) {
    const chip = el("div", "headword-chip");
    chip.appendChild(el("span", "hw-term", h.term));
    if (h.reading) chip.appendChild(el("span", "hw-reading", h.reading));
    hw.appendChild(chip);
  }
  section.appendChild(hw);

  // collect word classes across headwords (v5 etc.)
  const classes = uniq(
    (entry.headwords || []).flatMap((h) => h.word_classes || [])
  );
  if (classes.length) {
    const row = el("div", "grammar");
    for (const c of classes) row.appendChild(el("span", "tag", c));
    section.appendChild(row);
  }

  // definitions
  for (const def of entry.definitions || []) {
    const d = renderDefinition(def);
    // inject headword classes tags at definition level too if you want:
    // (I recommend keeping classes at entry level to avoid repetition.)
    section.appendChild(d);
  }

  return section;
}

function renderAll(lookup) {
  renderHeader(document.body, lookup);

  const entriesRoot = document.getElementById("entries");
  entriesRoot.innerHTML = "";

  for (const entry of lookup?.term_entries?.entries || []) {
    entriesRoot.appendChild(renderEntry(entry));
  }
}

// boot
(function main() {
  const lookup = window.__LOOKUP_RESULT;
  if (!lookup) {
    document.getElementById("entries").textContent =
      "No lookup payload received.";
    return;
  }

  renderAll(lookup);

  // optional: esc to close (Tauri 2)
  window.addEventListener("keydown", async (e) => {
    if (e.key === "Escape" && getCurrentWindow) {
      await getCurrentWindow().close();
    }
  });
})();
