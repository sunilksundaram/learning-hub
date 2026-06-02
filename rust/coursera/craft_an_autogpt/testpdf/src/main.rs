use pdf_oxide::PdfDocument;
use docx_rs::*;
use std::fs::File;
use std::io::{Write, Read};

fn get_page_count_pdf() -> Result<usize, pdf_oxide::Error> {
    let doc = PdfDocument::open("RevA-AI_Review_4.pdf")?;
    let count = doc.page_count()?;
    Ok(count)
}

fn extract_word_headings() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("RevA-AI_Review_4.docx").map_err(|_| DocxError::Unknown)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    // Parse the DOCX file
    let docx = read_docx(&buf)?;

    for child in &docx.document.children {
        if let DocumentChild::Paragraph(para) = child {
            // Check the style of the paragraph
            if let Some(style_id) = &para.property.style {
                let id = &style_id.val;
                
                // Usually "Heading1", "Heading2", etc.
                if id.to_lowercase().contains("heading") {
                    let text = para.children.iter()
                        .filter_map(|p_child| {
                            if let ParagraphChild::Run(run) = p_child {
                                Some(run.children.iter().filter_map(|r_child| {
                                    if let RunChild::Text(t) = r_child {
                                        Some(t.text.clone())
                                    } else { None }
                                }).collect::<String>())
                            } else { None }
                        })
                        .collect::<String>();

                    println!("{}: {}", id, text);
                }
            }
        }
    }
    Ok(())
}

fn get_page_heirarchy_pdf() -> std::io::Result<()>{
    let doc = PdfDocument::open("RevA-AI_Review_4.pdf").expect("Unable to open the document");

    match doc.extract_hierarchical_content(2) {
        Ok(Some(root)) => {
            let mut file = File::create("hierarchy_output.txt")?;

            println!("Structure type: {:#?}", root.structure_type);
            println!("Childern: {:#?}", &root.children);

            writeln!(file, "Structure type: {:#?}", root.structure_type)?;
            writeln!(file, "Childern: {:#?}", &root.children)?;
        }
        Ok(None) => println!("No hierarchical content found."),
        Err(e) => eprintln!("Error extracting hierarchy: {:?}", e),
    }

    Ok(())
}

fn main() {
    match get_page_count_pdf() {
        Ok(count) => println!("Page count = {:#?}", count),
        Err(e) => println!("Failed to open the document. Error: {:#?}", e),
    }
    //get_page_heirarchy_pdf();
    extract_word_headings();
}
