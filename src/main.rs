use comrak::{format_html, parse_document, Arena, ComrakOptions};
use yew::prelude::*;

use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter;

mod safehtml;

#[derive(Deserialize)]
struct Metadata {
	title: String,
	tags: Vec<String>,
}

#[function_component]
fn App() -> Html {
	let md_str = include_str!("content/test.md").trim();

	// Get Front Matter
	let document = YamlFrontMatter::parse::<Metadata>(&md_str).unwrap();
	let front_matter = document.metadata;
	let md = document.content;
	let Metadata { title, tags } = front_matter;

	// Render html
	let arena = Arena::new();
	let mut options = ComrakOptions::default();
	options.extension.front_matter_delimiter = Some("---".to_string());
	let root = parse_document(&arena, &md, &options);
	let mut md_html_vec = vec![];
	format_html(root, &ComrakOptions::default(), &mut md_html_vec).unwrap();
	let md_html = String::from_utf8(md_html_vec).unwrap();

	html!(
		<div>
			<h1>{title}</h1>
			<safehtml::SafeHtml html={md_html} />
			<p>{format!("Tags: {:?}", tags)}</p>
		</div>
	)
}

fn main() {
	yew::Renderer::<App>::new().render();
}
