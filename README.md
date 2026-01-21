<h1 align="center">🩺 rchart</h1>

<div align="center">
    <strong>
        A modern, free, extensible EHR for the world
    </strong>
    Built by doctors for doctors.
</div>

<br />

[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](code_of_conduct.md)

<div align="center">
    <p><strong>DEMO | DOWNLOAD | DOCS</strong></p>
    <p>Features | License | Installation | Contributing | FAQ | Credits</p>
</div>
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

<hr />

<p align="center">
  <a href="#-features">Features</a> •
  <a href="#-installation">Installation</a> •
  <a href="#-contributing">Contributing</a> •
  <a href="#-tech-stack">Tech Stack</a> •
  <a href="#-faq">FAQ</a>
</p>


**rchart** is a community developed electronic health record (EHR) that runs on Windows, MacOS, and Linux. As a project, rchart strives to be: fast, intuitive, extensible via community developed plugins!
---

### 🌟 The Vision
**rchart** is a community-driven Electronic Health Record (EHR) system designed to run anywhere—Windows, MacOS, and Linux. We believe medical software should be fast, offline-first, and focused entirely on the patient-physician relationship.

* **Physician-Centered:** Zero bloat. Focus on clinical efficiency, not clicking boxes for insurance companies.
* **Offline-First:** Critical medical data shouldn't rely on a 5G connection. Sync when you can; work when you must.
* **Privacy by Default:** Local-first storage with industry-standard encryption.

---
### Values:
1. FOSS - Free and Open Source Software
2. Physcian Centered Design - modern, efficient, productive workflows--no focus on billing!
3. Offline First - not everyone has 24/7 access to fast internet
4. Global Language Support - use rchart no matter what language you speak

# 1. Features

### 📋 Patient Management
* [x] **Universal Search:** Find patients, notes, or encounters in milliseconds.
* [x] **Patient Profiles:** Comprehensive view of vitals, history, and active issues.
* [ ] **Multi-Workplace Support:** Seamlessly switch between clinic, hospital, and private practice.

### ✍️ Clinical Documentation
* [x] **Encounter Logging:** Streamlined note-taking for every visit.
* [ ] **Note Templates:** Custom "Smart Phrases" to speed up repetitive entries.
* [ ] **Task List:** Built-in clinician to-do list integrated with patient charts.

### 🔧 Extensibility
* [ ] **Plugin System:** Add Stripe payments, Telehealth (WebRTC), or Lab integrations with one click.
* [ ] **Global Language Support:** Fully localizable UI for healthcare workers worldwide.

---

Missing a feature? Check out our roadmap:
1. new patient
2. patient list
3. view patient info
4. edit patient info
5. new patient encounter
6. add patient note
7. edit patient encounter
8. auth
9. support multiple users at once
10. support multiple workplaces
11. task list
12. calendar
13. global search function
14. note templates
15. custom UI themes
16. multilanguage support
17. custom niche workflows
18. extension support
19. default clinical support tools
20. patient population data visualization tools
21. billing
22. chat

# 2. License

# 3. Installation
You can download via the following. Alternatively you can download on your specific platform or build the program from source. What is building from source? Explained:
## Windows
## MacOS
## Linux

## 🛠 Tech Stack

| Component | Technology | Why? |
| :--- | :--- | :--- |
| **Frontend** | [SvelteKit](https://kit.svelte.dev/) | Lightning-fast UI with zero-overhead reactivity. |
| **Shell** | [Tauri](https://tauri.app/) | Secure, tiny binaries using the system's native webview. |
| **Logic** | [Rust](https://www.rust-lang.org/) | Memory safety and performance for sensitive medical data. |
| **Database** | [SQLite](https://sqlite.org/) | Local-first, portable, and easily encrypted via SQLCipher. |
| **Styling** | [TailwindCSS](https://tailwindcss.com/) | Rapid, maintainable UI development. |

---

## 🚀 Installation

> [!IMPORTANT]
> **rchart** is currently in **Alpha**. It is recommended for testing and development only. Do not use for live patient data yet.

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


# 4. Contributing
We can use a lot of help. We need those that can code, help support languages, create art, and manage the project! 

## Tech Stack
We are using Svelte (specifically the SvelteKit metaframework) with TypeScript enabled for the frontend UI. TailwindCSS helps us organize our CSS design system. On the backend, we are using Tauri and Rust connected to a SQLite database.

## How to Contribute
- dependencies
- building, running
- conventions

### Our Contributor's Code of Conduct:

### Adding language support:

### Improving the documentation:

### Reporting bugs:

### Testing:
- acessibility
- language

### Pull requests: 
Please utilize forks and pull requests to contribute. The default development branch is 'dev'

# 5. FAQ
So far no questions have been asked!

## Developing
- yarn install
- yarn dev

# 6. Credits
- Core Team: Logan Nguyen -- just me so far!
- Contributors: none so far!
- backers, sponsors?
- future dono link?

# README TODO
- [ ] add pictures off the app
- [ ] create a logo
- [ ] more badges
- [ ] add link to website for the download button
- [ ] add link to docs website
- [ ] create demo site and link it
- [ ] add an emogji in front of the motto
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
