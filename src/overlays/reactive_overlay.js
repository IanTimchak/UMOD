// ---------------------- TAURI GLOBAL API ----------------------
const { getCurrentWindow  } = window.__TAURI__.window;
const appWindow = getCurrentWindow();
const { writeText } = window.__TAURI__.clipboardManager;

// ---------------------- OCR TEXT ----------------------
const text = window.__OCR_TEXT ?? "(No OCR result)";
document.getElementById("ocr-text").innerText = text;

// Hide copy button initially
const copyBtn = document.getElementById("copy");
const copyAllBtn = document.getElementById("copy-all");
copyBtn.style.display = "none";

// -------- WINDOW CONTROLS --------
document.getElementById("close").onclick = () => appWindow.close();
document.getElementById("minimize").onclick = () => appWindow.minimize();

// -------- COPY ACTIONS --------
copyBtn.onclick = async () => {
    const selection = window.getSelection().toString();
    if (selection.trim().length > 0) {
        await writeText(selection);
    }
};

copyAllBtn.onclick = async () => writeText(text);

// -------- SHOW/HIDE COPY BUTTON BASED ON SELECTION --------

// helper: does the user have selected text?
function hasSelection() {
    const s = window.getSelection();
    return s && s.toString().trim().length > 0;
}

function updateCopyVisibility() {
    copyBtn.style.display = hasSelection() ? "inline-block" : "none";
}

// Listen for changes in selection
document.addEventListener("selectionchange", updateCopyVisibility);