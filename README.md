# Agora Rust SDK
*[中文](Readme.zh.md) | English*

Use Agora RTC SDK with Rust! 

## Prerequisites

- rustc
- cargo

*If you are not familiar with Rust, please visit [Rust Programming Language](https://www.rust-lang.org/) for more infomation.*

## Installation

Our rust SDK has already been uploaded to [crate.io](https://crates.io/) registry named `agorartc-sys`, thus either can you use SDK from crate.io or download the repository.

#### Method 1: Use Crate Registry

Add a dependency to the `Cargo.toml` of your project.

```rust
[dependencies]
agorartc-sys = "*"
```

#### Method 2: Download Manually

Clone the repository and add a dependency to the `Cargo.toml` of your project.

```rust
[dependencies]
agorartc-sys = { version = "*", path = "[your-path-to]/agorartc-sys"}
```

## Usage

**Visit [Agora-Rust-QuickStart](https://github.com/AgoraIO-Community/Agora-Rust-QuickStart) to obtain some existed demos.**

You can also write a simple demo yourself by the following instructins.

1. Open Terminal (macOS) or PowerShell (Windows) and create a new cargo project.

   ```bash
   $ cargo new first_agorartc_proj
   $ cd first_agorartc_proj
   ```

2. Add a dependency to the `Cargo.toml`.

   ```rust
   [dependencies]
   agorartc-sys = "*"
   ```

3. (macOS) Add a configuration to tell the compiler that `@rpath` is the loader path.

   ```bash
   $ mkdir .cargo
   $ touch .cargo/config.toml
   $ echo "[build]\nrustflags = [\"-C\", \"link-args=-Wl,-rpath,@loader_path\"]" > .cargo/config.toml
   ```

4. Write a simple demo in `src/main.rs`.

   ```rust
   unsafe extern "C" fn on_error(error: ::std::os::raw::c_int, msg: *const ::std::os::raw::c_char) {
       let msg = CStr::from_ptr(msg);
       println!("On Error: code: {:?}, msg: {:?}", error, msg);
   }
   
   unsafe extern "C" fn on_warning(warn: ::std::os::raw::c_int, msg: *const ::std::os::raw::c_char) {
       if msg != std::ptr::null() {
           let msg = CStr::from_ptr(msg);
           println!("On Warn: code: {:?}, msg: {:?}", warn, msg);
       }
       println!("On Warning: code: {:?}", warn);
   }
   
   unsafe extern "C" fn on_join_channel(
       arg1: *const ::std::os::raw::c_char,
       uid: agorartc_sys::agorartc::agorartcnative::uid_t,
       elapsed: ::std::os::raw::c_int,
   ) {
       println!("onJoinChannel");
   }
   
   unsafe extern "C" fn on_user_joined(uid: agorartc_sys::agorartc::agorartcnative::uid_t, elapsed: ::std::os::raw::c_int) {
       println!("onUserJoined");
   }
   
   fn main() {
       let rtc_engine = &agorartc_sys::agorartc::Agora_Rtc_Engine;
       rtc_engine.add_event_handler(&mut agorartc_sys::agorartc::agorartcnative::RtcEventHandler {
           onJoinChannelSuccess: Some(on_join_channel),
           onReJoinChannelSuccess: None,
           onLeaveChannel: None,
           onConnectionLost: None,
           onConnectionInterrupted: None,
           onRequestToken: None,
           onUserJoined: Some(on_user_joined),
           onUserOffline: None,
           onAudioVolumeIndication: None,
           onUserMuteAudio: None,
           onWarning: Some(on_warning),
           onError: Some(on_error),
           onRtcStats: None,
           onAudioMixingFinished: None,
           onAudioRouteChanged: None,
           onFirstRemoteVideoDecoded: None,
           onVideoSizeChanged: None,
           onClientRoleChanged: None,
           onUserMuteVideo: None,
           onMicrophoneEnabled: None,
           onApiCallExecuted: None,
           onFirstLocalAudioFrame: None,
           onFirstRemoteAudioFrame: None,
           onLastmileQuality: None,
           onAudioQuality: None,
           onStreamInjectedStatus: None,
           onStreamUnpublished: None,
           onStreamPublished: None,
           onStreamMessageError: None,
           onStreamMessage: None,
           onConnectionBanned: None,
           onRemoteVideoTransportStats: None,
           onRemoteAudioTransportStats: None,
           onTranscodingUpdated: None,
           onAudioDeviceVolumeChanged: None,
           onActiveSpeaker: None,
           onMediaEngineStartCallSuccess: None,
           onMediaEngineLoadSuccess: None,
           onConnectionStateChanged: None,
           onRemoteSubscribeFallbackToAudioOnly: None,
           onLocalPublishFallbackToAudioOnly: None,
           onUserEnableLocalVideo: None,
           onRemoteVideoStateChanged: None,
           onVideoDeviceStateChanged: None,
           onAudioEffectFinished: None,
           onRemoteAudioMixingEnd: None,
           onRemoteAudioMixingBegin: None,
           onCameraExposureAreaChanged: None,
           onCameraFocusAreaChanged: None,
           onCameraReady: None,
           onAudioDeviceStateChanged: None,
           onUserEnableVideo: None,
           onFirstRemoteVideoFrame: None,
           onFirstLocalVideoFrame: None,
           onRemoteAudioStats: None,
           onRemoteVideoStats: None,
           onLocalVideoStats: None,
           onNetworkQuality: None,
           onTokenPrivilegeWillExpire: None,
           onVideoStopped: None,
           onAudioMixingStateChanged: None,
           onFirstRemoteAudioDecoded: None,
           onLocalVideoStateChanged: None,
           onNetworkTypeChanged: None,
           onRtmpStreamingStateChanged: None,
           onLastmileProbeResult: None,
           onLocalUserRegistered: None,
           onUserInfoUpdated: None,
           onLocalAudioStateChanged: None,
           onRemoteAudioStateChanged: None,
           onLocalAudioStats: None,
           onChannelMediaRelayStateChanged: None,
           onChannelMediaRelayEvent: None,
           onFacePositionChanged: None,
           onTestEnd: None,
       });
       // If you do not have an App ID, see Appendix.
       rtc_engine.initialize("YOUR-APPID", agorartc_sys::agorartc::AREA_CODE::AREA_CODE_GLOBAL);
       rtc_engine.enable_video();
       rtc_engine.join_channel("", "channel-name", "", 0u32);
       let mut input = String::new();
       std::io::stdin().read_line(&mut input).expect("error: unable to read user input");
       rtc_engine.leave_channel();
       rtc_engine.release(true);
   }
   ```

5. Add App ID.

   If you do not have an App ID, see Appendix. Please obtain an App ID without token. Replace `YOUR-APPID` in `rtc_engine.initialize("YOUR-APPID", agorartc_sys::agorartc::AREA_CODE::AREA_CODE_GLOBAL);`  with your App ID.
   
6. Build your cargo project.

   ```bash
   $ cargo build
   ```

7. Download the required SDK.

   - (macOS) Download SDK [Agora Video SDK for macOS](https://download.agora.io/sdk/release/Agora_Native_SDK_for_Mac_v3_1_2_FULL.zip). Unzip the downloaded SDK package and copy the `AograRtcEngineKit.framework` from `libs` folder into `target/debug` folder.
   - (Windows) Download SDK [Agora Video SDK for Windows](https://download.agora.io/sdk/release/Agora_Native_SDK_for_Windows_v3_1_2_FULL.zip). Unzip the downloaded SDK package and copy the `agora_rtc_sdk.dll` and `agora_rtc_sdk.lib` files from `libs/x86_64` into `target/debug` folder.

8. Run demo.

   ```bash
   $ cargo run
   ```

## Appendix

### Create an Account and Obtain an App ID

To use our SDK, you must obtain an app ID: 

1. Create a developer account at [agora.io](https://dashboard.agora.io/signin/). Once you finish the sign-up process, you are redirected to the dashboard.
2. Navigate in the dashboard tree on the left to **Projects** > **Project List**.
3. Copy the app ID that you obtained from the dashboard into a text file. You will use it when you call our SDK.
