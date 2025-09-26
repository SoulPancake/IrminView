# 🧠 Irmin View - Desktop UI Explorer

A **cross-platform desktop UI application** for interacting with and visualizing **Irmin DB** stores — similar in spirit to MongoDB Compass, but tailored to Irmin's Git-like data model.

![Irmin View Demo](assets/demo-screenshot.png)

## ✨ Features

### Core UI Features
- ✅ **Tree Browser** - Navigate through branches, commits, and key/value pairs
- ✅ **Commit History** - View and explore commit timeline  
- ✅ **Branch Management** - Switch between and compare branches
- ✅ **Diff Viewer** - Compare changes between commits with visual diff
- ✅ **Search & Filter** - Find keys and nodes quickly
- ✅ **Dark/Light Mode** - Toggle between themes
- ✅ **Responsive Design** - Works on different screen sizes

### Current Implementation
- **Mock Data Support** - Complete UI with sample Irmin data
- **Rust Backend** - Tauri-based desktop application  
- **Modern Web UI** - HTML/CSS/JavaScript frontend
- **Cross-Platform** - Windows, macOS, Linux support

## 🚀 Quick Start

### Method 1: Using Makefile (Recommended)

1. **Clone the repository**
   ```bash
   git clone https://github.com/SoulPancake/IrminView.git
   cd IrminView
   ```

2. **Install dependencies and run the desktop app**
   ```bash
   # Install system dependencies automatically
   make install-deps
   
   # Run the desktop application (uses demo data by default)
   make run-app
   ```

3. **Alternative: Full setup with server backend**
   ```bash
   # Complete setup: dependencies + demo data + server + app build
   make complete-setup
   
   # Run with server backend
   make run-app-http
   ```

### Method 2: Manual Setup

1. **Prerequisites**
   - [Rust](https://rustup.rs/) (1.70 or later)
   - System dependencies: `pkg-config`, `libgtk-3-dev`, `libwebkit2gtk-4.1-dev`

2. **Build and run in development mode**
   ```bash
   cd src-tauri
   cargo run
   ```

3. **Build for production**
   ```bash
   cd src-tauri
   cargo build --release
   ```

### Available Makefile Commands

- `make run-app` - Run desktop app with demo data (no server needed)
- `make run-app-http` - Run desktop app connected to Irmin server
- `make build-app` - Build desktop application for production
- `make install-deps` - Install system dependencies
- `make help` - Show all available commands

## 🏗️ Architecture

```
IrminView/
├── src-tauri/          # Rust backend (Tauri application)
│   ├── src/
│   │   ├── main.rs     # Application entry point
│   │   ├── lib.rs      # Library exports
│   │   ├── irmin/      # Irmin data structures and logic
│   │   └── ui/         # UI-related commands and state
│   ├── tauri.conf.json # Tauri configuration
│   └── Cargo.toml      # Rust dependencies
├── frontend/
│   └── dist/           # Frontend assets (HTML/CSS/JS)
├── assets/
│   └── mock-data/      # Sample Irmin data for testing
├── tests/              # Integration tests
├── e2e/                # End-to-end tests (future)
└── README.md
```

## 🧪 Testing

### Unit Tests
```bash
cd src-tauri
cargo test
```

### Integration Tests
```bash
cd src-tauri
cargo test --test integration_tests
```

### Frontend Testing (Development)
Open the application and test:
- Tree navigation and expansion
- Theme switching
- Search functionality
- Commit and branch views
- Diff visualization

## 📊 Test Scenarios Covered

1. 🟢 **Initial Load** - Empty Irmin store, blank state
2. 🌿 **Simple Tree** - One branch, multiple keys and nested nodes  
3. 🌳 **Multi-Branch View** - Two branches with common ancestor
4. ⚔️ **Merge Conflict View** - Simulated conflict between commits (future)
5. 🔍 **Search Results** - Filter tree by key name
6. 🌘 **Dark/Light Mode** - UI appearance toggle

## 🔧 Tech Stack

- **Backend**: Rust + Tauri 2.0
- **Frontend**: Vanilla HTML/CSS/JavaScript
- **UI**: Custom responsive design
- **Data**: Mock Irmin structures (extensible to real Irmin)
- **Build**: Cargo + Tauri CLI

## 🎯 Current Status

### ✅ Completed
- Project scaffolding and structure
- Core Rust data types for Irmin concepts
- Complete UI with tree browser, commits, branches, diff viewer
- Theme switching (dark/light mode)
- Search functionality
- Mock data generation for testing
- Basic unit and integration tests
- Cross-platform desktop app foundation

### 🚧 In Progress
- Real Irmin backend integration
- Enhanced diff visualization
- Visual regression testing
- E2E test suite

### 📋 Future Enhancements
- Irmin store connection management
- Advanced merge conflict resolution
- Plugin system for extensibility
- Performance optimizations for large datasets
- Collaborative features

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT OR Apache-2.0 License - see the [LICENSE](LICENSE) files for details.

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - For the excellent desktop app framework
- [Irmin](https://irmin.org/) - For the inspiring Git-like database concepts
- The Rust and Web communities for amazing tools and libraries

---

> **Note**: This application currently uses mock data to demonstrate the UI and features. Integration with real Irmin stores is planned for future releases.

For questions, issues, or feature requests, please open an issue on GitHub.