# 🧭 KlaayGuard [Project ID: P-KG-001]

A cross-platform desktop security monitoring application that collects system information using osquery and reports it to a centralized API for comprehensive security analysis.

---

## 📚 Table of Contents

- [About](#about)
- [Features](#features)
- [Tech Stack](#tech-stack)
- [Installation](#installation)
- [Usage](#usage)
- [Screenshots](#screenshots)
- [Contact](#contact)

---

## 🧩 About

KlaayGuard is designed to provide enterprise-grade security monitoring for cross-platform desktop environments. It addresses the challenge of centralized security data collection and analysis by automatically managing osquery installations and transmitting security telemetry to a centralized monitoring platform.

> This project provides an automated security monitoring solution that runs silently in the background, collecting critical system information and reporting it to `https://api.klaay.dev` for real-time security analysis and threat detection.

---

## ✨ Features

- **Automated osquery Management** – Automatically installs and configures osquery on Windows, macOS, and Linux
- **Comprehensive Security Data Collection** – Monitors processes, network connections, installed software, and system configurations
- **Periodic Reporting** – Transmits security data to the central API every 15 minutes
- **System Tray Application** – Runs unobtrusively in the background with minimal user interaction
- **Secure Authentication** – Provides secure user authentication and encrypted data transmission
- **Cross-Platform Support** – Native desktop application for Windows, macOS, and Linux

---

## 🧠 Tech Stack

**Languages:** TypeScript, Rust  
**Frontend Framework:** React 18  
**Desktop Framework:** Tauri 2.x  
**UI Styling:** TailwindCSS  
**Security Engine:** osquery  
**Build Tools:** Vite, Cargo  
**Additional Libraries:** React Router, React Icons, ApexCharts, FullCalendar

---

## ⚙️ Installation

### Prerequisites

```bash
# Install Node.js (v22+ recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 22
nvm use 22

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Tauri CLI
cargo install tauri-cli

# Install Yarn (if not using npm)
npm install -g yarn
```

### Setup & Development

```bash
# Clone the repository
git clone https://github.com/klaayinc/klaayguard.git
cd klaayguard

# Install dependencies
yarn install

# Create environment file
echo "VITE_API_BASE_URL=https://api.klaay.dev" > .env
```

---

## 🚀 Usage

### Development Mode

```bash
# Start the development server
yarn tauri dev
```

The application will launch as a desktop app with hot-reload enabled for development.

### Production Build

```bash
# Build for current platform
yarn tauri build

# Build for specific platforms
yarn tauri build --target x86_64-pc-windows-msvc    # Windows
yarn tauri build --target aarch64-apple-darwin      # macOS Apple Silicon
yarn tauri build --target x86_64-apple-darwin       # macOS Intel
yarn tauri build --target x86_64-unknown-linux-gnu  # Linux
```

### Downloading Precompiled Builds (Mac)

1. Navigate to [https://github.com/klaayinc/klaayguard/releases](https://github.com/klaayinc/klaayguard/releases)
2. Look for the most recent "dev" release
3. Download the appropriate package (e.g., `klaay_XXX_aarch64.dmg` for Apple Silicon Macs)
4. Install the package

---

## 🧾 Configuration

### Environment Variables

Create a `.env` file with:

```
VITE_API_BASE_URL=https://api.klaay.dev
```

### API Endpoints

The application communicates with the following endpoints:

- `GET /klaayguard/config` – Fetch monitoring configuration
- `POST /klaayguard/data` – Submit collected system data

---

## 🖼 Screenshots

_Screenshots and demo GIFs will be added soon._

---

## 📬 Contact

**Author** Yu Du Song
**email** andyhung772@gmail.com
**Organization:** Klaay Inc.  
**GitHub:** [@klaayinc](https://github.com/waytothevenus)  
**API Endpoint:** [https://api.klaay.dev](https://api.klaay.dev)

---

## 🌟 Acknowledgements

- [Tauri](https://tauri.app) – Cross-platform desktop application framework
- [osquery](https://osquery.io) – SQL-powered operating system instrumentation and monitoring
- [React](https://react.dev) – UI library for building the frontend
- [TailwindCSS](https://tailwindcss.com) – Utility-first CSS framework

---

[![Built with Tauri](https://img.shields.io/badge/built%20with-Tauri-FFC131.svg?logo=tauri)](https://tauri.app)

# Build using Docker

docker compose run --rm klaayguard -- yarn run tauri build

```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test on target platforms
5. Submit a pull request

## 📄 License

[Add your license information here]
```
