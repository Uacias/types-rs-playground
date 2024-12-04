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

    let invoke_req = RpcRequest {
        id: 1,
        jsonrpc: "2.0",
        method: "starknet_traceTransaction",
        params: vec!["0x064f7c084d9cba540cba343f3ec1b69a06bd9169c9016e518d06d418003a31fd"],
    };

    // Set correct url
    let url = "https://starknet-mainnet.g.alchemy.com/starknet/version/rpc/v0_7/";
    let response = client
        .post(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&invoke_req)
        .send()
        .await?;

    if response.status().is_success() {
        let response: RpcResponse<TransactionTrace<Felt>> = response.json().await?;
        println!("{:?}", response);

        let file = File::create("response_invoke.json")?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &response)?;

        println!("Response successfully saved to asd.json");
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        eprintln!("Request failed with status {}: {}", status, error_text);
    }

    let deploy_acc = RpcRequest {
        id: 1,
        jsonrpc: "2.0",
        method: "starknet_traceTransaction",
        params: vec!["0x03cda84d48a31f0db50705347108c81a37292098e94d0ba5d9ab52c1568c2928"],
    };

    let response = client
        .post(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&deploy_acc)
        .send()
        .await?;

    if response.status().is_success() {
        let response: RpcResponse<TransactionTrace<Felt>> = response.json().await?;
        println!("{:?}", response);

        let file = File::create("response_deploy_acc.json")?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &response)?;

        println!("Response successfully saved to asd.json");
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        eprintln!("Request failed with status {}: {}", status, error_text);
    }

    let declare = RpcRequest {
        id: 1,
        jsonrpc: "2.0",
        method: "starknet_traceTransaction",
        params: vec!["0x0144949e295dd91d7aa29ca08304b053121ac9f62d7ff43067d5c47d0354855f"],
    };

    let response = client
        .post(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&declare)
        .send()
        .await?;

    if response.status().is_success() {
        let response: RpcResponse<TransactionTrace<Felt>> = response.json().await?;
        println!("{:?}", response);

        let file = File::create("response_declare.json")?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &response)?;

        println!("Response successfully saved to asd.json");
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        eprintln!("Request failed with status {}: {}", status, error_text);
    }

    let l1_handler = RpcRequest {
        id: 1,
        jsonrpc: "2.0",
        method: "starknet_traceTransaction",
        params: vec!["0x030aafc11773e25dff4bbef2d6801f201b474a5685aa7f90c8667ad82f0d2c06"],
    };

    let response = client
        .post(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&l1_handler)
        .send()
        .await?;

    if response.status().is_success() {
        let response: RpcResponse<TransactionTrace<Felt>> = response.json().await?;
        println!("{:?}", response);

        let file = File::create("response_l1_handler.json")?;
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
