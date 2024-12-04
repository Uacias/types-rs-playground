use reqwest::Client;
use serde::{Deserialize, Serialize};

use starknet_types_core::felt::Felt;
use starknet_types_rpc::TransactionTrace;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

#[derive(Serialize)]
struct RpcRequest<'a> {
    id: u32,
    jsonrpc: &'a str,
    method: &'a str,
    params: Vec<&'a str>,
}

#[derive(Deserialize, Debug, Serialize)]
struct RpcResponse<T> {
    result: T,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let rpc_request = RpcRequest {
        id: 1,
        jsonrpc: "2.0",
        method: "starknet_traceTransaction",
        params: vec!["0x064f7c084d9cba540cba343f3ec1b69a06bd9169c9016e518d06d418003a31fd"],
    };

    // Set correct url
    let url = "";
    let response = client
        .post(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&rpc_request)
        .send()
        .await?;

    if response.status().is_success() {
        let response: RpcResponse<TransactionTrace<Felt>> = response.json().await?;
        println!("{:?}", response);

        let file = File::create("response.json")?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &response)?;

        println!("Response successfully saved to asd.json");
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        eprintln!("Request failed with status {}: {}", status, error_text);
    }

    Ok(())
}
