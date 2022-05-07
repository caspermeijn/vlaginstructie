/* Copyright (C) 2022 Casper Meijn <casper@meijn.net>
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

fn main() -> Result<(), Box<dyn std::error::Error>> {
    handle("https://www.rijksoverheid.nl/onderwerpen/grondwet-en-statuut/vraag-en-antwoord/wanneer-kan-ik-de-vlag-uithangen-en-wat-is-de-vlaginstructie", "article", "data/rijksoverheid.md");
    handle(
        "https://www.koninklijkhuis.nl/onderwerpen/vlaggen-en-vlaginstructie/vlaginstructie",
        "article",
        "data/koninklijkhuis.md",
    );

    Ok(())
}

fn handle(url: &str, class_id: &str, filename: &str) {
    let contents = scrape_page(url, class_id).unwrap();

    let text = format!("# Url\n{}\n\n{}", url, contents);
    write_or_compare_file(text, filename);
}

fn scrape_page(url: &str, class_id: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest::blocking::get(url)?.text()?;

    let document = select::document::Document::from(resp.as_str());

    let content = document
        .find(select::predicate::Class(class_id))
        .next()
        .unwrap();

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
