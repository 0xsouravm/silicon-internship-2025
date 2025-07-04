# How to Contribute to This Repository

---

## 1. Fork the Repository

> **Note:** Forking must be done once via the GitHub website. After that, all steps are terminal-based.

Assume your fork is at `https://github.com/your-username/silicon-internship-2025.git` and the main repo is `https://github.com/0xsouravm/silicon-internship-2025.git`.

---
****
## 2. Clone Your Fork

```bash
git clone https://github.com/your-username/silicon-internship-2025.git
cd silicon-internship-2025
```

---

## 3. Add the Main Repository as Upstream

```bash
git remote add upstream https://github.com/0xsouravm/silicon-internship-2025.git
git remote -v
```

---

## 4. Create a New Branch for Your Changes

```bash
git checkout -b my-feature-branch
```

---

## 5. Make Changes and Commit

```bash
# Edit files as needed
git add .
git commit -m "Describe your changes"
```

---

## 6. Push Changes to Your Fork

```bash
git push origin my-feature-branch
```

---

## 7. Create a Pull Request

You can create a pull request using either the GitHub CLI or VS Code's GitHub Pull Request extension:

### Option 1: Using GitHub CLI

#### Install GitHub CLI

- **Linux (Debian/Ubuntu):**
  ```bash
  sudo apt install gh
  ```
- **Other methods & troubleshooting:**
  See the official installation guide: [https://github.com/cli/cli#installation](https://github.com/cli/cli#installation)

#### Create the Pull Request

```bash
gh pr create --base main --head your-username:my-feature-branch --title "PR Title" --body "PR Description"
```

### Option 2: Using VS Code

1. Install the "GitHub Pull Requests and Issues" extension in VS Code
   - Open VS Code
   - Go to Extensions (Ctrl+Shift+X)
   - Search for "GitHub Pull Requests and Issues"
   - Click Install

2. Create a Pull Request
   - Click on the Source Control icon in the VS Code sidebar (or press Ctrl+Shift+G)
   - Click on the "..." menu (More Actions)
   - Select "Create Pull Request"
   - Follow the prompts to:
     - Select the base branch (usually main)
     - Enter a title for your PR
     - Add a description
     - Review your changes
   - Click "Create" to submit your pull request

---

## 8. Syncing Your Fork with the Main Repository after changes made in main repository

### Fetch and Merge Changes from Upstream

```bash
git fetch upstream
git checkout main
git merge upstream/main
```

### Push Updates to Your Fork

```bash
git push origin main
```

---

## 9. Delete Branches After Merge (Optional)

```bash
git branch -d my-feature-branch
git push origin --delete my-feature-branch
```

---

## 10. Useful Commands

- List remotes:
  ```bash
  git remote -v
  ```
- Check status:
  ```bash
  git status
  ```
- Pull latest changes:
  ```bash
  git pull
  ```

---

**Note:** Replace `your-username` with your actual GitHub username. The repository name is `silicon-internship-2025` and the main repository username is `0xsouravm`.
