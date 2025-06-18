# Blog Publisher Script

A rust script to automate the process of adding a new blog entry to a static site.
It does the following:

1. Reads a pre-generated HTML blog file (`~/Documents/source.html`). I export my blog from an app called Bear Notes.
2. Moves it to the `public/` directory with a new filename
3. Updates the `App.jsx` file to include a link to the new blog
4. Commits and pushes the changes to GitHub

## Usage

Run the script with two arguments:

```bash
cargo run -- <title_with_underscores> "<description in quotes>"
```

### Example:
```bash
cargo run -- my_first_blog "A short intro about my first blog post"
```

This will:
1. Rename and move ~/Documents/source.html to ~/Developer/me/public/my_first_blog.html
2. Add a new link to ~/Developer/me/src/App.jsx
3. Commit the changes and push to the GitHub repo


### Requirements
- Rust installed
- source.html must be present at ~/Documents/
- Git must be initialized in ~/Developer/me/
- App.jsx must exist at ~/Developer/me/src/App.jsx
- Blog items in App.jsx should use the `<BlogItem />` format. Feel free to refer the code on my Github for the website itself.

### What gets committed

The commit message will be:
add '\<title>' to blog

### Notes
- Underscores in the title will be replaced with spaces for display.
- Description should be wrapped in quotes to be treated as a single argument.
