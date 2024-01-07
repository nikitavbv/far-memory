use {
    std::{process::Command, fs::{self, File, read_to_string}, io::{ErrorKind, Cursor}},
    tracing::warn,
    docx_rs::{
        Docx,
        Paragraph,
        Tab,
        LineSpacing,
        Run,
        AbstractNumbering,
        Level,
        Start,
        NumberFormat,
        LevelText,
        LevelJc,
        SpecialIndentType,
        NumberingId,
        AlignmentType,
        Numbering,
        IndentLevel,
        TableOfContents,
        TabLeaderType,
        Table as DocxTable,
        TableRow as DocxTableRow,
        TableCell as DocxTableCell,
        VMergeType,
        TableLayoutType,
        TableAlignmentType,
        TableCellMargins,
        WidthType,
        BreakType,
        Pic,
    },
    thiserror::Error,
    crate::thesis::{
        context::Context,
        content::{Content, Language, AbstractContent},
        components::{
            SectionHeaderComponent,
            ParagraphComponent,
            UnorderedListComponent,
            ImageComponent,
            AbstractSection,
            TaskSection,
            FrontPageSection,
            TopicCardDocument,
            runs_for_text_span,
        },
    },
};

#[derive(Error, Debug)]
pub enum PageCountingError {
    #[error("No pdf converter installed")]
    NoPdfConverterInstalled,
}

#[derive(Debug, Clone)]
pub enum Block {
    SectionHeader(SectionHeaderBlock),
    SubsectionHeader(SubsectionHeaderBlock),
    Paragraph(ParagraphBlock),
    OrderedList(Vec<TextSpan>),
    UnorderedList(Vec<String>),
    Image(ImageBlock),
    Placeholder(Box<Block>, String),
    Multiple(Vec<Block>),
    ReferencesList(Vec<String>),
    TableOfContents,
    AbstractSection(Language, AbstractContent),
    TaskSection,
    FrontPage,
    TopicCard,
    Note(String),
    Table(TableBlock),
    Application(ApplicationBlock),
}

#[derive(Debug, Clone)]
pub struct SectionHeaderBlock {
    title: String,
    has_numbering: bool, // will be added to document as "{section_number} {title}"
    include_in_table_of_contents: bool,
    page_break_before: bool,
}

impl SectionHeaderBlock {
    pub fn without_numbering(title: String) -> Self {
        Self {
            title,
            has_numbering: false,
            include_in_table_of_contents: true,
            page_break_before: true,
        }
    }

    pub fn do_not_include_in_table_of_contents(self) -> Self {
        Self {
            include_in_table_of_contents: false,
            ..self
        }
    }

    pub fn without_page_break_before(self) -> Self {
        Self {
            page_break_before: false,
            ..self
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubsectionHeaderBlock {
    title: String,
    level: u32, // default level is 1
    has_numbering: bool, // will be added to document as "{subsection_number} {title}"
    with_tab: bool,
    center: bool,
    bold: bool,
    line_spacing: i32,
}

impl SubsectionHeaderBlock {
    pub fn new(title: String) -> Self {
        Self {
            title,
            level: 1,
            has_numbering: true,
            with_tab: true,
            center: false,
            bold: false,
            line_spacing: 24 * 15,
        }
    }

    pub fn without_numbering(title: String) -> Self {
        Self {
            title,
            level: 1,
            has_numbering: false,
            with_tab: true,
            center: false,
            bold: false,
            line_spacing: 24 * 15,
        }
    }

    pub fn without_tab(self) -> Self {
        Self {
            with_tab: false,
            ..self
        }
    }

    pub fn center(self) -> Self {
        Self {
            center: true,
            ..self
        }
    }

    pub fn bold(self) -> Self {
        Self {
            bold: true,
            ..self
        }
    }

    pub fn with_line_spacing(self, font_size: usize, interval: f32) -> Self {
        Self {
            line_spacing: (font_size as f32 * interval * 10.0) as i32,
            ..self
        }
    }

    pub fn with_level(self, level: u32) -> Self {
        Self {
            level,
            ..self
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParagraphBlock {
    span: TextSpan,
    tab: bool,
    line_spacing: i32,
    before_spacing: Option<u32>,
    after_spacing: Option<u32>,
    columns: Option<usize>,
    alignment: Option<Alignment>,
}

impl ParagraphBlock {
    pub fn new(span: TextSpan) -> Self {
        Self {
            span,
            tab: true,
            line_spacing: 24 * 15,
            before_spacing: None,
            after_spacing: None,
            columns: None,
            alignment: None,
        }
    }

    pub fn with_tab(self, tab: bool) -> Self {
        Self {
            tab,
            ..self
        }
    }

    pub fn with_line_spacing(self, font_size: usize, interval: f32) -> Self {
        Self {
            line_spacing: (font_size as f32 * interval * 10.0) as i32,
            ..self
        }
    }

    pub fn with_before_spacing(self, before_spacing: u32) -> Self {
        Self {
            before_spacing: Some(before_spacing),
            ..self
        }
    }

    pub fn with_after_spacing(self, after_spacing: u32) -> Self {
        Self {
            after_spacing: Some(after_spacing),
            ..self
        }
    }

    pub fn with_columns(self, columns: usize) -> Self {
        Self {
            columns: Some(columns),
            ..self
        }
    }

    pub fn text(&self) -> &TextSpan {
        &self.span
    }

    pub fn with_alignment(self, alignment: Alignment) -> Self {
        Self {
            alignment: Some(alignment),
            ..self
        }
    }
}

#[derive(Debug, Clone)]
pub enum TextSpan {
    Regular(String),
    Bold(Box<TextSpan>),
    Italic(Box<TextSpan>),
    Multiple(Vec<TextSpan>),
    Link {
        text: String,
        url: String,
    },
    Reference(Box<TextSpan>, Reference),
    ApplicationReference(&'static str),
    Break,
    PageBreak,
}

#[derive(Debug, Clone)]
pub enum Reference {
    Publication {
        title: String,
        author: String,
        year: u32,
        published_in: String,
    },
    Website {
        title: String,
        link: String,
    },
}

pub enum ReferenceFormat {
    ConferenceAbstract,
    // ДСТУ 8302:2015
    Thesis,
}

impl Reference {
    pub fn for_publication(title: impl Into<String>, author: impl Into<String>, year: u32, published_in: impl Into<String>) -> Self {
        Self::Publication { title: title.into(), author: author.into(), year, published_in: published_in.into() }
    }

    pub fn for_website(title: impl Into<String>, link: impl Into<String>) -> Self {
        Self::Website { title: title.into(), link: link.into() }
    }

    pub fn text(&self, format: &ReferenceFormat) -> String {
        match self {
            Self::Publication { title, author, year, published_in } => match format {
                ReferenceFormat::ConferenceAbstract => format!("{}/{} // {}. {}", title, author, published_in, year),
                ReferenceFormat::Thesis => format!("{}. {}. {}. {}.", author, title, published_in, year),
            },
            Self::Website { title, link } => match format {
                ReferenceFormat::ConferenceAbstract => format!("{} [Online] Available at: {}", title, link),
                ReferenceFormat::Thesis => format!("{}. URL: {}", title, link),
            },
        }
    }

    pub fn id(&self) -> String {
        match self {
            Self::Publication { title, author, year, published_in } => format!("publication::{}_{}_{}_{}", title, author, year, published_in),
            Self::Website { title, link } => format!("website::{}", link),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImageBlock {
    path: String,
    description: String,
    scaling: f32,
    paper_style: bool,
}

impl ImageBlock {
    pub fn new(path: String, description: String) -> Self {
        Self {
            path,
            description,
            scaling: 1.0,
            paper_style: false,
        }
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn with_scaling(self, scaling: f32) -> Self {
        Self {
            scaling,
            ..self
        }
    }

    pub fn with_paper_style(self) -> Self {
        Self {
            paper_style: true,
            ..self
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableBlock {
    columns: Vec<TableCell>,
    rows: Vec<Vec<TableCell>>,
    description: String,
    split: Option<Vec<u32>>,
}

impl TableBlock {
    pub fn empty() -> Self {
        Self::new(Vec::new(), Vec::new(), "".to_owned())
    }

    pub fn new(columns: Vec<TableCell>, rows: Vec<Vec<TableCell>>, description: String) -> Self {
        Self {
            columns,
            rows,
            description,
            split: None,
        }
    }

    pub fn with_split(self, split: Vec<u32>) -> Self {
        Self {
            split: Some(split),
            ..self
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableCell {
    text: TextSpan,
    merge_continue: bool,
    merge_restart: bool,
    width: Option<usize>,
    columns: Option<usize>,
    alignment: Option<Alignment>,
    font_size: Option<usize>,
}

impl TableCell {
    pub fn new(text: TextSpan) -> Self {
        Self {
            text,
            merge_continue: false,
            merge_restart: false,
            width: None,
            columns: None,
            alignment: None,
            font_size: None,
        }
    }

    pub fn merge_continue(self) -> Self {
        Self {
            merge_continue: true,
            ..self
        }
    }

    pub fn merge_restart(self) -> Self {
        Self {
            merge_restart: true,
            ..self
        }
    }

    pub fn width(self, width: usize) -> Self {
        Self {
            width: Some(width),
            ..self
        }
    }

    pub fn columns(self, columns: usize) -> Self {
        Self {
            columns: Some(columns),
            ..self
        }
    }

    pub fn alignment(self, alignment: Alignment) -> Self {
        Self {
            alignment: Some(alignment),
            ..self
        }
    }

    pub fn font_size(self, font_size: usize) -> Self {
        Self {
            font_size: Some(font_size),
            ..self
        }
    }
}

#[derive(Debug, Clone)]
pub enum Alignment {
    Center,
}

#[derive(Debug, Clone)]
pub struct ApplicationBlock {
    id: &'static str,
    title: String,
    content: ApplicationContent,
}

impl ApplicationBlock {
    pub fn new(id: &'static str, title: String, content: ApplicationContent) -> Self {
        Self {
            id,
            title,
            content,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ApplicationContent {
    None,
    Image(Vec<u8>),
    SourceCode(Vec<&'static str>),
}

impl ApplicationContent {
    pub fn image_from_file(path: &str) -> Self {
        Self::Image(fs::read(path).unwrap())
    }
}

pub fn render_block_to_docx(document: Docx, context: &mut Context, content: &Content, block: Block) -> Docx {
    render_block_to_docx_with_params(document, context, content, None, block)
}

fn render_block_to_docx_with_params(document: Docx, context: &mut Context, content: &Content, placeholder: Option<String>, block: Block) -> Docx {
    assign_index_to_applications(context, &block);

    match block {
        Block::SectionHeader(header) => {
            let text = if header.has_numbering {
                format!("{}   {}", context.next_section_index(), header.title)
            } else {
                header.title.clone()
            };

            match placeholder {
                Some(v) => document.add_section_header_placeholder_component(text, v, header.include_in_table_of_contents, header.page_break_before),
                None => document.add_section_header_component(text, header.include_in_table_of_contents, header.page_break_before),
            }
        },
        Block::SubsectionHeader(header) => {
            let text = if header.has_numbering {
                let subsection_index = context.next_subsection_index(context.last_section_index(), header.level);
                format!("{}.{}   {}", context.last_section_index(), subsection_index.to_string(), header.title)
            } else {
                header.title.clone()
            };

            let paragraph = Paragraph::new();
            let paragraph = if header.with_tab {
                paragraph.add_tab(Tab::new().pos(710))
            } else {
                paragraph
            };
            let paragraph = if header.center {
                paragraph.align(AlignmentType::Center)
            } else {
                paragraph
            };

            let run = Run::new();
            let run = if header.bold {
                run.bold()
            } else {
                run
            };
            let run = if header.with_tab {
                run.add_tab()
            } else {
                run
            };

            document.add_paragraph(
                paragraph
                    .line_spacing(LineSpacing::new().before(300).line(header.line_spacing))
                    .style("Heading2")
                    .add_run(run.add_text(text))
            )
        },
        Block::Paragraph(paragraph) => match placeholder {
            Some(v) => document.add_paragraph_placeholder_component(paragraph.span, v),
            None => document.add_paragraph_component(context, paragraph.span, paragraph.tab, paragraph.line_spacing, paragraph.before_spacing, paragraph.after_spacing, paragraph.columns, paragraph.alignment),
        },
        Block::OrderedList(list) => {
            let numbering = context.next_numbering_id();

            let mut document = document
                .add_abstract_numbering(
                    AbstractNumbering::new(numbering)
                        .add_level(Level::new(
                            0,
                            Start::new(1),
                            NumberFormat::new("decimal"),
                            LevelText::new("%1. "),
                            LevelJc::new("start")
                        )
                    )
                )
                .add_numbering(Numbering::new(numbering, numbering));


            for i in 0..list.len() {
                let text = list.get(i).unwrap().clone();
                let text = if !text.to_plaintext().ends_with("?") && !text.to_plaintext().ends_with(".") {
                    TextSpan::Multiple(vec![text, if i == list.len() - 1 { "." } else { ";" }.into()])
                } else {
                    text
                };

                let runs = runs_for_text_span(context, text, Run::new());

                let mut paragraph = Paragraph::new()
                    .line_spacing(LineSpacing::new().line((24.0 * 1.15 * 10.0) as i32))
                    .numbering(NumberingId::new(numbering), IndentLevel::new(0))
                    .align(AlignmentType::Both);

                for run in runs {
                    paragraph = paragraph.add_run(run);
                }

                document = document.add_paragraph(paragraph);
            }

            document
        },
        Block::UnorderedList(list) => document.add_unordered_list_component(context, list),
        Block::Image(image) => document.add_image_component(context, context.last_section_index(), &image.path(), &image.description(), image.scaling, image.paper_style),
        Block::Placeholder(inner, description) => render_block_to_docx_with_params(document, context, content, Some(description), *inner),
        Block::Multiple(blocks) => blocks.into_iter().fold(document, |doc, block| render_block_to_docx_with_params(doc, context, content, placeholder.clone(), block)),
        Block::ReferencesList(references) => {
            let numbering = context.next_numbering_id();

            let document = document
                .add_abstract_numbering(
                    AbstractNumbering::new(numbering)
                        .add_level(Level::new(
                            0,
                            Start::new(1),
                            NumberFormat::new("decimal"),
                            LevelText::new("%1. "),
                            LevelJc::new("start")
                        ).indent(None, Some(SpecialIndentType::Hanging(300)), None, None)
                    )
                )
                .add_numbering(Numbering::new(numbering, numbering));

            references.into_iter().fold(document, |document, reference| document.add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().line(24 * 15))
                .numbering(NumberingId::new(numbering), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text(reference))))
        },
        Block::TableOfContents => document.add_table_of_contents(TableOfContents::new()
            .heading_styles_range(1, 3)
            .tab_leader_type(Some(TabLeaderType::None))
            .auto()
        ),
        Block::AbstractSection(language, abstract_content) => document.add_abstract_section(context, content, &abstract_content, &language),
        Block::TaskSection => document.add_task_section(context, content),
        Block::FrontPage => document.add_front_page_section(content),
        Block::TopicCard => document.add_topic_card_document(context, content),
        Block::Note(_) => panic!("note block is not supported in docx"),
        Block::Table(table) => {
            let mut rows: Vec<_> = table.rows.into_iter()
                .map(|row|
                    DocxTableRow::new(
                        row.into_iter()
                            .map(|cell| {
                                let run = Run::new();

                                let run = if let Some(font_size) = cell.font_size {
                                    run.size(font_size * 2)
                                } else {
                                    run
                                };

                                let paragraph = runs_for_text_span(context, cell.text, run).into_iter()
                                    .fold(Paragraph::new(), |p, r| p.add_run(r));

                                let paragraph = if let Some(alignment) = cell.alignment {
                                    match alignment {
                                        Alignment::Center => paragraph.align(AlignmentType::Center),
                                    }
                                } else {
                                    paragraph
                                };

                                let docx_cell = DocxTableCell::new()
                                    .add_paragraph(paragraph);

                                let docx_cell = if cell.merge_continue {
                                    docx_cell.vertical_merge(VMergeType::Continue)
                                } else if cell.merge_restart {
                                    docx_cell.vertical_merge(VMergeType::Restart)
                                } else {
                                    docx_cell
                                };

                                let docx_cell = if let Some(width) = cell.width {
                                    docx_cell.width(width, WidthType::Dxa)
                                } else {
                                    docx_cell
                                };

                                let docx_cell = if let Some(columns) = cell.columns {
                                    docx_cell.grid_span(columns)
                                } else {
                                    docx_cell
                                };

                                docx_cell
                            }).collect()
                    )
                ).collect();

            rows.insert(0, DocxTableRow::new(table.columns.into_iter().map(|cell| {
                let run =  Run::new().bold();

                let run = if let Some(font_size) = cell.font_size {
                    run.size(font_size * 2)
                } else {
                    run
                };

                let paragraph = runs_for_text_span(context, cell.text, run).into_iter()
                    .fold(Paragraph::new().align(AlignmentType::Center), |p, r| p.add_run(r));

                let docx_cell = DocxTableCell::new()
                    .add_paragraph(paragraph);

                let docx_cell = if cell.merge_continue {
                    docx_cell.vertical_merge(VMergeType::Continue)
                } else if cell.merge_restart {
                    docx_cell.vertical_merge(VMergeType::Restart)
                } else {
                    docx_cell
                };

                let docx_cell = if let Some(width) = cell.width {
                    docx_cell.width(width, WidthType::Dxa)
                } else {
                    docx_cell
                };

                let docx_cell = if let Some(columns) = cell.columns {
                    docx_cell.grid_span(columns)
                } else {
                    docx_cell
                };

                docx_cell
            }).collect()));

            let section_index = context.last_section_index();
            let table_index = context.next_table_index(section_index);

            let mut document = document;
            let mut rows = rows;
            let mut split = table.split.unwrap_or(Vec::new());
            let mut table_first_part = true;

            while !rows.is_empty() {
                let rows_to_add = if !split.is_empty() {
                    rows.drain(0..(split.remove(0) as usize)).collect::<Vec<_>>()
                } else {
                    rows.drain(0..rows.len()).collect::<Vec<_>>()
                };

                let table_title_text = if table_first_part {
                    format!("Таблиця {}.{} - {}.", section_index, table_index, table.description)
                } else {
                    format!("Продовження таблиці {}.{}", section_index, table_index)
                };
                let table_title = Run::new();

                let table_title = if !table_first_part {
                    table_title.add_break(BreakType::Page)
                } else {
                    table_title
                };

                let table_title = table_title.add_tab().add_text(table_title_text);

                document = document
                    .add_paragraph(Paragraph::new()
                        .add_tab(Tab::new().pos(710))
                        .line_spacing(LineSpacing::new().line(24 * 15))
                        .add_run(table_title))
                    .add_table(DocxTable::new(rows_to_add).align(TableAlignmentType::Center).layout(TableLayoutType::Autofit).margins(TableCellMargins::new().margin(80, 80, 80, 80)));

                table_first_part = false;
            }

            document
        },
        Block::Application(application) => {
            let document = document.add_paragraph(
                Paragraph::new()
                    .page_break_before(true)
                    .line_spacing(LineSpacing::new().line(24 * 15))
                    .align(AlignmentType::Center)
                    .add_run(Run::new()
                        .bold()
                        .add_text(format!("Додаток {}", application_letter_for_index(context.index_for_application_id(application.id).unwrap())).to_uppercase())
                        .add_break(BreakType::TextWrapping))
                    .add_run(Run::new().add_text(application.title))
            );

            let document = match application.content {
                ApplicationContent::None => document,
                ApplicationContent::Image(image) => {
                    let mut reader = image::io::Reader::new(Cursor::new(&image));
                    reader.set_format(image::ImageFormat::Jpeg);
                    let img = reader.decode().unwrap();
                    let width = img.width();
                    let height = img.height();

                    let width_emu = 5500000 as u32;
                    let height_emu = ((height as f32) / (width as f32) * (width_emu as f32)) as u32;

                    document.add_paragraph(
                        Paragraph::new()
                            .align(AlignmentType::Center)
                            .add_run(Run::new().add_image(Pic::new(&image).size(width_emu, height_emu)))
                    )
                },
                ApplicationContent::SourceCode(files) => {
                    let mut document = document;

                    for file in files {
                        let file_content = read_to_string(file).unwrap();

                        let font_size = 10 * 2;
                        let mut code_run = Run::new().size(font_size);
                        for line in file_content.lines() {
                            code_run = code_run.add_text(line).add_break(BreakType::TextWrapping);
                        }

                        document = document.add_paragraph(
                            Paragraph::new()
                                .line_spacing(LineSpacing::new().line((font_size * 10) as i32))
                                .add_run(Run::new().size(font_size).add_text(format!("Файл {}:", file)).add_break(BreakType::TextWrapping).add_break(BreakType::TextWrapping))
                                .add_run(code_run)
                        );
                    }

                    document
                }
            };

            document
        },
    }
}

pub fn render_block_to_html(block: Block) -> String {
    format!(
        r#"
        <html>
            <head>
                <meta charset="utf-8" />
                <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/purecss@3.0.0/build/pure-min.css" integrity="sha384-X38yfunGUhNzHpBaEBsWLO+A0HDYOQi8ufWDkZ0k9e0eXz/tH3II7uKZ9msv++Ls" crossorigin="anonymous">
                <link rel="preconnect" href="https://fonts.googleapis.com">
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
                <link href="https://fonts.googleapis.com/css2?family=Open+Sans:wght@400;600&display=swap" rel="stylesheet">
                <script type="text/javascript" async="" src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.4/MathJax.js?config=TeX-MML-AM_CHTML"></script>
                <script type="text/javascript" src="https://livejs.com/live.js"></script>
                <style>
                    body {{
                        max-width: 768px;
                        margin: 0 auto;
                        background-color: #fcfcfc;
                        font-family: 'Open Sans', sans-serif;
                        line-height: 1.6;
                    }}

                    h1, h2 {{
                        font-weight: 400;
                        margin: 1em 0 0 0;
                    }}

                    h2 {{
                        color: #4b4b4b;
                    }}

                    .note {{
                        background: #1f8dd6;
                        border-radius: 3px;
                        color: #fff;
                        padding: 0.3em 1em;
                    }}

                    p {{
                        margin: 0 0 12px 0;
                    }}

                    img {{
                        width: 100%;
                        height: auto;
                    }}

                    .image-description {{
                        width: 100%;
                        text-align: center;
                        font-style: italic;
                        margin-bottom: 12px;
                    }}

                    a {{
                        text-decoration: none;
                        color: inherit;
                    }}

                    p a {{
                        color: #686de0;
                    }}

                    table {{
                        width: 100%;
                    }}
                </style>
            </head>

            <body>{}</body>
        </html>
        "#,
        render_block_to_html_inner(block)
    )
}

fn render_block_to_html_inner(block: Block) -> String {
    match block {
        Block::SectionHeader(header) => {
            let without_whitespaces = header.title.replace(" ", "_");
            let id = html_escape::encode_text(&without_whitespaces);

            format!(
                "<h1 id=\"{}\"><a href=\"#{}\">{}</a></h1>",
                id,
                id,
                html_escape::encode_text(&header.title),
            )
        },
        Block::SubsectionHeader(header) => {
            let without_whitespaces = header.title.replace(" ", "_");
            let id = html_escape::encode_text(&without_whitespaces);

            format!(
                "<h2 id=\"{}\"><a href=\"#{}\">{}</a></h2>",
                id,
                id,
                html_escape::encode_text(&header.title),
            )
        },
        Block::Paragraph(paragraph) => format!("<p>{}</p>", render_text_span_to_html(paragraph.span)),
        Block::UnorderedList(text) => format!("<ul>{}</ul>", text.iter().map(|v| format!("<li>{}</li>", html_escape::encode_text(&v))).collect::<String>()),
        Block::Image(image) => format!("<img src=\"{}\" /><div class=\"image-description\">{}</div>", image.path(), html_escape::encode_text(&image.description())),
        Block::Placeholder(inner, _text) => format!("<div style=\"background-color: yellow;\">{}</div>", render_block_to_html_inner(*inner)),
        Block::Multiple(blocks) => blocks.into_iter().map(render_block_to_html_inner).collect::<String>(),
        Block::Note(text) => format!("<div class=\"note\">{}</div>", html_escape::encode_text(&text)),
        Block::Table(table) => format!(
            "<table class=\"pure-table\"><thead><tr>{}</tr></thead><tbody>{}</tbody></table>",
            render_table_header_to_html(&table.columns),
            render_table_rows_to_html(&table.rows),
        ),
        other => format!("<div>block of this type is not supported: {:?}</div>", other),
    }
}

fn render_table_header_to_html(columns: &[TableCell]) -> String {
    columns
        .iter()
        .map(|v| format!("<th>{}</th>", html_escape::encode_text(&v.text.to_plaintext())))
        .collect()
}

fn render_table_rows_to_html(rows: &[Vec<TableCell>]) -> String {
    rows
        .iter()
        .map(|v| format!("<tr>{}</tr>", render_table_row_to_html(v.as_slice())))
        .collect()
}

fn render_table_row_to_html(row: &[TableCell]) -> String {
    row
        .iter()
        .map(|v| format!("<td>{}</td>", html_escape::encode_text(&v.text.to_plaintext())))
        .collect()
}

fn render_text_span_to_html(span: TextSpan) -> String {
    match span {
        TextSpan::Regular(text) => html_escape::encode_text(&text).to_string(),
        TextSpan::Bold(inner) => format!("<b>{}</b>", render_text_span_to_html(*inner)),
        TextSpan::Italic(inner) => format!("<i>{}</i>", render_text_span_to_html(*inner)),
        TextSpan::Multiple(texts) => texts.into_iter().map(render_text_span_to_html).collect::<String>(),
        TextSpan::Link { text, url } => format!("<a href=\"{}\">{}</a>", url, html_escape::encode_text(&text)),
        TextSpan::Reference(_text, _reference) => unimplemented!(),
        TextSpan::Break | TextSpan::PageBreak => "<br />".to_owned(),
        TextSpan::ApplicationReference(_) => unimplemented!(),
    }
}

pub fn section_header(text: impl Into<SectionHeaderBlock>) -> Block {
    Block::SectionHeader(text.into())
}

pub fn subsection_header(text: impl Into<SubsectionHeaderBlock>) -> Block {
    Block::SubsectionHeader(text.into())
}

pub fn paragraph(text: impl Into<TextSpan>) -> Block {
    Block::Paragraph(ParagraphBlock::new(text.into()))
}

pub fn unordered_list(list: Vec<String>) -> Block {
    Block::UnorderedList(list)
}

pub trait TextBlockComponent {
    fn add_text_block(self, context: &mut Context, content: &Content, block: Block) -> Self;
}

impl TextBlockComponent for Docx {
    fn add_text_block(self, context: &mut Context, content: &Content, block: Block) -> Self {
        render_block_to_docx(self, context, content, block)
    }
}

pub struct Document {
    name: String,
    content: Block,

    docx_template: Docx,
    prepend_pdf: Vec<String>, // path
}

impl Document {
    pub fn new(name: &str, content: Block) -> Self {
        Self {
            name: name.to_owned(),
            content,

            docx_template: Docx::new(),
            prepend_pdf: Vec::new(),
        }
    }

    pub fn with_docx_template(self, docx_template: Docx) -> Self {
        Self {
            docx_template,
            ..self
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn docx(&self) -> Docx {
        self.docx_template.clone()
    }

    pub fn content(&self) -> Block {
        self.content.clone()
    }

    pub fn with_prepend_pdf(self, path: String) -> Self {
        Self {
            prepend_pdf: {
                let mut pdfs = self.prepend_pdf;
                pdfs.push(path);
                pdfs
            },
            ..self
        }
    }

    pub fn prepend_pdf(&self) -> Vec<String> {
        self.prepend_pdf.clone()
    }
}

pub fn print_placeholders(block: &Block) {
    match &block {
        Block::Placeholder(inner, placeholder) => {
            warn!("adding placeholder with description: {:?}", placeholder);
            print_placeholders(&*inner);
        },
        Block::Multiple(multiple) => multiple.iter().for_each(print_placeholders),
        Block::SectionHeader(_) => (),
        Block::SubsectionHeader(_) => (),
        Block::Paragraph(_) => (),
        Block::OrderedList(_) => (),
        Block::UnorderedList(_) => (),
        Block::Image(_) => (),
        Block::ReferencesList(_) => (),
        Block::TableOfContents => (),
        Block::AbstractSection(_, _) => (),
        Block::TaskSection => (),
        Block::FrontPage => (),
        Block::TopicCard => (),
        Block::Note(_) => (),
        Block::Table(_) => (),
        Block::Application(_) => (),
    }
}

pub fn count_pages(docx: Docx, content: &Content, block: &Block) -> Result<u32, PageCountingError> {
    let mut context = Context::new();

    let docx_path = "./output/tmp.docx";
    let pdf_path = "./output/tmp.pdf";

    docx
        .add_text_block(&mut context, content, block.clone())
        .build()
        .pack(File::create(docx_path).unwrap())
        .unwrap();

    if let Err(err) = Command::new("docx2pdf").args([docx_path, pdf_path]).output() {
        if err.kind() == ErrorKind::NotFound {
            return Err(PageCountingError::NoPdfConverterInstalled.into());
        }
    };

    let pdf = lopdf::Document::load(pdf_path).unwrap();
    Ok(pdf.get_pages().len() as u32)
}

pub fn count_images(block: &Block) -> u32 {
    match &block {
        Block::Placeholder(inner, _) => count_images(&*inner),
        Block::Multiple(multiple) => multiple.iter().map(count_images).sum(),
        Block::SectionHeader(_) => 0,
        Block::SubsectionHeader(_) => 0,
        Block::Paragraph(_) => 0,
        Block::OrderedList(_) => 0,
        Block::UnorderedList(_) => 0,
        Block::Image(_) => 1,
        Block::ReferencesList(_) => 0,
        Block::TableOfContents => 0,
        Block::AbstractSection(_, _) => 0,
        Block::TaskSection => 0,
        Block::FrontPage => 0,
        Block::TopicCard => 0,
        Block::Note(_) => 0,
        Block::Table(_) => 0,
        Block::Application(_) => 0,
    }
}

pub fn count_tables(block: &Block) -> u32 {
    match &block {
        Block::Placeholder(inner, _) => count_tables(&*inner),
        Block::Multiple(multiple) => multiple.iter().map(count_tables).sum(),
        Block::SectionHeader(_) => 0,
        Block::SubsectionHeader(_) => 0,
        Block::Paragraph(_) => 0,
        Block::OrderedList(_) => 0,
        Block::UnorderedList(_) => 0,
        Block::Image(_) => 0,
        Block::ReferencesList(_) => 0,
        Block::TableOfContents => 0,
        Block::AbstractSection(_, _) => 0,
        Block::TaskSection => 0,
        Block::FrontPage => 0,
        Block::TopicCard => 0,
        Block::Note(_) => 0,
        Block::Table(_) => 1,
        Block::Application(_) => 0,
    }
}

pub fn count_applications(block: &Block) -> u32 {
    match &block {
        Block::Placeholder(inner, _) => count_applications(&*inner),
        Block::Multiple(multiple) => multiple.iter().map(count_applications).sum(),
        Block::SectionHeader(_) => 0,
        Block::SubsectionHeader(_) => 0,
        Block::Paragraph(_) => 0,
        Block::OrderedList(_) => 0,
        Block::UnorderedList(_) => 0,
        Block::Image(_) => 0,
        Block::ReferencesList(_) => 0,
        Block::TableOfContents => 0,
        Block::AbstractSection(_, _) => 0,
        Block::TaskSection => 0,
        Block::FrontPage => 0,
        Block::TopicCard => 0,
        Block::Note(_) => 0,
        Block::Table(_) => 0,
        Block::Application(_) => 1,
    }
}

pub fn count_references(block: &Block) -> u32 {
    match &block {
        Block::Placeholder(inner, _) => count_references(&*inner),
        Block::Multiple(multiple) => multiple.iter().map(count_references).sum(),
        Block::SectionHeader(_) => 0,
        Block::SubsectionHeader(_) => 0,
        Block::Paragraph(_) => 0,
        Block::OrderedList(_) => 0,
        Block::UnorderedList(_) => 0,
        Block::Image(_) => 0,
        Block::ReferencesList(refs) => refs.len() as u32,
        Block::TableOfContents => 0,
        Block::AbstractSection(_, _) => 0,
        Block::TaskSection => 0,
        Block::FrontPage => 0,
        Block::TopicCard => 0,
        Block::Note(_) => 0,
        Block::Table(_) => 0,
        Block::Application(_) => 0,
    }
}

impl Into<SectionHeaderBlock> for &str {
    fn into(self) -> SectionHeaderBlock {
        SectionHeaderBlock {
            title: self.to_owned(),
            has_numbering: true,
            include_in_table_of_contents: true,
            page_break_before: true,
        }
    }
}

impl Into<SectionHeaderBlock> for String {
    fn into(self) -> SectionHeaderBlock {
        SectionHeaderBlock {
            title: self,
            has_numbering: true,
            include_in_table_of_contents: true,
            page_break_before: true,
        }
    }
}

impl Into<SubsectionHeaderBlock> for &str {
    fn into(self) -> SubsectionHeaderBlock {
        SubsectionHeaderBlock {
            title: self.to_owned(),
            level: 1,
            has_numbering: true,
            with_tab: true,
            center: false,
            bold: false,
            line_spacing: 24 * 15,
        }
    }
}

impl Into<SubsectionHeaderBlock> for String {
    fn into(self) -> SubsectionHeaderBlock {
        SubsectionHeaderBlock {
            title: self,
            level: 1,
            has_numbering: true,
            with_tab: true,
            center: false,
            bold: false,
            line_spacing: 24 * 15,
        }
    }
}

impl TextSpan {
    pub fn to_plaintext(&self) -> String {
        match self {
            TextSpan::Regular(text) => text.to_owned(),
            TextSpan::Bold(inner) => inner.to_plaintext(),
            TextSpan::Italic(inner) => inner.to_plaintext(),
            TextSpan::Multiple(texts) => texts.iter().map(|v| v.to_plaintext()).collect::<String>(),
            TextSpan::Link { text, url: _ } => text.to_owned(),
            TextSpan::Reference(text, _) => text.to_plaintext(),
            TextSpan::Break | TextSpan::PageBreak => "\n".to_owned(),
            TextSpan::ApplicationReference(_) => unimplemented!(),
        }
    }
}

impl Into<TextSpan> for String {
    fn into(self) -> TextSpan {
        TextSpan::Regular(self)
    }
}

impl Into<TextSpan> for &str {
    fn into(self) -> TextSpan {
        TextSpan::Regular(self.to_owned())
    }
}

impl Into<TextSpan> for Vec<TextSpan> {
    fn into(self) -> TextSpan {
        TextSpan::Multiple(self)
    }
}

pub fn bold(text: &str) -> TextSpan {
    TextSpan::Bold(Box::new(TextSpan::Regular(text.to_owned())))
}

pub fn application_letter_for_index(index: u32) -> String {
    vec![
        "А",
        "Б",
        "В",
        "Г",
        "Д",
        "Е",
        "Ж",
        "И",
        "К",
        "Л",
        "М",
        "Н",
        "П",
        "Р",
        "С",
        "Т",
        "У",
        "Ф",
        "Х",
        "Ч",
        "Ш",
        "Щ",
        "Ю",
        "Я",
    ][index as usize].to_owned()
}

fn assign_index_to_applications(context: &mut Context, text: &Block) {
    match text {
        Block::Multiple(inner) => inner.iter().for_each(|v| assign_index_to_applications(context, v)),
        Block::Application(application) => context.add_application(application.id),
        Block::FrontPage => (),
        Block::TaskSection => (),
        Block::AbstractSection(..) => (),
        Block::Paragraph(_) => (),
        Block::SectionHeader(_) => (),
        Block::TableOfContents => (),
        Block::UnorderedList(_) => (),
        Block::SubsectionHeader(_) => (),
        Block::Image(_) => (),
        Block::Placeholder(inner, _) => assign_index_to_applications(context, inner),
        Block::ReferencesList(_) => (),
        Block::OrderedList(_) => (),
        Block::TopicCard => (),
        Block::Note(..) => (),
        Block::Table{..} => (),
    }
}

pub fn empty_block() -> Block {
    Block::Multiple(vec![])
}

pub fn extract_references_text(format: &ReferenceFormat, block: &Block) -> Vec<String> {
    let mut result = Vec::new();
    extract_references_text_inner(&mut result, block, format);
    result
}

fn extract_references_text_inner(references: &mut Vec<String>, block: &Block, format: &ReferenceFormat) {
    match &block {
        Block::Paragraph(paragraph) => extract_references_text_span(references, paragraph.text(), format),
        Block::Multiple(multi) => multi.iter().for_each(|v| extract_references_text_inner(references, v, format)),
        Block::OrderedList(list) => list.iter().for_each(|v| extract_references_text_span(references, v, format)),
        _ => (),
    }
}

fn extract_references_text_span(references: &mut Vec<String>, text: &TextSpan, format: &ReferenceFormat) {
    match &text {
        TextSpan::Regular(_) => (),
        TextSpan::Bold(inner) => extract_references_text_span(references, inner, format),
        TextSpan::Italic(inner) => extract_references_text_span(references, inner, format),
        TextSpan::Multiple(multi) => multi.iter().for_each(|v| extract_references_text_span(references, v, format)),
        TextSpan::Link { .. } => (),
        TextSpan::Reference(inner, reference) => {
            extract_references_text_span(references, inner, format);

            let ref_text = reference.text(format);
            if !references.contains(&ref_text.to_owned()) {
                references.push(ref_text.to_owned());
            }
        },
        TextSpan::Break => (),
        TextSpan::PageBreak => (),
        TextSpan::ApplicationReference(_) => (),
    }
}

pub fn reference(inner_text: impl Into<TextSpan>, reference: Reference) -> TextSpan {
    TextSpan::Reference(Box::new(inner_text.into()), reference)
}
