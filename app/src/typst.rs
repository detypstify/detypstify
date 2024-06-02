// yoinked from https://github.com/tfachmann/typst-as-library

use comemo::Prehashed;
use typst::diag::FileResult;
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source};
use typst::text::{Font, FontBook};
use typst::Library;

/// Main interface that determines the environment for Typst.
pub struct TypstWrapperWorld {
    /// The content of a source.
    source: Source,

    /// The standard library.
    library: Prehashed<Library>,

    /// Metadata about all known fonts.
    book: Prehashed<FontBook>,

    /// Metadata about all known fonts.
    fonts: Vec<Font>,
}

impl TypstWrapperWorld {
    pub fn new(source: String) -> Self {
        let fonts = fonts();

        Self {
            library: Prehashed::new(Library::default()),
            book: Prehashed::new(FontBook::from_fonts(&fonts)),
            fonts,
            source: Source::detached(source),
        }
    }
}

/// This is the interface we have to implement such that `typst` can compile it.
///
/// I have tried to keep it as minimal as possible
impl typst::World for TypstWrapperWorld {
    /// Standard library.
    fn library(&self) -> &Prehashed<Library> {
        &self.library
    }

    /// Metadata about all known Books.
    fn book(&self) -> &Prehashed<FontBook> {
        &self.book
    }

    /// Accessing the main source file.
    fn main(&self) -> Source {
        self.source.clone()
    }

    /// Accessing a specified source file (based on `FileId`).
    fn source(&self, _id: FileId) -> FileResult<Source> {
        unimplemented!("Not supported")
    }

    /// Accessing a specified file (non-file).
    fn file(&self, _id: FileId) -> FileResult<Bytes> {
        unimplemented!("Not supported")
    }

    /// Accessing a specified font per index of font book.
    fn font(&self, id: usize) -> Option<Font> {
        self.fonts.get(id).cloned()
    }

    /// Get the current date.
    ///
    /// Optionally, an offset in hours is given.
    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        None
    }
}

/// Helper function
fn fonts() -> Vec<Font> {
    typst_assets::fonts()
        .flat_map(|entry| {
            let buffer = Bytes::from(entry);
            let face_count = ttf_parser::fonts_in_collection(&buffer).unwrap_or(1);
            (0..face_count).map(move |face| {
                Font::new(buffer.clone(), face).unwrap_or_else(|| {
                    panic!("failed to load font from (face index {face})")
                })
            })
        })
        .collect()
}
