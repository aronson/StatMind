#[macro_use]
extern crate log;

mod types;

use crate::types::{LuigiAi, MapType};
use anyhow::{anyhow, Error};
use discord_rich_presence::activity::Activity;
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use env_logger::Env;
use read_process_memory::copy_address;
use read_process_memory::{Pid, ProcessHandle};
#[cfg(target_os = "macos")]
use security_framework::authorization::{Authorization, AuthorizationItemSetBuilder, Flags};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{mem, thread, time};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

fn main() -> anyhow::Result<()> {
    // Init logger
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    // Get debug introspection (taskport) right on macOS
    #[cfg(target_os = "macos")]
    acquire_taskport_right()?;

    // Create a new System object and refresh process list
    let mut sys = System::new_all();
    sys.refresh_processes();

    // Find the process
    let native_process = sys
        .processes()
        .iter()
        .find(|(_, proc)| {
            proc.name().to_lowercase().contains("cogmind.exe")
                && proc.cmd().contains(&"-luigiAi".to_owned())
        })
        .map(|(_, proc)| proc);
    let wine_process = sys
        .processes()
        .iter()
        .find(|(_, proc)| {
            proc.name().to_lowercase().contains("wine")
                && proc.cmd().contains(&"-luigiAi".to_owned())
        })
        .map(|(_, proc)| proc);
    let process = native_process.or(wine_process);

    if let Some(process) = process {
        debug!("Opening handle to process...");
        // Get a handle to the process
        let pid = process.pid().as_u32() as Pid;
        let handle: ProcessHandle = (pid).try_into()?;

        let (mut client, payload) = init_discord_client_and_payload()?;

        loop {
            debug!("Reading Cogmind process memory...");
            let map_string = get_luigi_map(&handle)?;
            let result = client.set_activity(payload.clone().state(&map_string));
            match result {
                Ok(_) => {
                    info!("State updated! {}", map_string);
                    thread::sleep(time::Duration::from_secs(60));
                }
                Err(e) => {
                    error!("Error updating state:\n{}", e);
                    thread::sleep(time::Duration::from_secs(5));
                }
            }
        }
    } else {
        error!("No process found...");
    }
    Ok(())
}

fn init_discord_client_and_payload() -> Result<(DiscordIpcClient, Activity<'static>), Error> {
    let mut client = DiscordIpcClient::new("914720093701832724").map_err(|e| {
        error!("{}", e);
        anyhow!("Failed to init client!")
    })?;
    client.connect().map_err(|e| {
        error!("{}", e);
        anyhow!("Failed to connect to RPC endpoint!")
    })?;
    let assets = activity::Assets::new()
        .large_image("cogmind_logo")
        .small_image("go_flight")
        .large_text("Cogmind b13 X1")
        .small_text("Flight enjoyer");
    let mut buttons = Vec::new();
    let start = SystemTime::now();
    let start_time = start.duration_since(UNIX_EPOCH)?;
    let timestamp = activity::Timestamps::new().start(start_time.as_secs() as i64);
    buttons.push(activity::Button::new(
        "Visit Site",
        "https://gridsagegames.com/cogmind",
    ));
    buttons.push(activity::Button::new(
        "Buy Game",
        "https://www.gridsagegames.com/cogmind/buy.html",
    ));
    let payload = Activity::new()
        .assets(assets)
        .details("Playing b13 X1")
        .buttons(buttons)
        .timestamps(timestamp);
    Ok((client, payload))
}

fn get_presence(depth: i32, map_type: MapType) -> String {
    let map = match map_type {
        MapType::MapNone => "None",
        MapType::MapSan => "Sandbox",
        MapType::MapScr => "Junkyard",
        MapType::MapMat => "Materials",
        MapType::MapFac => "Factory",
        MapType::MapRes => "Research",
        MapType::MapAcc => "Access",
        MapType::MapSur => "Surface",
        MapType::MapMin => "Mines",
        MapType::MapExi => "Exiles",
        MapType::MapSto => "Storage",
        MapType::MapRec => "Recycling",
        MapType::MapWas => "Wastes",
        MapType::MapGar => "Garrison",
        MapType::MapDsf => "DSF",
        MapType::MapSub => "Subcaves",
        MapType::MapLow => "Lower Caves",
        MapType::MapUpp => "Upper Caves",
        MapType::MapPro => "Proxy Caves",
        MapType::MapDee => "Deep Caves",
        MapType::MapZio => "Zion",
        MapType::MapDat => "Data Miner",
        MapType::MapZhi => "Zhirov",
        MapType::MapWar => "Warlord",
        MapType::MapExt => "Extension",
        MapType::MapCet => "Cetus",
        MapType::MapArc => "Archives",
        MapType::MapHub => "Hub_04(d)",
        MapType::MapArm => "Armory",
        MapType::MapLab => "Lab",
        MapType::MapQua => "Quarantine",
        MapType::MapTes => "Testing",
        MapType::MapSec => "Section 7",
        MapType::MapCom => "Command",
        MapType::MapAc0 => "Access 0",
        MapType::MapLai => "Abomination Lair",
        MapType::MapTow => "Wartown",
        MapType::MapW00 => "w0",
        MapType::MapW01 => "w1",
        MapType::MapW02 => "w2",
        MapType::MapW03 => "w3",
        MapType::MapW04 => "w4",
        MapType::MapW05 => "w5",
        MapType::MapW06 => "w6",
        MapType::MapW07 => "w7",
        MapType::MapW08 => "w8",
    };

    format!("Current map: {}/{}", depth, map)
}

fn get_base_address(handle: &ProcessHandle) -> Result<usize, Error> {
    let first_check_address = 0xC6465C;
    let second_check_address = 0xC66724;
    let first_bytes = copy_address(first_check_address, mem::size_of::<u32>(), handle)?;
    let first_val = i32::from_le_bytes(first_bytes[0..4].try_into().unwrap());
    if first_val == 0x64ADFA4C {
        Ok(first_check_address)
    } else {
        let second_bytes = copy_address(second_check_address, mem::size_of::<u32>(), handle)?;
        let second_val = i32::from_le_bytes(second_bytes[0..4].try_into().unwrap());
        if second_val == 0x64ADFA4C {
            Ok(second_check_address)
        } else {
            Err(anyhow!("Could not find base address"))
        }
    }
}

fn get_luigi_map(handle: &ProcessHandle) -> Result<String, Error> {
    let bytes = copy_address(get_base_address(handle)?, mem::size_of::<LuigiAi>(), handle)?;
    let val: LuigiAi = LuigiAi::from(&bytes);
    let map_type =
        MapType::try_from(val.location_map).map_err(|_e| anyhow!("Failed to convert map type!"))?;
    Ok(get_presence(val.location_depth, map_type))
}

#[cfg(target_os = "macos")]
fn acquire_taskport_right() -> security_framework::base::Result<Authorization> {
    let rights = AuthorizationItemSetBuilder::new()
        .add_right("system.privilege.taskport")?
        .build();
    Authorization::new(
        Some(rights),
        None,
        Flags::EXTEND_RIGHTS | Flags::INTERACTION_ALLOWED | Flags::PREAUTHORIZE,
    )
}
