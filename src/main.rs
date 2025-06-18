use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

struct Data {
    contents: String,
    url: String,
    title: String,
    description: String,
}

fn input() -> Result<Data, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err("Provide a name and description (inside quotes) for the HTML file.".into());
    }
    let title = args[1].replace("_", " ");

    let contents = read_file();
    let data = Data {
        url: args[1].clone(),
        description: args[2].clone(),
        contents,
        title,
    };

    Ok(data)
}

fn move_file(url: &str) -> Result<(), Box<dyn Error>> {
    let home = env::var("HOME").expect("Can't read your HOME environment variable");

    let from = format!("{}/Documents/source.html", home);
    let to = format!("{}/Developer/me/public/{}.html", home, url);
    println!("{to}");

    fs::rename(from, to)?;
    Ok(())
}

fn read_file() -> String {
    let home = env::var("HOME").expect("Couldn't find home directory");
    let path = format!("{}/Developer/me/src/App.jsx", home);
    fs::read_to_string(&path).unwrap()
}

fn write_to_file(data: &Data) -> Result<(), Box<dyn Error>> {
    let contents = &data.contents;
    let url = &data.url;
    let title = &data.title;
    let description = &data.description;

    let home = env::var("HOME").expect("Couldn't find home directory");
    let path = format!("{}/Developer/me/src/App.jsx", home);

    let mut len = contents.lines().collect::<Vec<&str>>().len();

    for (i, line) in contents.lines().rev().enumerate() {
        if line.contains("BlogItem") {
            len = len - i;
            break;
        }
    }

    let extra = format!(
        r#"
<a href="{}.html" target="_blank">
    <BlogItem
        title="{}"
        description="{}"
    ></BlogItem>
</a>"#,
        url, title, description
    );

    let result = append_text_to_string(&contents, len, &extra);
    fs::write(&path, result)?;
    Ok(())
}

fn append_text_to_string(original: &str, lc: usize, extra: &str) -> String {
    let mut lines: Vec<String> = original.lines().map(String::from).collect();

    if lc < lines.len() {
        lines[lc].push_str(extra);
    }

    lines.join("\n")
}

fn push_to_github(title: &str) -> std::io::Result<()> {
    let home = env::var("HOME").expect("Couldn't find home directory");

    let msg = format!("add '{}' to blog", title);
    let path = format!("{home}/Developer/me/");
    env::set_current_dir(Path::new(&path)).expect("Failed to enter directory");

    let status = Command::new("git").args(["add", &path]).status()?;
    if !status.success() {
        eprintln!("git add failed");
        return Ok(());
    }

    let status = Command::new("git").args(["commit", "-m", &msg]).status()?;
    if !status.success() {
        eprintln!("git commit failed");
        return Ok(());
    }

    let status = Command::new("git").args(["push"]).status()?;
    if !status.success() {
        eprintln!("git push failed");
        return Ok(());
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = input()?;

    move_file(&data.url)?;
    write_to_file(&data)?;

    push_to_github(&data.title)?;
    println!("Pushed '{}' to GitHub", &data.title);
    Ok(())
}
