#!/bin/bash

# IrminView - OCaml Irmin Integration Setup Script
# This script sets up the OCaml environment and builds the Irmin bridge

set -e

echo "🧬 Setting up Irmin integration for IrminView..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "dune-project" ]; then
    print_error "Please run this script from the IrminView root directory"
    exit 1
fi

# Step 1: Check for OCaml and opam
print_status "Checking OCaml installation..."

if ! command -v ocaml &> /dev/null; then
    print_warning "OCaml not found. Installing..."
    
    # Try to install OCaml based on the system
    if command -v apt-get &> /dev/null; then
        sudo apt-get update
        sudo apt-get install -y ocaml opam m4
    elif command -v brew &> /dev/null; then
        brew install ocaml opam
    else
        print_error "Please install OCaml and opam manually"
        exit 1
    fi
else
    print_success "OCaml found: $(ocaml -version)"
fi

# Step 2: Initialize opam if needed
print_status "Setting up opam environment..."

if [ ! -d "$HOME/.opam" ]; then
    print_status "Initializing opam..."
    opam init --disable-sandboxing -a -y
fi

# Source opam environment
eval $(opam env)

# Step 3: Install Irmin dependencies
print_status "Installing Irmin and dependencies..."

# Check if packages are already installed
if opam list irmin &> /dev/null; then
    print_success "Irmin packages already installed"
else
    print_status "Installing required OCaml packages..."
    opam install -y dune irmin irmin-fs irmin-git yojson lwt cmdliner
fi

# Step 4: Build the OCaml bridge
print_status "Building OCaml bridge..."

if dune build; then
    print_success "OCaml bridge built successfully"
else
    print_error "Failed to build OCaml bridge"
    exit 1
fi

# Step 5: Install the CLI executable
print_status "Installing irmin-bridge-cli..."

if dune install; then
    print_success "irmin-bridge-cli installed"
else
    print_warning "Failed to install CLI, trying local build..."
fi

# Step 6: Test the installation
print_status "Testing installation..."

if command -v irmin-bridge-cli &> /dev/null; then
    print_success "irmin-bridge-cli is available in PATH"
    irmin-bridge-cli --help | head -5
else
    # Check if it's in the local build directory
    if [ -f "_build/install/default/bin/irmin-bridge-cli" ]; then
        print_warning "CLI built locally but not in PATH"
        print_status "You can run it with: ./_build/install/default/bin/irmin-bridge-cli"
    else
        print_error "CLI not found after installation"
        exit 1
    fi
fi

# Step 7: Set up a demo Irmin store
print_status "Setting up demo Irmin store..."

DEMO_STORE="./demo_irmin_store"

if [ ! -d "$DEMO_STORE" ]; then
    mkdir -p "$DEMO_STORE"
    cd "$DEMO_STORE"
    git init
    
    # Create a simple OCaml script to populate the store
    cat > setup_demo.ml << 'EOF'
#require "irmin,irmin-git,lwt.unix";;
open Lwt.Syntax;;

module Store = Irmin_git.FS.G (Irmin.Contents.String) (Irmin.Path.String_list) (Irmin.Branch.String);;

let setup_demo () =
  let config = Irmin_git.config "." in
  let* repo = Store.Repo.init config in
  let* main = Store.of_branch repo "main" in
  
  (* Add some demo data *)
  let* () = Store.set_exn main ["users"; "alice"] "Alice Smith" in
  let* () = Store.set_exn main ["users"; "bob"] "Bob Johnson" in
  let* () = Store.set_exn main ["config"; "database"] "localhost:5432" in
  let* () = Store.set_exn main ["config"; "cache_ttl"] "3600" in
  let* () = Store.set_exn main ["data"; "metrics"] "active_users:150" in
  
  Printf.printf "Demo Irmin store created successfully!\n";
  Lwt.return ()
;;

Lwt_main.run (setup_demo ());;
EOF
    
    # Run the setup script
    if ocaml setup_demo.ml; then
        print_success "Demo Irmin store created at $DEMO_STORE"
    else
        print_warning "Failed to create demo store, but directory exists"
    fi
    
    cd ..
else
    print_success "Demo Irmin store already exists at $DEMO_STORE"
fi

# Step 8: Test the bridge with the demo store
print_status "Testing bridge with demo store..."

if command -v irmin-bridge-cli &> /dev/null; then
    if irmin-bridge-cli tree --path "$DEMO_STORE" &> /dev/null; then
        print_success "Bridge successfully connected to demo store!"
    else
        print_warning "Bridge built but couldn't connect to store (this is OK for empty stores)"
    fi
fi

# Step 9: Build Rust application
print_status "Building Rust application..."

cd src-tauri

# Set PKG_CONFIG_PATH for system dependencies
export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig

if cargo build; then
    print_success "Rust application built successfully"
else
    print_error "Failed to build Rust application"
    cd ..
    exit 1
fi

cd ..

# Final summary
echo ""
print_success "🎉 Irmin integration setup complete!"
echo ""
echo "Next steps:"
echo "1. Start the application: cd src-tauri && cargo run"
echo "2. The UI will show Irmin-like demo data (falls back gracefully)"
echo "3. To use a real Irmin store, set IRMIN_STORE_PATH=/path/to/store"
echo ""
echo "Files created:"
echo "- OCaml bridge: ocaml-bridge/"
echo "- Demo store: $DEMO_STORE"
echo "- Documentation: IRMIN_INTEGRATION.md"
echo ""
print_status "Enjoy exploring Irmin data with IrminView! 🧬"