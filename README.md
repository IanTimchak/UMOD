
# **UMOD – Universal Mouse-Over Dictionary**

**UMOD** is a cross-platform desktop utility that lets you capture a defined screen region, run OCR on the selection, and interact with the extracted text via hover-triggered dictionary lookups.

Most modern MOD (Mouse-Over Dictionary) tools operate under browser applications. These tools work exceptionally well in this context and allow learners to seemlessly interact with their target language by simply highlighting the word. This works great in the browser, however similar tools for desktop applications are few-and-far between.

This is the problem; when learners of languages encounter unfamiliar terms or characters in non-selectable contexts, such as images, games, or videos, they need to then look up those characters somehow in a dictionary. For some languages this is straightforward, but for others (especially those with complex orthography, such as mandarin or japanese) looking up an unfamiliar character is not a simple task. It requires using other tools (such as google translate or OCR) to interpret that text and determine its phonetic counterpart, to then search in a dictionary. This process creates unnecessary friction that disconnects the user from their target medium, interrupting immersion and making spontaneous learning harder. Each lookup becomes a small workflow—taking a picture, running OCR, copying text into a dictionary, and switching between windows—when it should instead be a single, seamless action. UMOD addresses this by combining these steps into one tool: capture, recognize, and look up, all within the same overlay. This approach lets learners stay engaged with the content they’re consuming while still having immediate access to accurate, context-aware dictionary results.

UMOD was built for language learners who want the convenience of instant word lookups anywhere on their screen, not just in a browser. It is inspired by and extends [YOMITAN](https://github.com/yomidevs/yomitan), which provides great hover-based dictionary tools for web content.

The current scope of this project is focused on the Japanese language (日本語). Contributions for extending the lexical analysis and conjugations of other languages are appreciated (when the project reaches that point).


---

## Table of Contents

- [**UMOD – Universal Mouse-Over Dictionary**](#umod--universal-mouse-over-dictionary)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Quick Start](#quick-start)
  - [Usage Examples](#usage-examples)
  - [Prerequisites](#prerequisites)
    - [Operating System](#operating-system)
    - [Python](#python)
    - [Yomitan Browser Extension](#yomitan-browser-extension)
    - [Yomitan API](#yomitan-api)
    - [UMOD Python Dependencies](#umod-python-dependencies)
    - [Rust / Tauri Toolchain](#rust--tauri-toolchain)
  - [Installing and Running UMOD](#installing-and-running-umod)
  - [Configuration \& Settings](#configuration--settings)
  - [Contributing](#contributing)
  - [License](#license)
  - [Contact](#contact)

---

## Features

TBD

---

## Quick Start

TBD

---


## Usage Examples

```text
# Example: Capture region
Press <keybind> or select “Capture Region” from tray → drag to select → release to confirm → overlay opens with text.

# Example: Lookup a word
Hover over “origin” while holding Shift → lookup panel appears → click audio icon to hear pronunciation.

```

---

## Prerequisites

UMOD currently runs **only on Windows** and requires the following components to be installed and correctly configured.

### Operating System

* **Windows 10 or Windows 11 (64-bit)**

---

### Python

* **Python 3.14.1 (64-bit)**

  * Must be available on `PATH`
  * Verify:

    ```bash
    python --version
    ```

> Other Python versions are not supported.

---

### Yomitan Browser Extension

* Install the **Yomitan** browser extension
* Install dictionaries **inside Yomitan** by following the official documentation:

  * [https://yomitan.wiki/](https://yomitan.wiki/)
* Dictionaries must be fully installed and enabled in Yomitan before running UMOD

---

### Yomitan API

UMOD depends on a locally running Yomitan API service.

1. Clone and set up the Yomitan API repository:

   `https://github.com/yomidevs/yomitan-api`  
   
2. Follow **all setup instructions in that repository exactly**, including:

   * Python environment setup
   * Dependency installation
   * Running the API server
3. The API must be running and reachable before UMOD is launched.

---

### UMOD Python Dependencies

UMOD includes a Python OCR backend that must be initialized before use.

From the UMOD project directory containing `requirements.txt`:

```bash
pip install -r requirements.txt
```

Ensure this is executed using **Python 3.14.1**.

---

### Rust / Tauri Toolchain

UMOD is a Tauri application and requires the full Rust toolchain.

Install:

* **rustup**
* **rustc**
* **cargo**

Verify:

```bash
rustc --version
cargo --version
```

Tauri prerequisites must also be installed per the official documentation:

* [https://tauri.app/start/prerequisites/](https://tauri.app/start/prerequisites/)

---

## Installing and Running UMOD

1. Install the repository and navigate to its folder, then execute `> cd .\src-tauri`.

1. Ensure the following are all running and configured:

   * Yomitan extension installed with dictionaries
   * Yomitan API server running
   * Python 3.14.1 available

2. Build and run UMOD via Cargo in the UMOD\src-tauri root folder:  
   First, install the required rust/tauri dependencies:  
   ```bash
   cargo install  
   npm install  
   ```
   
   ```bash
   cargo run
   ```

   or for release:

   ```bash
   cargo build --release
   ```

UMOD runs primarily via **tray interaction and global hotkeys**. No main window is shown by default.

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

This project is licensed under the GPL License — see the [LICENSE](./LICENSE) file for details.

---

## Contact

Maintainer: Ian Timchak  
GitHub: [IanChak](https://github.com/IanTimchak)
