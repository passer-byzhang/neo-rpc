use serde::{Serialize, Deserialize};
use std::{fs, error::Error};
use std::path::Path;
use anyhow::Result;

const CONFIG_DIR_PATH:&str = "./.neo_cli";
const CONFIG_PATH:&str = "./.neo_cli/config.toml";
#[derive(Debug, Serialize, Deserialize)]
pub struct NeoConfig {
    pub rpc_url: String,
}

impl NeoConfig {
    pub fn init()-> Result<()>{
        if !Path::new(CONFIG_PATH).exists() {
            fs::create_dir(CONFIG_DIR_PATH)?;
            fs::File::create(CONFIG_PATH)?;
            
        }else{
            println!("There is already a config file, do you want to overwrite it? [y/n]:");
            let mut is_overwrite = String::new();
            std::io::stdin().read_line(&mut is_overwrite)?;
            match is_overwrite.trim() {
                "y" => {
                    
                },
                "n" => return Ok(()),
                _ => panic!("error")
            }

        }
        let mut url = String::new();
        println!("Please enter the rpc url:");
        std::io::stdin().read_line(&mut url)?;
        let config = NeoConfig {
            rpc_url:url.trim().to_string()
        };
        let toml = toml::to_string(&config).unwrap();
        fs::write(CONFIG_PATH,toml)?;
        println!("write into {:?}",CONFIG_PATH);
        Ok(())
    }

    pub fn get_config()->Result<NeoConfig,Box<dyn Error>>{
        let config:NeoConfig = toml::from_str(fs::read_to_string(CONFIG_PATH)?.as_str())?;
        Ok(config)
    }

    pub fn get_rpc_url(self:&Self)-> String{
        self.rpc_url.clone()
    }
}

#[test]
pub fn test_config(){
    assert_eq!("http://seed1t4.neo.org:20332",NeoConfig::get_config().unwrap().rpc_url);
}