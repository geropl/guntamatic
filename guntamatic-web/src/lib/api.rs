use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Debug, PartialEq)]
pub struct Context {
    pub addr: String,
    pub key: String,
}

#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Ack { ack: String },
    Err { err: String },
}

pub type Result = std::result::Result<Response, http_types::Error>;


#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum Synonym {
    /// Pk002: Powerchip/Powercorn/Biocom/Pro
    /// K0010: Therm/Biostar
    #[serde(rename = "PK002")]
    BoilerMode,
    #[serde(rename = "PR001")]
    ControlProgram,
    /// TODO Serialize
    #[serde(rename = "HKx01")]
    HeatingProgram {
        /// 0..8
        heating_circuit_id: u8,
    },
    #[serde(rename = "BKx06")]
    HotWaterReload {
        /// 0..2
        heating_circuit_id: u8,
    },
    #[serde(rename = "ZKx06")]
    AdditionalHotWaterReload {
        /// 0..2
        heating_circuit_id: u8,
    },
}

async fn run<P>(ctx: Context, synonym: Synonym, params: P) -> Result 
    where P: Serialize {
    let syn = serde_json::ser::to_string(&synonym)?;
    let params_str = serde_qs::to_string(&params)?;
    let desc_uri = format!("http://{}/ext/parset.cgi?key={}&syn={}&{}", ctx.addr, ctx.key, syn, params_str);
    let res: Response = surf::get(desc_uri)
        .recv_json()
        .await?;
    Ok(res)
}

#[derive(Debug, PartialEq)]
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum BoilerMode {
    Auto = 0,
    Off = 1,
    On = 2,
}

#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Value<V> where V: Serialize {
    pub value: V,
}
impl <V> Value<V> where V: Serialize {
    pub fn new(v: V) -> Self {
        Self { value: v }
    }
}

pub async fn set_boiler_mode(ctx: Context, mode: BoilerMode) -> Result {
    run(ctx, Synonym::BoilerMode, Value::new(mode)).await
}

#[derive(Debug, PartialEq)]
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ControlProgram {
    Off = 0,
    Normal = 1,
    HotWater = 2,
    /// Only for PC/BC/PH/TH/BS/PRO
    Manual = 3,
}

pub async fn set_control_program(ctx: Context, program: ControlProgram) -> Result {
    run(ctx, Synonym::ControlProgram, Value::new(program))
        .await
}


#[derive(Debug, PartialEq)]
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum HeatingProgram {
    Off = 0,
    Normal = 1,
    Heat = 2,
    Reduce = 3,
}

pub async fn set_heating_circuit_program(ctx: Context, heating_circuit_id: u8, program: HeatingProgram) -> Result {
    run(ctx, Synonym::HeatingProgram{ heating_circuit_id }, Value::new(program))
        .await
}

pub async fn set_hot_water_reload(ctx: Context, heating_circuit_id: u8) -> Result {
    run(ctx, Synonym::HotWaterReload{ heating_circuit_id }, Value { value: 1 })
        .await
}

pub async fn set_additional_hot_water_reload(ctx: Context, heating_circuit_id: u8) -> Result {
    run(ctx, Synonym::AdditionalHotWaterReload{ heating_circuit_id }, Value { value: 1 })
        .await
}
