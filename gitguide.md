
# 🧠 GIT CHEAT SHEET – *SalesTracking Project*

---

## 🛠️ One-Time Setup (Per Machine)
```bash
git config --global user.name "Your Name"
git config --global user.email "your@email.com"
```

---

## 📥 Start Work (Always do this first)
```bash
git pull origin main     # Pull latest changes from GitHub
```

---

## 💾 Save & Commit Your Changes
```bash
git add .                # Stage all changes
git commit -m "Your message here"  # Commit with a clear message
```

---

## 🚀 Push to GitHub
```bash
git push origin main     # Push your local commits to GitHub
```

---

## 📜 View Git History
```bash
git log --oneline        # Compact view of commit history
git log                  # Detailed history
```

---

## 🧭 Navigation & Undo

### Switch Between Versions
```bash
git checkout <commit_id> # Check out a previous commit (detached HEAD)
git switch main          # Return to latest commit on main branch
```

### Undo Changes
```bash
git restore <filename>   # Discard local changes to a file (before commit)
git reset --hard <commit_id>  # ⚠️ Reset project to earlier state (irreversible)
```

---

## 🌱 Optional: Create a Branch (For Safe Experimentation)
```bash
git checkout -b feature-name   # Create and switch to a new branch
# Work and commit as usual
git switch main                # Return to main
```

---

## 🔒 Backup Option (Manual)
- Zip your entire project folder  
- Or commit + push to GitHub (recommended)

---

## 💡 Pro Tip: Run Git From VS Code Terminal
```bash
# VS Code → Terminal → New Terminal
git status         # See what's changed
git add filename   # Add specific file
```
