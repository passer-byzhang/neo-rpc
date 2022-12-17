pub mod rpc_client;
pub mod operation;
pub mod config;


use anyhow::Result;
use rpc_client::RpcClient;
use clap::{Parser, Subcommand};


#[derive(Parser,Debug)]
#[clap(version = "1.0",author = "Alvan")]
struct Opts {
#[clap(subcommand)]
operation :Option<Operation>,
}


#[derive(Subcommand,Debug)]

enum Operation {
   Init,
   Run(Run),
   Query(Query),
}


#[derive(Parser,Debug)]

struct Query {

    method:String,

    params: Vec<String>,

}

#[derive(Parser,Debug)]

struct Run {
    #[clap(long,short)]
    method:String,

    params: Vec<String>,

}

 
#[tokio::main] //此处引入tokio 宏/macro
async fn main()->Result<()>{
    //let args = Args::parse();
    let config = config::NeoConfig::get_config().unwrap();
    let opts = Opts::parse();

    match opts.operation {
        Some(Operation::Query(x)) => {
            let rpc_client = RpcClient::new(config.get_rpc_url().as_str());
            match x.method.as_str() {
                operation::GET_BEST_BLOCK_HASH => println!("result : {:?}",rpc_client.get_best_block_hash().await.unwrap()),
                _=> println!("error")
            }
        },
        Some(Operation::Init)=>{
            config::NeoConfig::init().unwrap()
        },
        _ => println!("error")
    };
    //let client = RpcClient::new("http://seed1t4.neo.org:20332");
    //let x = client.get_best_block_hash().await?;
    Ok(())
}