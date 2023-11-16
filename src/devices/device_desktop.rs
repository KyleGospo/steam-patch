use super::Device;
use crate::devices::device_generic::DeviceGeneric;
use crate::devices::Patch;
use crate::patch::PatchFile;
use crate::server::SettingsRequest;
use crate::utils;

pub struct DeviceDesktop {
    device: DeviceGeneric
}

impl DeviceDesktop {
    pub fn new() -> Self {
        DeviceDesktop {
            device: DeviceGeneric::new(15, 800, 1500)
        }
    }
}

impl Device for DeviceDesktop {
    fn update_settings(&self, request: SettingsRequest) {

    }

    fn set_tdp(&self, tdp: i8) {
        println!("Feature not implemented on desktop (TDP): {}", tdp);
    }

    fn set_thermalpolicy(&self, thermalpolicy: i32){
        println!("Feature not implemented on desktop (Thermal policy): {}", thermalpolicy);
    }

    fn set_gpu(&self, gpu: i16) {
        println!("Feature not implemented on desktop (GPU): {}", gpu);
    }

    fn get_patches(&self) -> Vec<Patch> {
        vec![
            // Change resolution to Native (if Default) after installation
            Patch {
                text_to_find: "DownloadComplete_Title\"),o=ze(r,t.data.appid());const s=(0,O.Q2)();".to_string(),
                replacement_text: "DownloadComplete_Title\"),o=ze(r,t.data.appid()); SteamClient.Apps.GetResolutionOverrideForApp(t.data.appid()).then(res => res === \"Default\" && SteamClient.Apps.SetAppResolutionOverride(t.data.appid(), \"Native\")); const s=(0,O.Q2)();".to_string(),
                destination: PatchFile::Chunk,
            },
        ]
    }

    fn get_key_mapper(&self) -> Option<tokio::task::JoinHandle<()>> {
        None
    }
}
