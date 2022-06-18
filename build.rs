use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("tests/common")
        .compile(&["./tests/test.proto"], &["./"])?;
    Ok(())
}
