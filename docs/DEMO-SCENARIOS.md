# 🎬 Irmin View - Demo Scenarios Guide

A collection of practical demo scenarios to showcase Irmin View's capabilities and help users understand real-world usage patterns.

## 📋 Available Demo Data

### 📁 Demo Files Location
All demo files are located in `assets/demo-data/`:

- **`demo-commits.json`** - Complex commit history with multiple branches
- **`demo-branches.json`** - Various branch types and merge scenarios  
- **`demo-diffs.json`** - Different change types and diff examples
- **`enterprise-tree.json`** - Large-scale enterprise application structure
- **`sample-tree.json`** - Simple tree for basic demonstrations

---

## 🎯 Scenario 1: Basic Data Exploration

**Goal**: Learn the fundamentals of navigating an Irmin store

### Steps:

1. **Launch Application**
   ```bash
   cd src-tauri
   cargo tauri dev
   ```

2. **Explore the Tree Structure**
   - Start in Tree Browser (default view)
   - Click the root folder (📁 /) to expand
   - Notice files like `config.json`, `README.md`
   - Expand the `users/` directory
   - Observe file sizes and metadata

3. **Expected Results**
   ```
   📁 / (expanded)
   ├── 📄 config.json (45 B)
   ├── 📄 README.md (387 B)
   ├── 📁 users/ (expanded)
   │   ├── 📄 alice.json (125 B)
   │   ├── 📄 bob.json (118 B)
   │   └── 📄 charlie.json (124 B)
   └── 📁 logs/
       ├── 📄 app.log (245 B)
       └── 📄 error.log (112 B)
   ```

### Key Learning Points:
- Tree navigation with expand/collapse
- File vs directory identification
- Size and metadata display
- Nested directory structures

---

## 🎯 Scenario 2: Commit History Analysis

**Goal**: Understand version control and change tracking

### Steps:

1. **Switch to Commit History**
   - Click 📋 **Commit History** in sidebar
   - Wait for commits to load

2. **Analyze Commit Information**
   - Review the most recent commit:
     ```
     a1b2c3d4e5f6
     Add user management system
     Alice <alice@example.com> • 1/15/2024 6:46 AM • main
     ```
   - Note the progression of changes
   - Identify different authors and branches

3. **Observe Commit Patterns**
   - Main branch commits (production-ready)
   - Feature branch commits (development work)
   - Hotfix commits (urgent repairs)

### Key Learning Points:
- Git-like version control in Irmin
- Author attribution and timestamps
- Branch-based development workflow
- Commit message conventions

---

## 🎯 Scenario 3: Multi-Branch Development

**Goal**: Explore parallel development streams

### Steps:

1. **Access Branch Management**
   - Click 🌿 **Branches** in sidebar
   - Review all available branches

2. **Compare Branch Information**
   - **main**: Production branch (3 commits)
   - **feature/logging**: Development branch (1 commit)
   - Note different update times and commit counts

3. **Understanding Branch Status**
   ```
   main
   Head: a1b2c3d4e5f6
   Commits: 3
   Updated: 1/15/2024 6:46 AM
   
   feature/logging
   Head: d4c3b2a1f6e5
   Commits: 1
   Updated: 1/15/2024 9:00 AM
   ```

### Key Learning Points:
- Parallel development workflows
- Branch isolation and independence
- Head commit tracking
- Development vs production branches

---

## 🎯 Scenario 4: Visual Diff Analysis

**Goal**: Compare changes between versions

### Steps:

1. **Navigate to Diff Viewer**
   - Click 📊 **Diff Viewer** in sidebar
   - See the comparison interface

2. **Select Commits for Comparison**
   - **From**: `f6e5d4c3b2a1` (Update configuration schema)
   - **To**: `a1b2c3d4e5f6` (Add user management system)
   - Click **Show Diff**

3. **Analyze the Changes**
   - ✅ **Added**: `/users/alice.json` - New user data
   - ✏️ **Modified**: `/config.json` - Configuration updates
   - 🗑️ **Deleted**: `/old_file.txt` - Removed obsolete file

### Expected Diff Output:
```
✅ /users/alice.json (Added)
+ {"name": "Alice", "email": "alice@example.com", "role": "admin"}

✏️ /config.json (Modified)
- {"database": {"host": "localhost", "port": 5432}}
+ {"database": {"host": "localhost", "port": 5432}, "version": "1.0"}

🗑️ /old_file.txt (Deleted)
- This file was removed
```

### Key Learning Points:
- Visual change representation
- Addition, modification, deletion types
- Line-by-line diff analysis
- Content comparison capabilities

---

## 🎯 Scenario 5: Search and Discovery

**Goal**: Quickly find specific data in large stores

### Steps:

1. **Perform Basic Search**
   - In the Search section of sidebar
   - Enter "json" in search field
   - Click **Search** or press Enter

2. **Review Search Results**
   ```
   📄 /config.json
   File
   
   📄 /users/alice.json
   File
   
   📄 /users/bob.json
   File
   ```

3. **Try Different Search Terms**
   - Search "user" → finds user-related items
   - Search "log" → finds log files
   - Search "config" → finds configuration files

### Key Learning Points:
- Efficient data discovery
- Partial name matching
- Case-insensitive search
- Multiple result handling

---

## 🎯 Scenario 6: Theme and Customization

**Goal**: Personalize the interface for different work environments

### Steps:

1. **Test Light Theme** (Default)
   - Note the clean, professional appearance
   - White backgrounds with dark text
   - Blue accent colors

2. **Switch to Dark Theme**
   - Click 🌙 theme toggle in header
   - Icon changes to ☀️
   - Observe the visual transformation

3. **Compare Experiences**
   - **Light**: Better for bright environments, printing
   - **Dark**: Reduced eye strain, modern look
   - **Consistency**: All functionality remains the same

### Key Learning Points:
- Interface customization options
- Accessibility considerations
- Persistent theme preferences
- Visual comfort optimization

---

## 🎯 Scenario 7: Enterprise Data Structure

**Goal**: Navigate complex, real-world data organizations

### Demo Data: `enterprise-tree.json`

### Steps:

1. **Load Enterprise Demo**
   - Replace current mock data with enterprise structure
   - Restart application to see complex hierarchy

2. **Explore Enterprise Structure**
   ```
   📁 /enterprise-app
   ├── 📁 config/
   │   ├── 📄 database.json (312 B)
   │   ├── 📄 redis.json (156 B)
   │   └── 📄 api.json (245 B)
   ├── 📁 users/
   │   ├── 📁 admins/
   │   │   ├── 📄 alice.json (298 B)
   │   │   └── 📄 bob.json (312 B)
   │   └── 📁 employees/
   │       ├── 📄 charlie.json (287 B)
   │       └── 📄 diana.json (321 B)
   ├── 📁 services/
   │   ├── 📄 authentication.json (342 B)
   │   └── 📄 user-management.json (278 B)
   └── 📁 logs/
       ├── 📄 application.log (658 B)
       └── 📄 errors.log (412 B)
   ```

3. **Navigate Deep Hierarchies**
   - Expand multiple levels: `users/` → `admins/` → `alice.json`
   - Notice role-based organization
   - Observe file size variations

### Key Learning Points:
- Complex organizational structures
- Role-based access patterns
- Service-oriented architectures
- Enterprise-scale data management

---

## 🎯 Scenario 8: Development Workflow Simulation

**Goal**: Understand typical software development patterns

### Using: `demo-commits.json` and `demo-branches.json`

### Steps:

1. **Review Development Timeline**
   ```
   a1b2c3d4... - Add user authentication system (Alice, main)
   f6e5d4c3... - Update database configuration (Bob, main)  
   b2a1f6e5... - Initial project structure (Charlie, main)
   d4c3b2a1... - Add logging system (Diana, feature/logging)
   ```

2. **Analyze Branch Strategy**
   - **main**: Stable production releases
   - **develop**: Integration branch
   - **feature/**: Individual feature development
   - **hotfix/**: Critical bug fixes
   - **release/**: Release preparation

3. **Track Feature Development**
   - Feature branches created from main
   - Multiple developers working in parallel
   - Merge back to main when complete

### Key Learning Points:
- Git-flow development patterns
- Collaborative development workflows
- Branch protection strategies
- Release management processes

---

## 🎯 Scenario 9: Performance and Scale Testing

**Goal**: Test interface with large datasets

### Steps:

1. **Load Large Tree Structure**
   - Use `enterprise-tree.json` with deep nesting
   - Expand multiple directories simultaneously

2. **Test Search Performance**
   - Search for common terms across large datasets
   - Observe response times and result accuracy

3. **Navigation Efficiency**
   - Navigate deep hierarchies quickly
   - Use breadcrumb-style navigation mentally
   - Test expand/collapse performance

### Key Learning Points:
- Interface scalability
- Search optimization
- Large dataset navigation
- Performance considerations

---

## 🎯 Scenario 10: Integration Demonstration

**Goal**: Show how Irmin View fits into larger workflows

### Steps:

1. **Data Source Flexibility**
   - Current: Mock data demonstration
   - Future: Real Irmin store connection
   - Integration: API endpoints for live data

2. **Export and Sharing**
   - Visual screenshots for documentation
   - Diff analysis for code reviews
   - Branch status for project management

3. **Development Integration**
   - Use alongside Git workflows
   - Compare with traditional version control
   - Demonstrate Irmin advantages

### Key Learning Points:
- Tool ecosystem integration
- Workflow enhancement possibilities
- Future expansion capabilities
- Real-world application scenarios

---

## 🚀 Advanced Demo Techniques

### Custom Demo Data Creation

1. **Modify Existing Files**
   ```bash
   # Edit demo data files
   code assets/demo-data/demo-commits.json
   ```

2. **Add New Scenarios**
   - Create merge conflict examples
   - Add more complex branch structures
   - Include performance test data

3. **Test Edge Cases**
   - Very large files
   - Deep directory nesting
   - Unicode characters in names
   - Binary file representations

### Presentation Tips

1. **Start Simple**: Begin with basic tree navigation
2. **Build Complexity**: Gradually introduce advanced features
3. **Show Context**: Explain why each feature matters
4. **Interactive Demo**: Let audience try features themselves
5. **Real Scenarios**: Use examples from actual development work

---

## 📝 Demo Checklist

Before running a demo:

- [ ] Application builds and starts successfully
- [ ] All demo data files are present
- [ ] Network connectivity (if needed)
- [ ] Screen sharing/projection setup
- [ ] Backup plans for technical issues
- [ ] Audience-appropriate scenarios selected
- [ ] Time management for each scenario

### Quick Demo (5 minutes):
- Tree navigation
- Theme toggle
- Basic search

### Full Demo (15-30 minutes):
- All core features
- Multiple scenarios
- Advanced capabilities
- Q&A session

---

## 🎓 Learning Outcomes

After completing these scenarios, users should understand:

1. **Core Concepts**: Trees, commits, branches, diffs
2. **Navigation Skills**: Efficient data exploration
3. **Search Capabilities**: Quick data discovery
4. **Version Control**: Git-like change tracking
5. **Interface Customization**: Theme and preference options
6. **Real-World Applications**: Enterprise and development scenarios

For more detailed instructions, see the [UI Guide](UI-GUIDE.md) or [README](../README.md).