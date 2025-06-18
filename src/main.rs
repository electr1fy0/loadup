use std::env;
use std::fs;
use std::process;

struct Data {
    contents: String,
    url: String,
    title: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Provide a name for the HTML file.");
        process::exit(1);
    }
    let home = env::var("HOME").expect("Can't read your $HOME environment variable");

    let from = format!("{}/Documents/source.html", home);
    let to = format!("{}/Developer/me/public/{}.html", home, args[1]);
    println!("{to}");

    fs::rename(from, to)?;
    read();
    Ok(())
}

fn read() {
    let home = env::var("HOME").unwrap();
    let path = format!("{}/Developer/me/src/App.jsx", home);
    let contents: String = fs::read_to_string(&path).unwrap();

    let mut len = contents.lines().collect::<Vec<&str>>().len();

    // println!("{}", len);
    for (i, line) in contents.lines().rev().enumerate() {
        if line.contains("BlogItem") {
            len = len - i;
            break;
        }
    }
    let url = "first_brain_is_a_clown";
    let title = url.replace("_", " ");
    let description = "idk";

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

    let ans = append(&contents, len, &extra);
    fs::write(&path, ans);
    println!("{}", contents);
}

fn append(original: &str, lc: usize, extra: &str) -> String {
    let mut lines: Vec<String> = original.lines().map(String::from).collect();
    println!("{lc}");

    if lc < lines.len() {
        lines[lc].push_str(extra);
    }

    lines.join("\n")
}
