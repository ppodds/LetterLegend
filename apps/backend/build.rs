use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/proto/player/*.proto"], &["src/proto/"])?;
    prost_build::compile_protos(&["src/proto/control/*.proto"], &["src/proto/"])?;
    prost_build::compile_protos(&["src/proto/lobby/*.proto"], &["src/proto/"])?;
    Ok(())
}
