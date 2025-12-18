// ---------------------- TAURI GLOBAL API ----------------------
const { getCurrentWindow } = window.__TAURI__.window;
const { writeText } = window.__TAURI__.clipboardManager;
const { invoke } = window.__TAURI__.core;

const appWindow = getCurrentWindow();

// ---------------------- OCR TEXT ----------------------
const text = window.__OCR_TEXT ?? "(No OCR result)";
document.getElementById("ocr-text").innerText = text;

// ---------------------- BUTTONS ----------------------
const copyBtn = document.getElementById("copy");
const copyAllBtn = document.getElementById("copy-all");
const lookupBtn = document.getElementById("lookup");

// Hide selection-based buttons initially
copyBtn.style.display = "none";
lookupBtn.style.display = "none";

// ---------------------- COPY ACTIONS ----------------------
copyBtn.onclick = async () => {
    const selection = window.getSelection().toString();
    if (selection.trim().length > 0) {
        await writeText(selection);
    }
};

copyAllBtn.onclick = async () => {
    await writeText(text);
};

// ---------------------- LOOKUP ACTION ----------------------
lookupBtn.onclick = async () => {
    const selection = window.getSelection().toString().trim();
    if (!selection) return;

    try {
        await invoke("lookup_selected_text", { text: selection });
    } catch (err) {
        console.error("Lookup failed:", err);
        // Optional: surface a toast / inline error later
    }
};

// ---------------------- SELECTION VISIBILITY ----------------------

// helper: does the user have selected text?
function hasSelection() {
    const s = window.getSelection();
    return s && s.toString().trim().length > 0;
}

function updateSelectionButtons() {
    const visible = hasSelection();
    copyBtn.style.display = visible ? "inline-block" : "none";
    lookupBtn.style.display = visible ? "inline-block" : "none";
}

// Listen for changes in selection
document.addEventListener("selectionchange", updateSelectionButtons);
