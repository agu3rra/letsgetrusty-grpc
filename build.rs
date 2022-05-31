// Build script for Cargo and will configure tonic build
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/payments.proto")?;
    Ok(())
}
