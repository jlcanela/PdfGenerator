use interoptopus::util::NamespaceMappings;
use interoptopus::{Error, Interop};

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};

    Generator::new(
        Config {
            class: "Pdf".to_string(),
            dll_name: "libpdf_generator".to_string(),
            namespace_mappings: NamespaceMappings::new("PdfGenerator"),
            ..Config::default()
        },
        pdf_generator::my_inventory(),
    )
    .write_file("../PdfApp/PdfGenerator.cs")?;

    Ok(())
}
