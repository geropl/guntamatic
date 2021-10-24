use std::time::Duration;

use clap::Clap;

#[cfg(feature = "sink_prometheus")]
mod sink_prometheus;
#[cfg(feature = "sink_influxdb")]
mod sink_influxdb;

#[derive(Clap)]
#[derive(Clone)]
pub struct Options {
    /// The interval in which to poll the device for data [seconds]
    #[clap(
        long = "interval",
        short = 'i',
        default_value = "30",
        parse(try_from_str = super::super::parse_duration)
    )]
    pub interval: Duration,

    /// The sink to write DAQ data to
    #[clap(subcommand)]
    pub sink: Sink,
}

#[derive(Clap)]
#[derive(Clone)]
pub enum Sink {
    // #[cfg(feature = "sink_prometheus")]
    // #[clap(
    //     name = "prometheus",
    //     about = "Serve measurements as prometheus metrics"
    // )]
    // Prometheus(sink_prometheus::Options),
    #[cfg(feature = "sink_influxdb")]
    #[clap(
        name = "influxdb",
        about = "Push parsed DAQ data into the configured InfluxDB"
    )]
    InfluxDB(sink_influxdb::Options),
}

pub async fn exec(_global_opts: &super::super::Options, web_opts: &super::Options, opts: &Options) -> Result<(), anyhow::Error> {
    use guntamatic_web as gweb;

    let sink = opts.sink.clone();
    let web_opts = web_opts.clone();
    let opts = opts.clone();
    
    let (tx, rc) = flume::unbounded::<gweb::DaqData>();
    let _listener = tokio::spawn(async move {
        loop {
            info!("retrieving DAQ data...");
            let daq_data = gweb::load_and_parse_daq_data(web_opts.addr.as_str(), web_opts.key.as_str())
                .await;
            match daq_data {
                Err(err) => error!("error while retrieving DAQ data: {}", err),
                Ok(daq_data) => {
                    let res = tx.send_async(daq_data).await;
                    if let Err(err) = res {
                        error!("error while forwarding DAQ data: {}", err);
                    }
                },
            };
            
            tokio::time::sleep(opts.interval).await;
        }
    });

    match &sink {
        #[cfg(feature = "sink_prometheus")]
        Sink::Prometheus(prom_opts) => {
            sink_prometheus::serve_metrics(prom_opts, results_rc).await?;
        },
        #[cfg(feature = "sink_influxdb")]
        Sink::InfluxDB(influx_opts) => {
            sink_influxdb::drain(&influx_opts, rc).await?;
        },
    };
    Ok(())
}
