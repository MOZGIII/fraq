use spirv_builder::{MetadataPrintout, SpirvBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !std::env::var("TARGET").unwrap().starts_with("spirv") {
        SpirvBuilder::new(
            std::env::var_os("CARGO_MANIFEST_DIR").unwrap(),
            "spirv-unknown-spv1.3",
        )
        .print_metadata(MetadataPrintout::Full)
        .build()?;
    }
    Ok(())
}
