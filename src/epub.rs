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
        let econtent = EpubContent::new(
            format!("chapter{:?}.html", index),
            c.get_content().as_bytes(),
        )
        .title(c.get_name())
        .reftype(ReferenceType::Cover);
        buider.add_content(econtent)?;
        index += 1;
    }
    // Set the stylesheet (create a "stylesheet.css" file in EPUB that is used by some generated files)
    buider.stylesheet(dummy_css.as_bytes())?;

    // Generate a toc inside of the document, that will be part of the linear structure.
    buider.inline_toc().generate(&mut buffer)?;
    Ok(())
}
