fn main() -> Result<(), Box<dyn std::error::Error>> {
    handle("https://www.rijksoverheid.nl/onderwerpen/grondwet-en-statuut/vraag-en-antwoord/wanneer-kan-ik-de-vlag-uithangen-en-wat-is-de-vlaginstructie", "article", "data/rijksoverheid.md");

    Ok(())

}

fn handle(url: &str, class_id: &str, filename: &str) {
    let contents = scrape_page(url, class_id).unwrap();

    let text = format!("# Url\n{}\n\n{}", url, contents);
    write_or_compare_file(text, filename);
}

fn scrape_page(url: &str, class_id: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest::blocking::get(url)?
        .text()?;

    let document = select::document::Document::from(resp.as_str());

    let content = document.find( select::predicate::Class(class_id)).next().unwrap();

    let content_text = content.html();

    let markdown = html2md::parse_html(&content_text);

    Ok(markdown)
}

fn write_or_compare_file(text: String, filename: &str) {
    let path = std::path::Path::new(&filename);

    if path.exists() {
        let file_content = std::fs::read_to_string(path).unwrap();
        if file_content != text {
            // let changeset = difference::Changeset::new(&file_content, &text, "");
            let changeset = prettydiff::diff_lines(&file_content, &text);
            
            println!("{}", changeset);
            panic!("{} has different content", filename);
        }
    } else {
        std::fs::write(filename, text).unwrap();
    }
}
