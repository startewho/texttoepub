use epub_builder::EpubBuilder;
use epub_builder::EpubContent;
use epub_builder::ReferenceType;
use epub_builder::Result;
use epub_builder::ZipLibrary;

use crate::meta;
use std::fs::File;

// Try to print Zip file to stdout
pub fn gen_epub<'a>(book: meta::Book<'a>) -> Result<()> {
    // Some dummy content to fill our books

    let dummy_css = "body { background-color: pink }";

    let mut buffer = File::create("foo.epub").unwrap();
    // Create a new EpubBuilder using the zip library
    let mut buider = EpubBuilder::new(ZipLibrary::new().unwrap())?;
    for m in book.get_metas().iter() {
        buider.metadata(&m.key, &m.value)?;
    }
    let mut index = 1;
    for c in book.get_chapter().iter() {
        let start=c.get_start();
        let next_c:usize;
        if index>=book.get_chapter().len() {
             next_c=book.get_source().len();
        }
        else {
            next_c=book.get_chapter()[index].get_start();
        }
        let content=book.get_source()[start..next_c].as_bytes();
        let econtent = EpubContent::new(
            format!("chapter{:?}.html", index),
            content,
        )
        .title(c.get_name());
        buider.add_content(econtent)?;
        index += 1;
    }
    // Set the stylesheet (create a "stylesheet.css" file in EPUB that is used by some generated files)
    buider.stylesheet(dummy_css.as_bytes())?;

    // Generate a toc inside of the document, that will be part of the linear structure.
    buider.inline_toc().generate(&mut buffer)?;
    Ok(())
}
