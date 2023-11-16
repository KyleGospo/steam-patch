use super::Device;
use crate::config::{get_global_config, self};
use crate::devices::device_generic::DeviceGeneric;
use crate::devices::Patch;
use crate::patch::PatchFile;
use crate::server::SettingsRequest;
use crate::steam::SteamClient;
use crate::{utils, main};
use std::fs::File;
use std::path::Path;
use std::{fs, env};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::io::{self, Write, Read};
use std::io::BufRead;
use std::collections::HashMap;


pub struct DeviceGo {
    device: DeviceGeneric,
}
#[derive(Debug)]
struct ByteData {
    index: usize,
    value: String,
}

impl DeviceGo {
    pub fn new(tdp: i8, gpu: i16) -> Self {
        DeviceGo {device: DeviceGeneric::new(tdp, 800,gpu)}
}
}

impl Device for DeviceGo {
    fn set_thermalpolicy(&self, thermalpolicy: i32){
        // The actual implementation would go here
        println!("Feature not implemented outside of ROG ALLY (Thermal policy): {}", thermalpolicy);
    }

    fn update_settings(&self, request: SettingsRequest) {
        if let Some(per_app) = &request.per_app {
            println!("{:#?}",per_app);
            // TDP changes
            if let Some(true) = per_app.is_tdp_limit_enabled {
                if let Some(tdp) = per_app.tdp_limit {
                    self.set_tdp(tdp);
                }
            } 
            if let Some(gpu) = per_app.gpu_performance_manual_mhz {
                self.set_gpu(gpu);
            }
        }
    }
    //Add more patches for device specific
    fn get_patches(&self) -> Vec<Patch> {
        let mut patches = self.device.get_patches();
        patches.push(Patch {
            text_to_find: String::from("this.m_rgControllers=new Map,\"undefined\"!=typeof SteamClient&&(this.m_hUnregisterControllerDigitalInput"),
            replacement_text: String::from("this.m_rgControllers=new Map; window.HandleSystemKeyEvents = this.HandleSystemKeyEvents; \"undefined\"!=typeof SteamClient&&(this.m_hUnregisterControllerDigitalInput"),
            destination: PatchFile::Library,
        });
        patches
    }

    fn set_tdp(&self, tdp: i8) {
        self.device.set_tdp(tdp);
    }

    fn set_gpu(&self, gpu: i16) {
        //Placeholder for later implementations
        println!("New GPU clock: {}", gpu);
    }

    fn get_key_mapper(&self) -> Option<tokio::task::JoinHandle<()>> {
        tokio::spawn(async move {
            let mut steam = SteamClient::new();
            steam.connect().await;
            start_mapper(steam);
        });
        None
    }
}

fn read_from_hidraw(device_path: &str, buffer_size: usize) -> io::Result<Vec<u8>> {
    let path = Path::new(device_path);
    let mut device = File::open(path)?;

    let mut buffer = vec![0u8; buffer_size];
    let bytes_read = device.read(&mut buffer)?;

    buffer.truncate(bytes_read);
    Ok(buffer)
}

pub fn start_mapper(mut steam:SteamClient) -> Option<tokio::task::JoinHandle<()>> {
    let conf = get_global_config();
    let device_path = "/dev/hidraw2"; 
    let buffer_size = 1024;
    let mut previous_data = Vec::new(); // Variable to keep track of prev states
    println!("Steam mapper {}", conf.mapper);
    if conf.mapper {
        Some(tokio::spawn(async move {
            println!("Mapper enabled");
            loop {
                match read_from_hidraw(device_path, buffer_size) {
                    Ok(data) => {

                        if previous_data != data {
                            // print!("Controller data: {:?}",data);
                            if(data[18] == 64){
                                println!("Show QAM");
                                        steam
                                            .execute("GamepadNavTree.m_Controller.OnButtonActionInternal(true, 28, 2)")
                                            .await;
                            }
                            if(data[18] == 128){
                                println!("Show Menu");
                                        steam
                                            .execute("GamepadNavTree.m_Controller.OnButtonActionInternal(true, 27, 2); console.log(\"Show Menu\");")
                                            .await;
                            }
                            if(data[18] == 128 && data[19] == 32) {
                                println!("Show keyboard")
                            }
                            
                            //Update prev state
                            previous_data = data.clone();
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to read from device: {}", e);
                        eprintln!("Retrying in 1 second");
                        thread::sleep(Duration::from_secs(1));
                        tokio::spawn(async move {
                            start_mapper(steam)
                        });
                        break
                    },
                }
            }
            
        }))
    } else {
        println!("Mapper disabled");
        None
    }
}
