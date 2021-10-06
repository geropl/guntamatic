use clap::Clap;
use anyhow::anyhow;
use influxdb_client as influxdb;
use influxdb::{Client, Point, Precision, TimestampOptions};

use guntamatic_web as gweb;
use lazy_static::lazy_static;

#[derive(Clap)]
#[derive(Clone)]
pub struct Options {
    #[clap()]
    pub url: String,

    #[clap()]
    pub token: String,

    #[clap()]
    pub bucket: Option<String>,

    #[clap()]
    pub org: Option<String>,
}

lazy_static!{
    static ref WHITESPACE: regex::Regex = regex::Regex::new(r"\s+").unwrap();
}

pub async fn drain(opts: &Options, results_rc: flume::Receiver<gweb::DaqData>) -> Result<(), anyhow::Error> {
    use std::time::SystemTime;
    use influxdb_client::Timestamp;

    let client = {
        let mut client = Client::new(&opts.url, &opts.token)
            .with_precision(Precision::MS);
        if let Some(bucket) = &opts.bucket {
            client = client.with_bucket(bucket);
        }
        if let Some(org) = &opts.org {
            client = client.with_org(org);
        }
        client
    };

    loop {
        let data = match results_rc.recv_async().await {
            Err(err) => {
                error!("error receiving DAQ data: {}", err);
                continue;
            },
            Ok(p) => p,
        };
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        info!("DAQ data received: at {}", now.as_millis());
        
        let points = daq_data_to_points(data)?;
        let ts = Timestamp::Str(now.as_millis().to_string());   // precision as specified above
        client.insert_points(&points, TimestampOptions::Use(ts))
            .await
            .map_err(|err| anyhow!("error inserting into influxdb: {:?}", err))?;
        info!("wrote DAQ data to influxdb")
    }
}

fn daq_data_to_points(daq: gweb::DaqData) -> Result<Vec<Point>, anyhow::Error> {
    use gweb::DataType::*;
    use influxdb::Value;

    let mut points: Vec<Point> = vec![];
    for v in daq.values {
        let desc = v.description;
        let value = match desc.typ {
            Boolean => Value::Bool(v.value.as_bool().unwrap_or(false)),
            Integer => Value::Int(v.value.as_i64().unwrap_or(0)),
            Float => Value::Float(v.value.as_f64().unwrap_or(0.0)),
            String => Value::Str(v.value.as_str().unwrap_or("").into()),
        };
        let name = format!("{}_{}", desc.id, desc.name)
            .to_lowercase();
        let name = WHITESPACE.replace_all(name.as_str(), "-");
        let desc = format!("{} [{}]", desc.name, desc.unit.map_or("none".to_string(), |u| u.to_string()));
        let point = Point::new(name)
            .field("description", Value::Str(desc))
            .field("value", value);
        points.push(point);
    }
    Ok(points)
}