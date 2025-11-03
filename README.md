
# **UMOD – Universal Mouse-Over Dictionary**

**UMOD** is a cross-platform desktop utility that lets you capture a defined screen region, run OCR on the selection, and interact with the extracted text via hover-triggered dictionary lookups.

Most modern MOD (Mouse-Over Dictionary) tools operate under browser applications. These tools work exceptionally well in this context and allow learners to seemlessly interact with their target language by simply highlighting the word. This works great in the browser, however similar tools for desktop applications are few-and-far between.

This is the problem; when learners of languages encounter unfamiliar terms or characters in non-selectable contexts, such as images, games, or videos, they need to then look up those characters somehow in a dictionary. For some languages this is straightforward, but for others (especially those with complex orthography, such as mandarin or japanese) looking up an unfamiliar character is not a simple task. It requires using other tools (such as google translate or OCR) to interpret that text and determine its phonetic counterpart, to then search in a dictionary. This process creates unnecessary friction that disconnects the user from their target medium, interrupting immersion and making spontaneous learning harder. Each lookup becomes a small workflow—taking a picture, running OCR, copying text into a dictionary, and switching between windows—when it should instead be a single, seamless action. UMOD addresses this by combining these steps into one tool: capture, recognize, and look up, all within the same overlay. This approach lets learners stay engaged with the content they’re consuming while still having immediate access to accurate, context-aware dictionary results.

UMOD was built for language learners who want the convenience of instant word lookups anywhere on their screen, not just in a browser. It is inspired by and extends [YOMITAN](https://github.com/yomidevs/yomitan), which provides great hover-based dictionary tools for web content.

The current scope of this project is focused on the Japanese language (日本語). Contributions for extending the lexical analysis and conjugations of other languages are appreciated (when the project reaches that point).


---

## Table of Contents

* [Features](#features)
* [Quick Start](#quick-start)
* [Architecture](#architecture)
* [Usage Examples](#usage-examples)
* [Installation](#installation)
* [Deployment](#deployment)
* [Configuration & Settings](#configuration-settings)
* [Contributing](#contributing)
* [License](#license)
* [Contact](#contact)

---

## Features

* Capture screen region via keybind or tray.
* Transparent overlay preserving underlying applications.
* Drag & select area with visual bounds feedback.
* OCR converts region to editable text.
* Select/copy text via keyboard or mouse.
* Hover with modifier key to look up words in-place.
* Results panel anchored to selection/cursor; scrollable; supports audio if available.
* Scoped lookup panels: nested scopes, unique per scope, cascading destruction.
* Notes system: save OCR text or lookup results; attach memos and images; stored in JSON.
* Home overlay: search tab, notes tab, settings tab.
* Multiple dictionaries: enable/disable, set order/priority, concurrent requests with caching and timeouts.
* Headless startup: runs in background with tray menu and global keybinds; overlays only when invoked.

---

## Quick Start

1. Launch the application (via tray or keybind).
2. Invoke region capture.
3. Drag to select the area you want to capture.
4. OCR runs and extracts text into the overlay.
5. Hover over a word while holding the configured modifier key to see dictionary results.
6. Save notes or copy text as needed.

---

## Architecture

**Presentation Layer** — Manages UI overlays, panels, and interaction.
**Application Logic Layer** — Handles state, scope management, subsystem coordination.
**Infrastructure Layer** — Manages storage, configuration, keybind/tray integration.
**Cross-cutting Concerns** — Settings management, logging, session persistence.

**Subsystems**

* **OCR Subsystem**: Takes image input, returns text.
* **Dictionary Subsystem**: Normalises input, dispatches to enabled dictionaries, aggregates results.
* **Notes Subsystem**: Manages note creation, editing, persistence, viewer overlay.
* **Overlay Management**: Handles transparent overlay rendering, selection UX, panel management.

---

## Usage Examples

```text
# Example: Capture region
Press <keybind> or select “Capture Region” from tray → drag to select → release to confirm → overlay opens with text.

# Example: Lookup a word
Hover over “origin” while holding Shift → lookup panel appears → click audio icon to hear pronunciation.

# Example: Save a note
Within the overlay, click “Save note” → add tags/memo → note stored and accessible via Home overlay → Notes tab.
```

---

## Installation

*(To be added)*

---

## Deployment

*(To be added)*

---

## Configuration & Settings

* Set global keybinds for capture, hide/show overlay, home overlay.
* Configure dictionaries: enable/disable, prioritise, add local dictionaries.
* Persist settings across sessions; conflict detection for keybinds.
* Launch on system startup option.
* Reset to defaults, apply/cancel semantics.

---

## Contributing

If you’d like to contribute:

1. Fork the repository and create a branch for your feature or fix.
2. Ensure code style and tests pass.
3. Submit a pull request with description of what you did.
4. Follow issue templates and pull request guidelines [contributing.md](./contributing.md).

See [Issues](https://github.com/IanTimchak/UMOD/issues) for current tasks and suggestions.

---

## License

This project is licensed under the MIT License — see the [LICENSE](./LICENSE) file for details.

---

## Contact

Maintainer: Ian Timchak
GitHub: [IanChak](https://github.com/IanTimchak)
