<h1 align="center">🩺 rchart</h1>


<div align="center">
    <strong>A modern, free, extensible EHR for the world</strong>
    <br>
    Built by doctors, for doctors.
</div>

<br />

<div align="center">
  <img src="https://img.shields.io/badge/Version-0.1.0--alpha-blue?style=for-the-badge" alt="Version" />
  <img src="https://img.shields.io/badge/License-AGPL--3.0-orange?style=for-the-badge" alt="License" />
  <img src="https://img.shields.io/badge/Built%20With-Svelte%20%26%20Rust-61DAFB?style=for-the-badge&logo=svelte" alt="Tech Stack" />
  <br />
  <a href="code_of_conduct.md">
    <img src="https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg?style=flat-square" alt="Contributor Covenant" />
  </a>
  <img src="https://img.shields.io/github/stars/your-username/rchart?style=flat-square&color=yellow" alt="Stars" />
</div>

<div align="center">
    <p><strong>DEMO | DOWNLOAD | DOCS</strong></p>
    <p>Features | License | Installation | Contributing | FAQ | Credits</p>
</div>

<hr />

<p align="center">
  <a href="#-features">Features</a> •
  <a href="#-installation">Installation</a> •
  <a href="#-contributing">Contributing</a> •
  <a href="#-tech-stack">Tech Stack</a> •
  <a href="#-faq">FAQ</a>
</p>

---

## 🌟 Vision
**rchart** is a community-driven Electronic Health Record (EHR) system designed to run anywhere—Windows, MacOS, and Linux. As a project, rchart strives to be: fast, intuitive, extensible via community developed plugins!

* **FOSS:** Free and Open Source Software forever
* **Physician-Centered:** Workflows designed with clinicans in mind, not billing codes
* **Offline-First:** Built-in local DB so you can work without a 5G connection
* **Global:** Multilanguage support for healthcare workers everywhere

---

## ✨ Features

### 📋 Clinical & Patient Management
* [x] **Universal Search:** Find patients, notes, or encounters in milliseconds
* [x] **Patient Profiles:** Comprehensive view of vitals, history, and active issues
* [ ] **Smart Calendar:** Appointment scheduling with real-time status tracking
* [ ] **Multi-Workplace Support:** Seamlessly switch between clinic, hospital, and private practice
* [x] **Encounter Logging:** Streamlined note-taking for every visit
* [ ] **Note Templates:** Custom "dot phrases" to speed up repetitive entries
* [ ] **Task List:** Built-in clinician to-do list integrated with patient charts

### 🔧 Extensibility
* [ ] **Plugin System:** Community-developed extensions (Stripe, Telehealth, etc.)
* [ ] **Global Language Support:** Fully localizable UI for healthcare workers worldwide
* [ ] **📊 Data Visualization:** Patient population health tools
* [ ] **📹 Telehealth:** Secure, encrypted video visits built directly into the chart.
* *[ ] **💳 Billing & Payments:** Integrated Stripe module for co-pays and invoicing.
* [ ] **🎙️ AI Tooling:** Note scribing, clinical decision support

> [!TIP]
> **Roadmap:** See our full 22-point roadmap in the [Wiki](#) (Auth, Billing, Chat, and more).

---

## 🛠 Tech Stack

| Component | Technology | Description |
| :--- | :--- | :--- |
| **Frontend** | [SvelteKit](https://kit.svelte.dev/) | Svelte 5 (Runes) for a fast, reactive UI |
| **Shell** | [Tauri](https://tauri.app/) | Secure, tiny binaries using the system's native webview. |
| **Database** | [SQLite](https://sqlite.org/) | Encrypted-at-rest local storage |
| **Logic** | [Rust](https://www.rust-lang.org/) | Memory-safe backend for handling sensitive PHI |

---

## 🚀 Installation

> [!IMPORTANT]
> **rchart** is currently in **Alpha**. It is recommended for testing and development only. Do not use for live patient data.

### 🔨 Building From Source
Ensure you have **Node.js**, **Yarn**, and **Rust** installed.
```bash
git clone [https://github.com/your-username/rchart.git](https://github.com/your-username/rchart.git)
cd rchart
yarn install
yarn dev
```
### 📥 Download Binaries
Download the latest installer for your platform:
- [**Windows (.msi)**](#)
- [**MacOS (.dmg)**](#)
- [**Linux (.deb / .AppImage)**](#)

### 🔨 Building from Source
Ensure you have **Node.js**, **Yarn**, and **Rust** installed.
```bash
# Clone the repository
git clone [https://github.com/your-username/rchart.git](https://github.com/your-username/rchart.git)

# Install dependencies
yarn install

# Run in development mode
yarn tauri dev


# Contributing
We can use a lot of help. We need those that can code, help support languages, create art, and manage the project! 

### Pull requests: 
Please utilize forks and pull requests to contribute. The default development branch is 'dev'

# 6. Credits
- Core Team: Logan Nguyen -- just me so far!
- Contributors: none so far!

# README TODO
- [ ] add pictures off the app
- [ ] create a logo
- [ ] more badges
- [ ] add link to website for the download button
- [ ] add link to docs website
- [ ] create demo site and link it
- [ ] donation link
- [ ] have the table of contents work properly
- [ ] list the current features
- [ ] add a video showing off the features
- [ ] link to roadmap on website
- [ ] explain AGPL3 and how you can use it
- [ ] properly explain how to dowload the app on each platform
- [ ] explain how to set up a dev enviroment
- [ ] add links to the tech stack technologies mentioned
- [ ] link code of conduct file
- [ ] create a discord server? slack?
- [ ] we need a contributor guide for adding language support
- [ ] add a guide for improving the documentation
- [ ] add a guide for reporting bug, etc
