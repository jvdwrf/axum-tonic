use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src")
        .compile(&["./company_info.proto"], &["./"])?;

    Ok(())
}
