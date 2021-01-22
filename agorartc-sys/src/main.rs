#![allow(non_upper_case_globals)]

use std::io;

use agorartc_sys::agorartc::AgoraRtcEngine;
use agorartc_sys::agorartc::AREA_CODE;

static YOUR_APP_ID: &str = "";

fn on_api_call_executed() {
    println!("API Excuted");
}

fn main() {
    AgoraRtcEngine::instance().event_handler.onApiCallExecuted = Some(on_api_call_executed);
    println!("{}", AgoraRtcEngine::instance().event_handler.onApiCallExecuted.is_some());

    AgoraRtcEngine::instance().initialize(YOUR_APP_ID, AREA_CODE::AREA_CODE_GLOBAL);
    AgoraRtcEngine::instance().enable_video();
    {
        let video_device_manager = AgoraRtcEngine::instance().create_video_device_manager();
        let count = video_device_manager.get_device_count();
        println!("video device count: {}", count);

        let (name, id, ret) = video_device_manager.get_device(0);
        println!("device: {}, {}, {}", name, id, ret);
    }
    AgoraRtcEngine::instance().join_channel("", "123", "", 0u32);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    println!("{}", input);
    AgoraRtcEngine::instance().leave_channel();
    AgoraRtcEngine::instance().release(true);
}
