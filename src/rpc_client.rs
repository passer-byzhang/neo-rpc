use std::env;
use reqwest::{Client,Url,Request,Response, Error};
use std::{collections::HashMap, str::FromStr};
use serde_json::{Value,json,Number,Map};

#[derive(Debug,Clone)]
pub struct RpcParams{
    jsonrpc:String,
    id:u64,
    method:String,
    params:Value
}

impl RpcParams {
    pub fn new(method:String,params:Value) -> RpcParams {
        RpcParams {
            jsonrpc:"2.0".to_string(),
            method,
            id:1,
            params
        }
    }

    pub fn to_value(&self) -> Value{
        let p = self.clone();
        let mut v = json!(null);
        v["params"] = p.params;
        v["method"] = Value::String(p.method);
        v["jsonrpc"] = Value::String(p.jsonrpc);
        v["id"] = Value::Number(Number::from(p.id));
        v
    }
}

pub struct RpcClient{
    pub client : Client,
    pub url: Url,
}


impl RpcClient {
    pub fn new(url:&str) -> RpcClient {
        let url = Url::parse(url).unwrap();
        return RpcClient {
            client : Client::new(),
            url : url
        }
    }

    pub async fn get_best_block_hash(self:&Self)->Result<String,Error> {
        let params = Value::Array(vec![]);
        let rpc_params = RpcParams::new("getbestblockhash".to_string(),params).to_value();
        let result_raw = self.client.post(self.url.as_str()).json(&rpc_params.as_object().unwrap()).send().await?.text().await?;
        let result = Value::from_str(result_raw.as_str()).unwrap();
        Ok(result["result"].as_str().unwrap().to_string())
    }
}