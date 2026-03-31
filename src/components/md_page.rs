use leptos::prelude::*;

use crate::components::{
    highlight_code, ApiPart, ApiPartDataAttrs, ApiPartProps, ComponentPageHeader, DataAttr,
    DemoTabs, DocSection, Prop,
};

/// A demo entry: the live component (as a ViewFn for Send+Sync) and its source code.
pub struct DemoEntry {
    pub view: ViewFn,
    pub source: String,
}

// ---------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------

struct Frontmatter<'a> {
    title: &'a str,
    description: &'a str,
    features: Vec<&'a str>,
}

#[derive(Clone)]
enum Inline<'a> {
    Text(&'a str),
    Code(&'a str),
    ComponentRef(&'a str),
    Bold(Vec<Inline<'a>>),
}

enum Block<'a> {
    Paragraph(Vec<Inline<'a>>),
    CodeFence { lang: &'a str, code: &'a str },
    OrderedList(Vec<Vec<Inline<'a>>>),
    Demo(&'a str),
    Heading3(&'a str),
}

struct PropDef<'a> {
    name: &'a str,
    r#type: &'a str,
    default: &'a str,
    description: &'a str,
}

struct DataAttrDef<'a> {
    name: &'a str,
    description: &'a str,
}

struct ApiPartDef<'a> {
    name: &'a str,
    description: &'a str,
    renders: &'a str,
    props: Vec<PropDef<'a>>,
    data_attrs: Vec<DataAttrDef<'a>>,
}

enum Section<'a> {
    General {
        title: &'a str,
        blocks: Vec<Block<'a>>,
    },
    ApiReference {
        preamble: Vec<Block<'a>>,
        parts: Vec<ApiPartDef<'a>>,
    },
}

struct PageData<'a> {
    frontmatter: Frontmatter<'a>,
    sections: Vec<Section<'a>>,
}

// ---------------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------------

fn parse_frontmatter(input: &str) -> (Frontmatter<'_>, &str) {
    let rest = input.strip_prefix("---\n").unwrap_or(input);
    let end = rest.find("\n---\n").unwrap_or(0);
    let yaml = &rest[..end];
    let after = &rest[end + 5..]; // skip "\n---\n"

    let mut title = "";
    let mut description = "";
    let mut features = Vec::new();

    for line in yaml.lines() {
        if let Some(v) = line.strip_prefix("title: ") {
            title = v.trim();
        } else if let Some(v) = line.strip_prefix("description: ") {
            description = v.trim();
        } else if let Some(v) = line.strip_prefix("  - ") {
            features.push(v.trim());
        }
    }

    (Frontmatter { title, description, features }, after)
}

fn split_h2_sections(body: &str) -> Vec<(&str, &str)> {
    let mut sections = Vec::new();
    let mut rest = body;

    // Skip the # Title line (h1)
    if let Some(idx) = rest.find("\n## ") {
        rest = &rest[idx + 1..];
    } else {
        return sections;
    }

    while let Some(start) = rest.strip_prefix("## ") {
        let title_end = start.find('\n').unwrap_or(start.len());
        let title = &start[..title_end];
        let after_title = if title_end < start.len() {
            &start[title_end + 1..]
        } else {
            ""
        };

        let next_h2 = after_title.find("\n## ");
        let body = match next_h2 {
            Some(idx) => {
                rest = &after_title[idx + 1..];
                &after_title[..idx]
            }
            None => {
                rest = "";
                after_title
            }
        };

        sections.push((title, body.trim()));
    }

    sections
}

fn parse_blocks<'a>(body: &'a str) -> Vec<Block<'a>> {
    let mut blocks = Vec::new();
    let mut para_lines: Vec<&str> = Vec::new();
    let mut in_fence = false;
    let mut fence_lang = "";
    let mut fence_start = 0;

    let lines: Vec<&str> = body.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];

        if line.starts_with("```") {
            if !in_fence {
                flush_paragraph(&mut para_lines, &mut blocks);
                fence_lang = line[3..].trim();
                // Find the byte offset for the code content
                let line_start = byte_offset(body, line);
                fence_start = line_start + line.len() + 1; // skip this line + newline
                in_fence = true;
            } else {
                let line_start = byte_offset(body, line);
                let code = body[fence_start..line_start].trim_end_matches('\n');
                blocks.push(Block::CodeFence { lang: fence_lang, code });
                in_fence = false;
            }
            i += 1;
            continue;
        }

        if in_fence {
            i += 1;
            continue;
        }

        if line.starts_with("<!-- demo: ") {
            flush_paragraph(&mut para_lines, &mut blocks);
            let name = line
                .strip_prefix("<!-- demo: ")
                .and_then(|s| s.strip_suffix(" -->"))
                .unwrap_or("")
                .trim();
            blocks.push(Block::Demo(name));
            i += 1;
            continue;
        }

        if line.starts_with("### ") {
            flush_paragraph(&mut para_lines, &mut blocks);
            blocks.push(Block::Heading3(&line[4..]));
            i += 1;
            continue;
        }

        if line.starts_with("1. ") || (line.len() > 3 && line.as_bytes()[0].is_ascii_digit() && line[1..].starts_with(". ")) {
            flush_paragraph(&mut para_lines, &mut blocks);
            let mut items = Vec::new();
            let mut j = i;
            while j < lines.len() {
                let l = lines[j];
                if let Some(rest) = strip_list_prefix(l) {
                    items.push(parse_inline(rest));
                    j += 1;
                } else {
                    break;
                }
            }
            blocks.push(Block::OrderedList(items));
            i = j;
            continue;
        }

        if line.is_empty() {
            flush_paragraph(&mut para_lines, &mut blocks);
        } else {
            para_lines.push(line);
        }

        i += 1;
    }

    flush_paragraph(&mut para_lines, &mut blocks);
    blocks
}

fn strip_list_prefix(line: &str) -> Option<&str> {
    let bytes = line.as_bytes();
    if bytes.is_empty() || !bytes[0].is_ascii_digit() {
        return None;
    }
    line.find(". ").map(|idx| &line[idx + 2..])
}

fn flush_paragraph<'a>(lines: &mut Vec<&'a str>, blocks: &mut Vec<Block<'a>>) {
    if lines.is_empty() {
        return;
    }
    let text = if lines.len() == 1 {
        lines[0]
    } else {
        // Multi-line paragraph: join with spaces. Requires allocation + leak.
        let joined = lines.join(" ");
        Box::leak(joined.into_boxed_str())
    };
    blocks.push(Block::Paragraph(parse_inline(text)));
    lines.clear();
}

fn byte_offset(haystack: &str, needle: &str) -> usize {
    needle.as_ptr() as usize - haystack.as_ptr() as usize
}

// ---------------------------------------------------------------------------
// API Reference parser
// ---------------------------------------------------------------------------

fn parse_api_reference<'a>(body: &'a str) -> (Vec<Block<'a>>, Vec<ApiPartDef<'a>>) {
    let first_h3 = body.find("\n### ");
    let preamble_text = match first_h3 {
        Some(idx) => &body[..idx],
        None => body,
    };
    let preamble = parse_blocks(preamble_text);

    let mut parts = Vec::new();
    let parts_text = match first_h3 {
        Some(idx) => &body[idx + 1..],
        None => return (preamble, parts),
    };

    // Split on ### headings
    let mut remaining = parts_text;
    while let Some(rest) = remaining.strip_prefix("### ") {
        let title_end = rest.find('\n').unwrap_or(rest.len());
        let name = &rest[..title_end];
        let after_title = if title_end < rest.len() { &rest[title_end + 1..] } else { "" };

        let next_h3 = after_title.find("\n### ");
        let part_body = match next_h3 {
            Some(idx) => {
                remaining = &after_title[idx + 1..];
                &after_title[..idx]
            }
            None => {
                remaining = "";
                after_title
            }
        };

        parts.push(parse_api_part(name.trim(), part_body.trim()));
    }

    (preamble, parts)
}

fn parse_api_part<'a>(name: &'a str, body: &'a str) -> ApiPartDef<'a> {
    let mut description_lines: Vec<&str> = Vec::new();
    let mut renders = "";
    let mut props = Vec::new();
    let mut data_attrs = Vec::new();
    let mut in_table = false;
    let mut table_cols = 0;
    let mut rows_in_table = 0;

    for line in body.lines() {
        if line.starts_with('|') {
            if !in_table {
                in_table = true;
                table_cols = line.matches('|').count() - 1;
                rows_in_table = 0;
                continue; // skip header row
            }
            rows_in_table += 1;
            if rows_in_table == 1 && line.contains("---") {
                continue; // skip separator row
            }
            let cells: Vec<&str> = line
                .trim_matches('|')
                .split('|')
                .map(|c| c.trim())
                .collect();

            if table_cols >= 4 && cells.len() >= 4 {
                props.push(PropDef {
                    name: cells[0],
                    r#type: strip_backticks(cells[1]),
                    default: strip_backticks_or_dash(cells[2]),
                    description: cells[3],
                });
            } else if table_cols >= 2 && cells.len() >= 2 {
                data_attrs.push(DataAttrDef {
                    name: cells[0],
                    description: cells[1],
                });
            }
        } else {
            in_table = false;
            rows_in_table = 0;

            if line.is_empty() {
                continue;
            }
            if line.starts_with("Renders a") || line.starts_with("Renders an") {
                if let Some(tag) = extract_renders_tag(line) {
                    renders = tag;
                }
            } else {
                description_lines.push(line);
            }
        }
    }

    let description = if description_lines.len() == 1 {
        description_lines[0]
    } else if description_lines.is_empty() {
        ""
    } else {
        let joined = description_lines.join(" ");
        Box::leak(joined.into_boxed_str())
    };

    ApiPartDef { name, description, renders, props, data_attrs }
}

fn strip_backticks(s: &str) -> &str {
    s.strip_prefix('`').and_then(|s| s.strip_suffix('`')).unwrap_or(s)
}

fn strip_backticks_or_dash(s: &str) -> &str {
    let stripped = strip_backticks(s);
    if stripped == "\u{2014}" || stripped == "-" {
        "\u{2014}"
    } else {
        stripped
    }
}

fn extract_renders_tag<'a>(line: &'a str) -> Option<&'a str> {
    let start = line.find('`')? + 1;
    let end = start + line[start..].find('`')?;
    Some(&line[start..end])
}

// ---------------------------------------------------------------------------
// Inline parser
// ---------------------------------------------------------------------------

fn parse_inline(text: &str) -> Vec<Inline<'_>> {
    let mut result = Vec::new();
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut pos = 0;
    let mut text_start = 0;

    while pos < len {
        match bytes[pos] {
            b'`' => {
                // Emit preceding text
                if pos > text_start {
                    result.push(Inline::Text(&text[text_start..pos]));
                }

                if pos + 1 < len && bytes[pos + 1] == b'~' {
                    // ComponentRef: `~Name~`
                    let start = pos + 2;
                    if let Some(end) = find_pattern(text, start, "~`") {
                        result.push(Inline::ComponentRef(&text[start..end]));
                        pos = end + 2;
                    } else {
                        result.push(Inline::Text(&text[pos..pos + 1]));
                        pos += 1;
                    }
                } else {
                    // Regular code: `code`
                    let start = pos + 1;
                    if let Some(end) = memchr(b'`', &bytes[start..]) {
                        let end = start + end;
                        result.push(Inline::Code(&text[start..end]));
                        pos = end + 1;
                    } else {
                        result.push(Inline::Text(&text[pos..pos + 1]));
                        pos += 1;
                    }
                }
                text_start = pos;
            }
            b'*' if pos + 1 < len && bytes[pos + 1] == b'*' => {
                if pos > text_start {
                    result.push(Inline::Text(&text[text_start..pos]));
                }
                let start = pos + 2;
                if let Some(end) = find_pattern(text, start, "**") {
                    let inner = parse_inline(&text[start..end]);
                    result.push(Inline::Bold(inner));
                    pos = end + 2;
                } else {
                    result.push(Inline::Text(&text[pos..pos + 2]));
                    pos += 2;
                }
                text_start = pos;
            }
            _ => {
                pos += 1;
            }
        }
    }

    if text_start < len {
        result.push(Inline::Text(&text[text_start..]));
    }

    result
}

fn find_pattern(text: &str, start: usize, pattern: &str) -> Option<usize> {
    text[start..].find(pattern).map(|i| start + i)
}

fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == needle)
}

// ---------------------------------------------------------------------------
// Top-level parse
// ---------------------------------------------------------------------------

fn parse_page(source: &str) -> PageData<'_> {
    let (frontmatter, body) = parse_frontmatter(source);
    let raw_sections = split_h2_sections(body);

    let sections = raw_sections
        .into_iter()
        .map(|(title, body)| {
            if title == "API Reference" {
                let (preamble, parts) = parse_api_reference(body);
                Section::ApiReference { preamble, parts }
            } else {
                Section::General {
                    title,
                    blocks: parse_blocks(body),
                }
            }
        })
        .collect();

    PageData { frontmatter, sections }
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

const INLINE_CODE: &str = "rounded bg-slate-100 px-1 py-0.5 text-sm text-slate-700";
const INLINE_COMPONENT: &str = "rounded bg-slate-100 px-1 py-0.5 text-sm text-accent-700";

pub fn render_md_page(
    source: &'static str,
    render_demo: impl Fn(&str) -> Option<DemoEntry> + Copy + Send + Sync + 'static,
) -> impl IntoView {
    let page = parse_page(source);
    let features: &'static [&'static str] = page.frontmatter.features.leak();

    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title=page.frontmatter.title
                description=page.frontmatter.description
                features=features
            />
            {page.sections.into_iter().map(move |s| render_section(s, render_demo)).collect_view()}
        </div>
    }
}

fn render_section(section: Section<'static>, render_demo: impl Fn(&str) -> Option<DemoEntry> + Copy + Send + Sync + 'static) -> AnyView {
    match section {
        Section::General { title, blocks } => view! {
            <DocSection title=title>
                {blocks.into_iter().map(move |b| render_block(b, render_demo)).collect_view()}
            </DocSection>
        }
        .into_any(),
        Section::ApiReference { preamble, parts } => view! {
            <DocSection title="API Reference">
                {preamble.into_iter().map(move |b| render_block(b, render_demo)).collect_view()}
                {parts.into_iter().map(render_api_part).collect_view()}
            </DocSection>
        }
        .into_any(),
    }
}

fn render_block(block: Block<'static>, render_demo: impl Fn(&str) -> Option<DemoEntry> + Copy + Send + Sync + 'static) -> AnyView {
    match block {
        Block::Paragraph(inlines) => view! {
            <p class="mb-4 text-slate-600">{render_inlines(inlines)}</p>
        }
        .into_any(),
        Block::CodeFence { lang, code } => render_highlighted_code(lang, code).into_any(),
        Block::OrderedList(items) => view! {
            <ol class="mb-4 list-decimal pl-6 space-y-2 text-slate-600">
                {items.into_iter().map(|item| view! {
                    <li>{render_inlines(item)}</li>
                }).collect_view()}
            </ol>
        }
        .into_any(),
        Block::Demo(name) => {
            if let Some(entry) = render_demo(name) {
                let stored_view = StoredValue::new(entry.view);
                view! {
                    <DemoTabs source=entry.source>
                        {move || stored_view.with_value(|f| f.run())}
                    </DemoTabs>
                }
                .into_any()
            } else {
                view! { <p class="text-red-500">{format!("Unknown demo: {name}")}</p> }.into_any()
            }
        }
        Block::Heading3(text) => view! {
            <h3 class="mt-10 mb-2 text-lg font-semibold text-slate-900">{text}</h3>
        }
        .into_any(),
    }
}

fn render_highlighted_code(lang: &'static str, code: &'static str) -> impl IntoView {
    let code_ref = NodeRef::<leptos::html::Code>::new();
    let class = if lang.is_empty() {
        "font-mono".to_string()
    } else {
        format!("language-{lang} font-mono")
    };

    Effect::new(move |_| {
        if let Some(el) = code_ref.get() {
            highlight_code(&el);
        }
    });

    view! {
        <div class="relative mb-4">
            <crate::components::CopyButton text=code.to_string() />
            <pre class="overflow-x-auto rounded-lg bg-code p-4 pr-10 text-xs leading-relaxed">
                <code node_ref=code_ref class=class>
                    {code}
                </code>
            </pre>
        </div>
    }
}

fn render_inlines(inlines: Vec<Inline<'static>>) -> Vec<AnyView> {
    inlines.into_iter().map(render_inline).collect()
}

fn render_inline(inline: Inline<'static>) -> AnyView {
    match inline {
        Inline::Text(t) => t.into_any(),
        Inline::Code(c) => view! {
            <code class=INLINE_CODE>{c}</code>
        }
        .into_any(),
        Inline::ComponentRef(c) => view! {
            <code class=INLINE_COMPONENT>{c}</code>
        }
        .into_any(),
        Inline::Bold(children) => view! {
            <strong>{render_inlines(children)}</strong>
        }
        .into_any(),
    }
}

fn render_api_part(part: ApiPartDef<'static>) -> AnyView {
    // Store raw prop data (all &'static str, so Send+Sync) for ChildrenFn compatibility
    let prop_data: Vec<(&'static str, &'static str, &'static str, &'static str)> = part
        .props
        .into_iter()
        .map(|p| (p.name, p.r#type, p.default, p.description))
        .collect();

    let attr_data: Vec<(&'static str, &'static str)> = part
        .data_attrs
        .into_iter()
        .map(|a| (a.name, a.description))
        .collect();

    let props_view = if prop_data.is_empty() {
        None
    } else {
        let stored = StoredValue::new(prop_data);
        Some(
            view! {
                <ApiPartProps>
                    {move || stored.with_value(|defs| {
                        defs.iter().map(|(name, ty, default, desc)| {
                            view! { <Prop name=*name r#type=*ty default=*default description=*desc /> }.into_any()
                        }).collect_view()
                    })}
                </ApiPartProps>
            }
            .into_any(),
        )
    };

    let attrs_view = if attr_data.is_empty() {
        None
    } else {
        Some(
            view! {
                <ApiPartDataAttrs>
                    {attr_data.into_iter().map(|(name, desc)| {
                        view! { <DataAttr name=name description=desc /> }.into_any()
                    }).collect_view()}
                </ApiPartDataAttrs>
            }
            .into_any(),
        )
    };

    view! {
        <ApiPart name=part.name description=part.description renders=part.renders>
            {props_view}
            {attrs_view}
        </ApiPart>
    }
    .into_any()
}
