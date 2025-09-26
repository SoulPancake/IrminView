# 🎯 Irmin View - Complete UI Guide

A comprehensive step-by-step guide to using the Irmin View desktop application for exploring and managing Irmin DB stores.

## 📋 Table of Contents

1. [Getting Started](#getting-started)
2. [Main Interface Overview](#main-interface-overview)
3. [Tree Browser](#tree-browser)
4. [Commit History](#commit-history)
5. [Branch Management](#branch-management)
6. [Diff Viewer](#diff-viewer)
7. [Search & Filter](#search--filter)
8. [Theme & Settings](#theme--settings)
9. [Advanced Features](#advanced-features)
10. [Troubleshooting](#troubleshooting)

---

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ installed
- Git for version control
- Terminal/Command prompt access

### Launch the Application

1. **Clone the repository**:
   ```bash
   git clone https://github.com/SoulPancake/IrminView.git
   cd IrminView
   ```

2. **Start the development server**:
   ```bash
   cd src-tauri
   cargo tauri dev
   ```

3. **Wait for compilation** - The application will automatically open when ready.

---

## 🏠 Main Interface Overview

The Irmin View interface consists of four main areas:

```
┌─────────────────────────────────────────────────────────┐
│ 🏠 Header: App Title + Theme Toggle + Refresh          │
├─────────────────┬───────────────────────────────────────┤
│ 📁 Sidebar      │ 📊 Main Content Area                 │
│                 │                                       │
│ • Navigation    │ • Tree Browser                        │
│ • Search        │ • Commit History                      │
│ • Results       │ • Branch Management                   │
│                 │ • Diff Viewer                         │
├─────────────────┴───────────────────────────────────────┤
│ 📊 Status Bar: Connection + Item Count                 │
└─────────────────────────────────────────────────────────┘
```

### Header Components
- **App Title**: "Irmin View - Desktop UI Explorer"
- **Theme Toggle**: 🌙/☀️ button to switch between light and dark modes
- **Refresh Button**: 🔄 to reload data from the Irmin store

### Sidebar Sections
- **Navigation Menu**: Switch between different views
- **Search**: Find specific keys or values
- **Search Results**: Display matching items

### Status Bar Information
- **Connection Status**: Shows current data source (Mock Data Connected)
- **Item Count**: Total number of items in current view

---

## 🌳 Tree Browser

The Tree Browser is your main tool for exploring the Irmin store structure.

### Basic Navigation

1. **Access Tree Browser**:
   - Click the 🌳 **Tree Browser** button in the sidebar
   - This is the default view when the app starts

2. **Expand Directories**:
   - Click on any folder icon (📁) to expand/collapse
   - Directory names show without file extensions
   - Files show with their extensions and sizes

3. **View File Information**:
   - **File Icon**: 📄 for regular files
   - **Directory Icon**: 📁 for folders
   - **Size Information**: Displayed in bytes (e.g., "45 B", "1.2 KB")

### Tree Structure Examples

```
📁 / (root)
├── 📄 config.json (45 B)
├── 📄 README.md (387 B)
├── 📁 users/
│   ├── 📄 alice.json (125 B)
│   ├── 📄 bob.json (118 B)
│   └── 📄 charlie.json (124 B)
└── 📁 logs/
    ├── 📄 app.log (245 B)
    └── 📄 error.log (112 B)
```

### Tree Interaction Tips

- **Single Click**: Expand/collapse directories
- **Visual Feedback**: Expanded nodes show indented children
- **Nested Navigation**: Explore deep directory structures easily
- **File Metadata**: Size and type information at a glance

---

## 📋 Commit History

View and analyze the complete commit timeline of your Irmin store.

### Accessing Commit History

1. **Navigate to Commits**:
   - Click 📋 **Commit History** in the sidebar navigation
   - Wait for commits to load

2. **Commit Information Display**:
   ```
   ┌─────────────────────────────────────────┐
   │ a1b2c3d4e5f6 (commit hash)             │
   │ Add user management system              │
   │ Alice <alice@example.com> • 1/15/2024  │
   │ 6:46 AM • main                          │
   └─────────────────────────────────────────┘
   ```

### Understanding Commit Data

Each commit entry shows:
- **Commit Hash**: Unique identifier (shortened)
- **Commit Message**: Description of changes
- **Author**: Name and email address
- **Timestamp**: Date and time of commit
- **Branch**: Which branch the commit belongs to

### Commit History Features

- **Chronological Order**: Latest commits appear first
- **Multi-Branch Support**: Commits from all branches
- **Rich Metadata**: Complete author and timing information
- **Visual Separation**: Each commit in its own card

---

## 🌿 Branch Management

Manage and explore different branches in your Irmin store.

### Viewing Branches

1. **Access Branch View**:
   - Click 🌿 **Branches** in the sidebar
   - All branches load automatically

2. **Branch Information Cards**:
   ```
   ┌─────────────────────────────┐
   │ main                        │
   │ Head: a1b2c3d4e5f6          │
   │ Commits: 3                  │
   │ Updated: 1/15/2024 6:46 AM  │
   └─────────────────────────────┘
   ```

### Branch Details

Each branch card displays:
- **Branch Name**: e.g., "main", "feature/auth"
- **Head Commit**: Latest commit hash
- **Commit Count**: Total commits in branch
- **Last Updated**: When the branch was last modified

### Branch Types

Common branch patterns you'll see:
- **main**: Primary production branch
- **develop**: Development integration branch
- **feature/***: Feature development branches
- **hotfix/***: Critical bug fixes
- **release/***: Release preparation branches

---

## 📊 Diff Viewer

Compare changes between commits with visual diff highlighting.

### Creating a Diff

1. **Access Diff Viewer**:
   - Click 📊 **Diff Viewer** in the sidebar

2. **Select Commits**:
   - **From Commit**: Choose the earlier commit (baseline)
   - **To Commit**: Choose the later commit (comparison)
   - Click **Show Diff** button

3. **Diff Controls**:
   ```
   [Select from commit...] → [Select to commit...] [Show Diff]
   ```

### Understanding Diff Output

The diff viewer shows three types of changes:

#### ✅ Added Files
```
┌─────────────────────────────────────┐
│ ✅ /users/alice.json (Added)        │
│ + {"name": "Alice", "email": "..."}  │
└─────────────────────────────────────┘
```

#### ✏️ Modified Files
```
┌─────────────────────────────────────┐
│ ✏️ /config.json (Modified)          │
│ - {"database": {"host": "..."}}      │
│ + {"database": {"host": "..."}, ...} │
└─────────────────────────────────────┘
```

#### 🗑️ Deleted Files
```
┌─────────────────────────────────────┐
│ 🗑️ /old_file.txt (Deleted)          │
│ - This file was removed             │
└─────────────────────────────────────┘
```

### Diff Features

- **Color Coding**: Green for additions, red for deletions, yellow for modifications
- **Line-by-Line Changes**: See exact content differences
- **File Path Display**: Complete path to changed files
- **Change Type Icons**: Visual indicators for change types

---

## 🔍 Search & Filter

Quickly find specific keys, files, or content in your Irmin store.

### Using Search

1. **Access Search**:
   - Look for the "Search" section in the sidebar
   - Always visible regardless of current view

2. **Perform a Search**:
   ```
   ┌─────────────────────────┐
   │ Search keys... [Search] │
   └─────────────────────────┘
   ```
   - Enter search term in the input field
   - Press Enter or click the **Search** button

3. **View Results**:
   ```
   ┌─────────────────────────┐
   │ 📄 /config.json         │
   │ File                    │
   └─────────────────────────┘
   ```

### Search Features

- **Real-time Results**: Instant feedback as you type
- **Key Matching**: Searches through file and directory names
- **Case Insensitive**: Finds matches regardless of capitalization
- **Partial Matching**: Finds items containing your search term
- **Click to Navigate**: Click results to navigate to that item

### Search Tips

- **Use Partial Terms**: Search "json" to find all JSON files
- **Directory Names**: Search "user" to find user-related directories
- **File Extensions**: Search ".log" to find log files
- **Specific Names**: Search exact names for precise matches

---

## 🎨 Theme & Settings

Customize the appearance and behavior of Irmin View.

### Theme Switching

1. **Toggle Theme**:
   - Click the theme button in the header: 🌙 (for dark) or ☀️ (for light)
   - Theme changes instantly

2. **Theme Differences**:

   **Light Theme**:
   - White/light gray backgrounds
   - Dark text for high contrast
   - Blue accent colors
   - Professional, clean appearance

   **Dark Theme**:
   - Dark gray/black backgrounds
   - Light text for reduced eye strain
   - Bright accent colors
   - Modern, sleek appearance

### Visual Preferences

- **Automatic Persistence**: Your theme choice is remembered
- **System Integration**: Themes work well with OS preferences
- **Accessibility**: Both themes provide good contrast ratios

---

## 🚀 Advanced Features

### Keyboard Shortcuts

| Action | Shortcut | Description |
|--------|----------|-------------|
| Refresh | F5 | Reload data from Irmin store |
| Search | Ctrl+F | Focus search input |
| Theme Toggle | Ctrl+T | Switch between light/dark |

### Data Integration

The current version uses mock data to demonstrate functionality:

- **Mock Tree Structure**: Realistic file system representation
- **Sample Commits**: Representative git-like history
- **Branch Simulation**: Multiple development branches
- **Diff Examples**: Various types of changes

### Performance Considerations

- **Lazy Loading**: Large trees load efficiently
- **Smooth Animations**: 300ms transitions between views
- **Responsive Design**: Works on different screen sizes
- **Memory Efficient**: Optimized for large datasets

---

## 🛠️ Troubleshooting

### Common Issues

#### Application Won't Start
```bash
# Solution: Ensure Rust and dependencies are installed
cargo --version
rustup update
```

#### Blank Screen on Launch
- Check if the frontend files are present in `frontend/dist/`
- Verify Tauri configuration in `src-tauri/tauri.conf.json`
- Try rebuilding: `cargo tauri build`

#### Search Not Working
- Ensure you're in a view with data loaded
- Check that mock data is properly loaded
- Try refreshing with the 🔄 button

#### Theme Not Switching
- Try clicking the theme button again
- Check browser developer tools for JavaScript errors
- Refresh the application

### Getting Help

1. **Check Logs**: Look at the terminal where you ran `cargo tauri dev`
2. **Developer Tools**: Press F12 to open browser dev tools
3. **GitHub Issues**: Report bugs on the project repository
4. **Documentation**: Refer to the README.md for setup instructions

### Performance Optimization

- **Large Trees**: Use search to navigate instead of expanding everything
- **Memory Usage**: Refresh the app periodically for large datasets
- **Network**: For remote Irmin stores, ensure stable connection

---

## 🎯 Quick Reference

### Essential Workflows

1. **Explore Data Structure**:
   Tree Browser → Expand directories → View files

2. **Analyze Changes**:
   Commit History → Note hashes → Diff Viewer → Compare

3. **Find Specific Data**:
   Search → Enter term → Click results

4. **Switch Views**:
   Sidebar navigation → Click desired view

5. **Customize Appearance**:
   Header → Theme toggle → Instant change

### Status Indicators

- 🟢 **Connected**: Green dot = data loaded successfully
- 🔄 **Loading**: Spinning icon = data loading
- ❌ **Error**: Red indicator = connection issues
- 📊 **Count**: Number shows total items in view

---

## 📚 Next Steps

Once you're comfortable with the basics:

1. **Explore Demo Data**: Check out `assets/demo-data/` for complex examples
2. **Integrate Real Data**: Connect to actual Irmin stores (future feature)
3. **Customize Views**: Modify the UI to suit your workflow
4. **Contribute**: Help improve Irmin View on GitHub

---

> **💡 Tip**: The best way to learn Irmin View is to explore! Start with the Tree Browser, try different searches, and experiment with the diff viewer to understand your data structure.

For more information, see the main [README.md](../README.md) or visit the [GitHub repository](https://github.com/SoulPancake/IrminView).