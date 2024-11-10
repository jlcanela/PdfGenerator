use interoptopus::patterns::string::AsciiPointer;
use interoptopus::{ffi_function, ffi_type, Inventory, InventoryBuilder, function};
use futures::executor::block_on;

use std::path::{Path, PathBuf};

use typst::{diag::{FileError, FileResult, Warned}, foundations::{Bytes, Datetime}, syntax::{FileId, Source, VirtualPath}, text::{Font, FontBook}, utils::LazyHash, Library};
use typst_kit::fonts::FontSlot;

#[ffi_type]
#[repr(C)]
pub enum FFIError {
    Ok = 0,
    Null = 100,
    Panic = 200,
    Delegate = 300,
    Failed = 400,
}

#[ffi_type]
#[repr(C)]
pub struct ByteBuffer {
    pub ptr: *mut u8,
    pub len: u64,
}

/// Generate PDF from input text and JSON configuration
#[ffi_function]
#[no_mangle]
pub extern "C" fn generate_pdf(
    input: AsciiPointer,
    json_config: AsciiPointer,
    out_buffer: *mut ByteBuffer
) -> FFIError {
    // Convert input string
    let input_str = match input.as_str() {
        Ok(s) => s,
        Err(_) => {
            return FFIError::Failed;
        }
    };

    // Convert JSON string
    let json_str = match json_config.as_str() {
        Ok(s) => s,
        Err(_) => "{}"
    };

    if out_buffer.is_null() {
        return FFIError::Failed;
    }

    match block_on(do_generate_pdf(input_str, json_str)) {
        Ok(buffer) => {
            // Create heap-allocated buffer
            let buffer = buffer.into_boxed_slice();
            let len = buffer.len() as u64;
            let ptr = Box::into_raw(buffer) as *mut u8;
            
            // Write to the output buffer
            unsafe {
                (*out_buffer) = ByteBuffer {
                    ptr,
                    len,
                };
            }
            
            FFIError::Ok
        },
        Err(()) => FFIError::Failed
    }
            
}

/// Free memory allocated by generate_pdf
#[ffi_function]
#[no_mangle]
pub extern "C" fn free_binary_data(data: ByteBuffer) {
    if !data.ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(std::slice::from_raw_parts_mut(data.ptr, data.len as usize));
        }
    }
}



pub async fn do_generate_pdf(input: &str, json: &str) -> Result<Vec<u8>, ()> {
    let fonts = typst_kit::fonts::FontSearcher::new().include_embedded_fonts(true).search();

    let world = SimpleWorld {
        source: input.to_string(),
        json: json.to_string(),
        library: typst::utils::LazyHash::new(typst::Library::default()),
        fonts: fonts.fonts,
        font_book: typst::utils::LazyHash::new(fonts.book),
    };

    let Warned { output, warnings } = typst::compile(&world);
    
    if !warnings.is_empty() {
        //web_sys::console::log_1(&format!("Warnings: {:?}", warnings).into());
    }

    match output {
        Ok(document) => {
            let options = typst_pdf::PdfOptions::default();
            let pdf: Result<Vec<u8>, _> = typst_pdf::pdf(&document, &options);
            match pdf {
                Ok(buffer) => Ok(buffer),
                Err(e) => Err(()),
            }
        }
        Err(e) => Err(()),
    }
}

// Include the SimpleWorld struct and its implementation here
// (same as in your original main.rs)

struct SimpleWorld {
    source: String,
    json: String,
    library: LazyHash<Library>,
    fonts: Vec<FontSlot>,
    font_book: LazyHash<FontBook>,
}

impl typst::World for SimpleWorld {

    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.font_book
    }

    fn main(&self) -> FileId {
        let path = Path::new("/main.typst");
        //"/main.typst");
        FileId::new(None, VirtualPath::new(path))
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        match id.vpath().as_rooted_path().to_str() {
            Some(path) if path == "/main.typst" => {
                let source = Source::new(id, self.source.clone());
                Ok(source)
            },
            Some(path) if path == "/data.json" => {
                let source = Source::new(id, self.json.clone());
                Ok(source)
            }
            _ => {
                let err = FileError::NotFound(PathBuf::from(id.vpath().as_rooted_path().as_os_str())); 
                Err(err)
            }
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        match self.source(id) {
            Ok(source) => {
                let bytes = Box::leak(source.text().as_bytes().to_vec().into_boxed_slice());
                Ok(Bytes::from_static(bytes))
            }
            Err(e) => Err(e),
        }
    }

    #[doc = " Try to access the font with the given index in the font book."]
    fn font(&self, index: usize) -> Option<Font> {
        self.fonts[index].get()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        None
    }
}

pub fn my_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(function!(generate_pdf))
        .register(function!(free_binary_data))
        .inventory()
}
