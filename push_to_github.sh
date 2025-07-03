#!/bin/bash
# Script to push TauFoldZKVM to GitHub repository

set -e

echo "🚀 Preparing to push TauFoldZKVM to GitHub..."

# Repository URL
REPO_URL="https://github.com/TheDarkLightX/TauFoldzkVM.git"
TARGET_DIR="${1:-../../TauFoldZKVM}"

# First, run the preparation script if not already done
if [ ! -d "$TARGET_DIR" ]; then
    echo "📁 Running preparation script first..."
    ./prepare_standalone.sh "$TARGET_DIR"
fi

cd "$TARGET_DIR"

# Update repository URLs in files
echo "📝 Updating repository URLs..."
sed -i '' 's|https://github.com/YOUR_USERNAME/TauFoldZKVM|https://github.com/TheDarkLightX/TauFoldzkVM|g' README.md 2>/dev/null || \
sed -i 's|https://github.com/YOUR_USERNAME/TauFoldZKVM|https://github.com/TheDarkLightX/TauFoldzkVM|g' README.md

sed -i '' 's|https://github.com/YOUR_USERNAME/TauFoldZKVM|https://github.com/TheDarkLightX/TauFoldzkVM|g' Cargo.toml 2>/dev/null || \
sed -i 's|https://github.com/YOUR_USERNAME/TauFoldZKVM|https://github.com/TheDarkLightX/TauFoldzkVM|g' Cargo.toml

sed -i '' 's|https://github.com/YOUR_USERNAME/TauFoldZKVM|https://github.com/TheDarkLightX/TauFoldzkVM|g' CONTRIBUTING.md 2>/dev/null || \
sed -i 's|https://github.com/YOUR_USERNAME/TauFoldZKVM|https://github.com/TheDarkLightX/TauFoldzkVM|g' CONTRIBUTING.md

# Initialize git if needed
if [ ! -d .git ]; then
    echo "📁 Initializing git repository..."
    git init
fi

# Add all files
echo "📋 Adding files to git..."
git add .

# Create initial commit
echo "💾 Creating initial commit..."
git commit -m "Initial commit: TauFoldZKVM v1.0.0

A production-ready zero-knowledge virtual machine with mathematical correctness guarantees.

Features:
- 45 complete instructions with Tau constraint validation
- Python and Rust implementations
- Rich TUI with debugger and profiler
- 5 demo applications (calculator, crypto, smart contracts, games)
- 457 validated Tau constraint files
- Comprehensive documentation and examples

Built on the foundations of Tau language constraint satisfaction and Boolean contract design."

# Add remote
echo "🔗 Adding GitHub remote..."
git remote add origin "$REPO_URL" 2>/dev/null || git remote set-url origin "$REPO_URL"

# Create and push main branch
echo "⬆️ Pushing to GitHub..."
git branch -M main
git push -u origin main

echo "✅ Successfully pushed to GitHub!"
echo ""
echo "🎉 TauFoldZKVM is now live at: https://github.com/TheDarkLightX/TauFoldzkVM"
echo ""
echo "📋 Next steps:"
echo "1. Go to https://github.com/TheDarkLightX/TauFoldzkVM/settings"
echo "2. Add description: 'Production-ready zero-knowledge VM with mathematical guarantees'"
echo "3. Add topics: zkvm, zero-knowledge, tau, constraint-satisfaction, rust, python"
echo "4. Enable GitHub Pages for documentation"
echo "5. Set up GitHub Actions secrets if needed"
echo "6. Create initial release: v1.0.0"
echo ""
echo "🚀 Optional enhancements:"
echo "- Add shields/badges to README"
echo "- Create GitHub Pages site"
echo "- Set up issue templates"
echo "- Configure security policy"
echo "- Add code of conduct"