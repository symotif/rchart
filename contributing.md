# Contributing to rchart 🩺

First off, thank you for considering contributing to rchart! It’s people like you who will make modern, free healthcare tech a reality.

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

---

## 🗺️ How Can I Contribute?

### 1. 👩‍⚕️ Clinical Insights (Non-Code)
You don't need to be a programmer to help. We need doctors, nurses, and medical students to:
- **Review Workflows:** Is the "New Encounter" screen intuitive? 
- **Terminology:** Ensure our medical terms are accurate across different specialties.
- **Feature Requests:** Open an [Issue](https://github.com/your-username/rchart/issues) to describe a real-world problem you face in the clinic.

### 2. 🌍 Translations (i18n)
Healthcare is global. If you speak multiple languages, help us translate the UI strings located in `src/lib/i18n/`.

### 3. 💻 Code Contributions
We love Pull Requests! Whether it's fixing a typo in Svelte or optimizing an SQL query in Rust, here is how to get started:

#### Tech Stack Requirements:
- **Node.js** (v18+) & **Yarn**
- **Rust** (Stable toolchain)
- **Tauri CLI** (`cargo install tauri-cli`)

#### Development Workflow:
1. **Fork** the repository and create your branch from `dev`.
2. **Install dependencies:** `yarn install`.
3. **Run in dev mode:** `yarn tauri dev`.
4. **Style Guide:** - **Frontend:** Use TailwindCSS classes; follow Svelte 5 Rune patterns.
   - **Backend:** Follow standard Rust `clippy` suggestions.
5. **Commit Messages:** We use [Conventional Commits](https://www.conventionalcommits.org/) (e.g., `feat: add FHIR patient schema` or `fix: resolve sqlite lock on exit`).

---

## 🐛 Reporting Bugs
When reporting a bug, please include:
- Your Operating System (Windows/Mac/Linux).
- Clear steps to reproduce the error.
- Screenshots if the issue is visual.
- Any error logs from the Tauri terminal or Browser console.

---

## 🛠️ Security & Privacy
**IMPORTANT:** Because rchart handles Protected Health Information (PHI):
- **Never** include real patient data in bug reports or screenshots. Use dummy data (e.g., "John Doe, DOB 01/01/1970").
- If you find a security vulnerability, please **do not** open a public issue. Email Logan directly at [your-email@example.com].

---

## 📜 Pull Request Process
1. Ensure any install or build dependencies are removed before the end of the layer when doing a build.
2. Update the README.md with details of changes to the interface, this includes new environment variables, exposed ports, or useful locations.
3. You may merge the Pull Request once you have the sign-off of at least one maintainer.

---

**Thank you for helping us build the rchart!**