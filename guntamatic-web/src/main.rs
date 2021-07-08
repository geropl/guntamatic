mod lib;

use anyhow::anyhow;

fn main() -> Result<(), anyhow::Error> {
    smol::block_on(async {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let (addr, key) = match args.as_slice() {
            [addr, key] => Ok((addr, key)),
            _ => Err(anyhow!("expected addr and key parameters")),
        }?;

        lib::load_and_parse(addr, key)
            .await
            .map_err(|err| anyhow!("{}", err))
    })
}
