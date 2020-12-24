# Agora-Rust-SDK 

*[English](README.md) | 中文*

Agora RTC Rust SDK，目前支持 Windows 和 macOS 平台。

## 运行环境

- rustc
- cargo

若您第一次接触Rust语言，请访问[Rust官网](https://www.rust-lang.org/zh-CN/)以获取更多信息。

## 安装

我们的rust SDK已经被上传至[crate.io](https://crates.io/)目录中，您可以通过`agorartc-sys`关键词检索到我们在crate.io上的主页。您可以使用我们在crate.io上的SDK或下载仓库中的SDK。

### 方法一：使用Crate目录

在您项目的`Cargo.toml`中加入依赖项。

```rust
[dependencies]
agorartc-sys = "*"
```

### 方法二：手动下载

Clone当前仓库并在您项目的`Cargo.toml`中加入依赖项。

```rust
[dependencies]
agorartc-sys = { version = "*", path = "[your-path-to]/agorartc-sys"}
```

## 使用方法

**您可以通过访问[Agora-Rust-QuickStart](https://github.com/AgoraIO-Community/Agora-Rust-QuickStart)获取现有的示例。**

您也可以根据如下教程完成一个简单的示例。

1. 打开终端（macOS）或PowerShell（Windows）并创建一个cargo项目。

   ```bash
   $ cargo new first_agorartc_proj
   $ cd first_agorartc_proj
   ```

2. 在您项目的`Cargo.toml`中加入依赖项。

   ```rust
   [dependencies]
   agorartc-sys = "*"
   ```

3. （macOS）添加一个配置让编译器在编译时将loader路径设置为`@rpath`。

   ```bash
   $ mkdir .cargo
   $ touch .cargo/config.toml
   $ echo "[build]\nrustflags = [\"-C\", \"link-args=-Wl,-rpath,@loader_path\"]" > .cargo/config.toml
   ```

4. 在`src/main.rs`中写下一个简单的示例。

   ```rust
   fn main() {
       let rtc_engine = &agorartc_sys::agorartc::Agora_Rtc_Engine;
       rtc_engine.add_event_handler(&mut agorartc_sys::agorartc::agorartcnative::RtcEventHandler {
           onJoinChannelSuccess: None,
           onReJoinChannelSuccess: None,
           onLeaveChannel: None,
           onConnectionLost: None,
           onConnectionInterrupted: None,
           onRequestToken: None,
           onUserJoined: None,
           onUserOffline: None,
           onAudioVolumeIndication: None,
           onUserMuteAudio: None,
           onWarning: None,
           onError: None,
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
       rtc_engine.initialize("YOUR-APPID", agorartc_sys::agorartc::AREA_CODE::AREA_CODE_GLOBAL); // 如您还未获取App ID，您可以查看附录。
       rtc_engine.enable_video();
       rtc_engine.join_channel("", "channel-name", "", 0u32);
       let mut input = String::new();
       std::io::stdin().read_line(&mut input).expect("error: unable to read user input");
       rtc_engine.leave_channel();
       rtc_engine.release(true);
   }
   ```

5. 编译您的项目

   ```bash
   $ cargo build
   ```

6. 下载所需的SDK。

   - （macOS）在 [Agora Video SDK for macOS](https://download.agora.io/sdk/release/Agora_Native_SDK_for_Mac_v3_1_2_FULL.zip) 下载 SDK。解压缩之后，将 `libs` 目录下的 `AograRtcEngineKit.framework` 复制到`target/debug`中。
   - （Windows）在 [Agora Video SDK for Windows](https://download.agora.io/sdk/release/Agora_Native_SDK_for_Windows_v3_1_2_FULL.zip) 下载 SDK。解压缩之后，将 `libs/x86_64` 目录下的 `agora_rtc_sdk.dll` 和 `agora_rtc_sdk.lib` 复制到`target/debug`中。

7. 运行示例。

   ```bash
   $ cargo run
   ```

## 附录

### 创建Agora账户并获取App ID

如果想要使用我们的SDK，您需要先获得一个App ID：

1. 在[agora.io](https://dashboard.agora.io/signin/)中注册一个账号。当您完成注册后，您将被链接至控制台。
2. 在控制台左侧点击**Projects** > **Project List**。
3. 请将您从控制台中获取的App ID保存，您将会在调用SDK时使用。