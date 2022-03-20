use eva_common::prelude::*;
use std::io;
use std::io::prelude::*;

fn main() -> EResult<()> {
    let i = io::stdin();
    let mut o = std::io::stdout();
    let mut stdin = i.lock();
    let mut buf = Vec::new();
    stdin.read_to_end(&mut buf)?;
    let value: Value = serde_yaml::from_slice(&buf).map_err(Error::invalid_data)?;
    o.write_all(&rmp_serde::to_vec_named(&value).map_err(Error::invalid_data)?)?;
    o.flush()?;
    Ok(())
}
