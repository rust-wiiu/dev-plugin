#![no_std]
#![no_main]

use core::ptr;

use wut::bindings::OSDynLoad_NotifyData;
use wut::bindings::*;
use wut::font::icons;
use wut::prelude::*;
use wut::*;

use notifications;
use wups::prelude::*;
use wups::*;

// use wups::{
//     wups_plugin, wups_get_config,
//     memory::{os_dynload_get_number_of_rpls, os_dynload_get_rpl_info, OSDynLoadNotifyData},
//     notifications::{add_info_notification, NMColor, add_info_notification_ex},
// };

// wups_plugin!(KioskPatch, "Patches Kiosk stuff");
WUPS_PLUGIN_NAME!("KioskPatch");

fn is_current_game_menu_app() -> bool {
    matches!(
        // unsafe { OSGetTitleID() },
        wut::title::current_title(),
        0x0005000010162B00 | 0x0005000010176900 | 0x0005000010176A00
    )
}

#[no_mangle]
pub extern "C" fn on_application_start() {
    if is_current_game_menu_app() {
        // add_info_notification("Splatoon Detected, applying patch");
        notifications::info("Splatoon Detected, applying patch");

        // let rpl_count = unsafe { os_dynload_get_number_of_rpls() };
        // let mut rpls = vec![OSDynLoadNotifyData::default(); rpl_count];
        let rpl_count = unsafe { wut::bindings::OSDynLoad_GetNumberOfRPLs() };
        let mut rpls = vec![wut::bindings::OSDynLoad_NotifyData::default(); rpl_count as usize];

        if unsafe { wut::bindings::OSDynLoad_GetRPLInfo(0, rpl_count as u32, rpls.as_mut_ptr()) }
            == 0
        {
            // add_info_notification("Failed to get RPLs");
            notifications::info("Failed to get RPLs");
            return;
        }

        for rpl in rpls.iter() {
            // if let Some(name) = rpl.name_as_str() {

            let name = unsafe { alloc::ffi::CString::from_raw(rpl.name) }
                .to_string_lossy()
                .into_owned();

            if name.contains("Gambit") {
                // add_info_notification(name);
                // let text_color = NMColor::new(255, 255, 255, 255);
                // let bg_color = NMColor::new(0, 0, 0, 0);
                // add_info_notification_ex(name, 30.0, text_color, bg_color, None, None, true);
                // add_info_notification("Applying patches!!!!");
                notifications::NotificationBuilder::<notifications::Info>::default()
                    .background_color(notifications::Color::black(1.0))
                    .text_color(notifications::Color::white(1.0))
                    .text(&format!("{} - Applying patches!!!!", name))
                    .show();

                // let game_data_region = rpl.data_addr as *mut u8;
                let game_data_region = rpl.dataAddr as *mut u8;
                let str_loc = unsafe { game_data_region.add(0x000009ac) } as *mut u8;
                let new_url = b"https://s-res.spfn.cc/post\0";

                unsafe {
                    ptr::copy_nonoverlapping(new_url.as_ptr(), str_loc, new_url.len());
                }
                // }
            }
        }
    }
}
