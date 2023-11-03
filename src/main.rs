use serde_json::{Result, Value};
use std::fs;

fn main() -> Result<()> {
    let data = read_file("podaci.json");
    let received_data = parse_data(data)?;
    println!("received data\n {}", received_data);
    Ok(())
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("should read file")
}
fn parse_data(data: String) -> Result<String> {
    let v: Value = serde_json::from_str(&data)?;
    /*
    0-Temperatura
    1-Solar panel voltage
    2-Battery voltage
    3-Battery current Averige
    4-Battery status
    5-Battery status averige
    6-Solar panel day/night
    7-Solar panel day/night averige
    8- Modem power state
    */
    let result = format!(
        "Station name : {}\n {} : {} : {}: {}: {}: {}: {}: {}: {}: {}: {}: {}: {}: {}: {}: {}",
        v["head"]["environment"]["station_name"],
        v["head"]["fields"][0]["name"],
        v["data"][0]["vals"][0],
        v["head"]["fields"][1]["name"],
        v["data"][0]["vals"][1],
        v["head"]["fields"][2]["name"],
        v["data"][0]["vals"][2],
        v["head"]["fields"][3]["name"],
        v["data"][0]["vals"][3],
        v["head"]["fields"][4]["name"],
        v["data"][0]["vals"][4],
        v["head"]["fields"][5]["name"],
        v["data"][0]["vals"][5],
        v["head"]["fields"][6]["name"],
        v["data"][0]["vals"][6],
        v["head"]["fields"][7]["name"],
        v["data"][0]["vals"][7],
    );

    Ok(result)
}
