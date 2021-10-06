use clap::Clap;
use anyhow::anyhow;

#[derive(Clap)]
#[derive(Clone)]
pub struct Options {

}

pub async fn exec(_global_opts: &super::super::Options, web_opts: &super::Options, _opts: &Options) -> Result<(), anyhow::Error> {
    use guntamatic_web as gweb;
    
    let daq_data = gweb::load_and_parse_daq_data(web_opts.addr.as_str(), web_opts.key.as_str())
        .await
        .map_err(|err| anyhow!("{}", err))?;
    println!("{:#?}", daq_data);

    Ok(())
}