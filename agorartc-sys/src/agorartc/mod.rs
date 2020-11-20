//
//  Created by Yifan Dong on 2020/10/20.
//  Copyright © 2020 Agora. All rights reserved.
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
extern crate num;

extern crate lazy_static;

pub mod agorartcnative;

use std::ffi::{CString, CStr};
use lazy_static::lazy_static;


/**
 * IP areas.
 */
#[derive(FromPrimitive)]
pub enum AREA_CODE {
    /**
     * Mainland China.
     */
    AREA_CODE_CN = 1 << 0,
    /**
     * North America.
     */
    AREA_CODE_NA = 1 << 1,
    /**
     * Europe.
     */
    AREA_CODE_EUR = 1 << 2,
    /**
     * Asia, excluding Mainland China.
     */
    AREA_CODE_AS = 1 << 3,
    /**
     * Japan.
     */
    AREA_CODE_JAPAN = 1 << 4,
    /**
     * India.
     */
    AREA_CODE_INDIA = 1 << 5,
    /**
     * (Default) Global.
     */
    AREA_CODE_GLOBAL = 0xFFFFFFFF,
}

/** The channel profile.
*/
#[derive(FromPrimitive)]
pub enum CHANNEL_PROFILE_TYPE {
    /** (Default) Communication. This profile applies to scenarios such as an audio call or video call,
     * where all users can publish and subscribe to streams.
     */
    CHANNEL_PROFILE_COMMUNICATION = 0,
    /** Live streaming. In this profile, uses have roles, namely, host and audience (default).
     * A host both publishes and subscribes to streams, while an audience subscribes to streams only.
     * This profile applies to scenarios such as a chat room or interactive video streaming.
     */
    CHANNEL_PROFILE_LIVE_BROADCASTING = 1,
    /** 2: Gaming. This profile uses a codec with a lower bitrate and consumes less power. Applies to the gaming scenario, where all game players can talk freely.
     */
    CHANNEL_PROFILE_GAME = 2,
}

/** Client roles in the live interactive streaming. */
#[derive(FromPrimitive)]
pub enum CLIENT_ROLE_TYPE {
    /** 1: Host. A host can both send and receive streams. */
    CLIENT_ROLE_BROADCASTER = 1,
    /** 2: Audience, the default role. An audience can only receive streams. */
    CLIENT_ROLE_AUDIENCE = 2,
}

/** **DEPRECATED** Video profiles. */
#[derive(FromPrimitive)]
pub enum VIDEO_PROFILE_TYPE {
    /** 0: 160 * 120, frame rate 15 fps, bitrate 65 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_120P = 0,
    /** 2: 120 * 120, frame rate 15 fps, bitrate 50 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_120P_3 = 2,
    /** 10: 320*180, frame rate 15 fps, bitrate 140 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_180P = 10,
    /** 12: 180 * 180, frame rate 15 fps, bitrate 100 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_180P_3 = 12,
    /** 13: 240 * 180, frame rate 15 fps, bitrate 120 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_180P_4 = 13,
    /** 20: 320 * 240, frame rate 15 fps, bitrate 200 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_240P = 20,
    /** 22: 240 * 240, frame rate 15 fps, bitrate 140 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_240P_3 = 22,
    /** 23: 424 * 240, frame rate 15 fps, bitrate 220 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_240P_4 = 23,
    /** 30: 640 * 360, frame rate 15 fps, bitrate 400 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_360P = 30,
    /** 32: 360 * 360, frame rate 15 fps, bitrate 260 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_360P_3 = 32,
    /** 33: 640 * 360, frame rate 30 fps, bitrate 600 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_360P_4 = 33,
    /** 35: 360 * 360, frame rate 30 fps, bitrate 400 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_360P_6 = 35,
    /** 36: 480 * 360, frame rate 15 fps, bitrate 320 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_360P_7 = 36,
    /** 37: 480 * 360, frame rate 30 fps, bitrate 490 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_360P_8 = 37,
    /** 38: 640 * 360, frame rate 15 fps, bitrate 800 Kbps.
     @note `LIVE_BROADCASTING` profile only.
     */
    VIDEO_PROFILE_LANDSCAPE_360P_9 = 38,
    /** 39: 640 * 360, frame rate 24 fps, bitrate 800 Kbps.
     @note `LIVE_BROADCASTING` profile only.
     */
    VIDEO_PROFILE_LANDSCAPE_360P_10 = 39,
    /** 100: 640 * 360, frame rate 24 fps, bitrate 1000 Kbps.
     @note `LIVE_BROADCASTING` profile only.
     */
    VIDEO_PROFILE_LANDSCAPE_360P_11 = 100,
    /** 40: 640 * 480, frame rate 15 fps, bitrate 500 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_480P = 40,
    /** 42: 480 * 480, frame rate 15 fps, bitrate 400 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_480P_3 = 42,
    /** 43: 640 * 480, frame rate 30 fps, bitrate 750 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_480P_4 = 43,
    /** 45: 480 * 480, frame rate 30 fps, bitrate 600 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_480P_6 = 45,
    /** 47: 848 * 480, frame rate 15 fps, bitrate 610 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_480P_8 = 47,
    /** 48: 848 * 480, frame rate 30 fps, bitrate 930 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_480P_9 = 48,
    /** 49: 640 * 480, frame rate 10 fps, bitrate 400 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_480P_10 = 49,
    /** 50: 1280 * 720, frame rate 15 fps, bitrate 1130 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_720P = 50,
    /** 52: 1280 * 720, frame rate 30 fps, bitrate 1710 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_720P_3 = 52,
    /** 54: 960 * 720, frame rate 15 fps, bitrate 910 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_720P_5 = 54,
    /** 55: 960 * 720, frame rate 30 fps, bitrate 1380 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_720P_6 = 55,
    /** 60: 1920 * 1080, frame rate 15 fps, bitrate 2080 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_1080P = 60,
    /** 62: 1920 * 1080, frame rate 30 fps, bitrate 3150 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_1080P_3 = 62,
    /** 64: 1920 * 1080, frame rate 60 fps, bitrate 4780 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_1080P_5 = 64,
    /** 66: 2560 * 1440, frame rate 30 fps, bitrate 4850 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_1440P = 66,
    /** 67: 2560 * 1440, frame rate 60 fps, bitrate 6500 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_1440P_2 = 67,
    /** 70: 3840 * 2160, frame rate 30 fps, bitrate 6500 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_4K = 70,
    /** 72: 3840 * 2160, frame rate 60 fps, bitrate 6500 Kbps. */
    VIDEO_PROFILE_LANDSCAPE_4K_3 = 72,
    /** 1000: 120 * 160, frame rate 15 fps, bitrate 65 Kbps. */
    VIDEO_PROFILE_PORTRAIT_120P = 1000,
    /** 1002: 120 * 120, frame rate 15 fps, bitrate 50 Kbps. */
    VIDEO_PROFILE_PORTRAIT_120P_3 = 1002,
    /** 1010: 180 * 320, frame rate 15 fps, bitrate 140 Kbps. */
    VIDEO_PROFILE_PORTRAIT_180P = 1010,
    /** 1012: 180 * 180, frame rate 15 fps, bitrate 100 Kbps. */
    VIDEO_PROFILE_PORTRAIT_180P_3 = 1012,
    /** 1013: 180 * 240, frame rate 15 fps, bitrate 120 Kbps. */
    VIDEO_PROFILE_PORTRAIT_180P_4 = 1013,
    /** 1020: 240 * 320, frame rate 15 fps, bitrate 200 Kbps. */
    VIDEO_PROFILE_PORTRAIT_240P = 1020,
    /** 1022: 240 * 240, frame rate 15 fps, bitrate 140 Kbps. */
    VIDEO_PROFILE_PORTRAIT_240P_3 = 1022,
    /** 1023: 240 * 424, frame rate 15 fps, bitrate 220 Kbps. */
    VIDEO_PROFILE_PORTRAIT_240P_4 = 1023,
    /** 1030: 360 * 640, frame rate 15 fps, bitrate 400 Kbps. */
    VIDEO_PROFILE_PORTRAIT_360P = 1030,
    /** 1032: 360 * 360, frame rate 15 fps, bitrate 260 Kbps. */
    VIDEO_PROFILE_PORTRAIT_360P_3 = 1032,
    /** 1033: 360 * 640, frame rate 30 fps, bitrate 600 Kbps. */
    VIDEO_PROFILE_PORTRAIT_360P_4 = 1033,
    /** 1035: 360 * 360, frame rate 30 fps, bitrate 400 Kbps. */
    VIDEO_PROFILE_PORTRAIT_360P_6 = 1035,
    /** 1036: 360 * 480, frame rate 15 fps, bitrate 320 Kbps. */
    VIDEO_PROFILE_PORTRAIT_360P_7 = 1036,
    /** 1037: 360 * 480, frame rate 30 fps, bitrate 490 Kbps. */
    VIDEO_PROFILE_PORTRAIT_360P_8 = 1037,
    /** 1038: 360 * 640, frame rate 15 fps, bitrate 800 Kbps.
     @note `LIVE_BROADCASTING` profile only.
     */
    VIDEO_PROFILE_PORTRAIT_360P_9 = 1038,
    /** 1039: 360 * 640, frame rate 24 fps, bitrate 800 Kbps.
     @note `LIVE_BROADCASTING` profile only.
     */
    VIDEO_PROFILE_PORTRAIT_360P_10 = 1039,
    /** 1100: 360 * 640, frame rate 24 fps, bitrate 1000 Kbps.
     @note `LIVE_BROADCASTING` profile only.
     */
    VIDEO_PROFILE_PORTRAIT_360P_11 = 1100,
    /** 1040: 480 * 640, frame rate 15 fps, bitrate 500 Kbps. */
    VIDEO_PROFILE_PORTRAIT_480P = 1040,
    /** 1042: 480 * 480, frame rate 15 fps, bitrate 400 Kbps. */
    VIDEO_PROFILE_PORTRAIT_480P_3 = 1042,
    /** 1043: 480 * 640, frame rate 30 fps, bitrate 750 Kbps. */
    VIDEO_PROFILE_PORTRAIT_480P_4 = 1043,
    /** 1045: 480 * 480, frame rate 30 fps, bitrate 600 Kbps. */
    VIDEO_PROFILE_PORTRAIT_480P_6 = 1045,
    /** 1047: 480 * 848, frame rate 15 fps, bitrate 610 Kbps. */
    VIDEO_PROFILE_PORTRAIT_480P_8 = 1047,
    /** 1048: 480 * 848, frame rate 30 fps, bitrate 930 Kbps. */
    VIDEO_PROFILE_PORTRAIT_480P_9 = 1048,
    /** 1049: 480 * 640, frame rate 10 fps, bitrate 400 Kbps. */
    VIDEO_PROFILE_PORTRAIT_480P_10 = 1049,
    /** 1050: 720 * 1280, frame rate 15 fps, bitrate 1130 Kbps. */
    VIDEO_PROFILE_PORTRAIT_720P = 1050,
    /** 1052: 720 * 1280, frame rate 30 fps, bitrate 1710 Kbps. */
    VIDEO_PROFILE_PORTRAIT_720P_3 = 1052,
    /** 1054: 720 * 960, frame rate 15 fps, bitrate 910 Kbps. */
    VIDEO_PROFILE_PORTRAIT_720P_5 = 1054,
    /** 1055: 720 * 960, frame rate 30 fps, bitrate 1380 Kbps. */
    VIDEO_PROFILE_PORTRAIT_720P_6 = 1055,
    /** 1060: 1080 * 1920, frame rate 15 fps, bitrate 2080 Kbps. */
    VIDEO_PROFILE_PORTRAIT_1080P = 1060,
    /** 1062: 1080 * 1920, frame rate 30 fps, bitrate 3150 Kbps. */
    VIDEO_PROFILE_PORTRAIT_1080P_3 = 1062,
    /** 1064: 1080 * 1920, frame rate 60 fps, bitrate 4780 Kbps. */
    VIDEO_PROFILE_PORTRAIT_1080P_5 = 1064,
    /** 1066: 1440 * 2560, frame rate 30 fps, bitrate 4850 Kbps. */
    VIDEO_PROFILE_PORTRAIT_1440P = 1066,
    /** 1067: 1440 * 2560, frame rate 60 fps, bitrate 6500 Kbps. */
    VIDEO_PROFILE_PORTRAIT_1440P_2 = 1067,
    /** 1070: 2160 * 3840, frame rate 30 fps, bitrate 6500 Kbps. */
    VIDEO_PROFILE_PORTRAIT_4K = 1070,
    /** 1072: 2160 * 3840, frame rate 60 fps, bitrate 6500 Kbps. */
    VIDEO_PROFILE_PORTRAIT_4K_3 = 1072,
    // Default 640 * 360, frame rate 15 fps, bitrate 400 Kbps.
    // VIDEO_PROFILE_DEFAULT = VIDEO_PROFILE_LANDSCAPE_360P,
}

/** The priority of the remote user.
 */
#[derive(FromPrimitive)]
pub enum PRIORITY_TYPE {
    /** 50: The user's priority is high.
     */
    PRIORITY_HIGH = 50,
    /** 100: (Default) The user's priority is normal.
    */
    PRIORITY_NORMAL = 100,
}

/** Audio profiles.

Sets the sample rate, bitrate, encoding mode, and the number of channels:*/
#[derive(FromPrimitive)]
pub enum AUDIO_PROFILE_TYPE // sample rate, bit rate, mono/stereo, speech/music codec
{
    /**
     0: Default audio profile:
     - For the interactive streaming profile: A sample rate of 48 KHz, music encoding, mono, and a bitrate of up to 64 Kbps.
     - For the `COMMUNICATION` profile:
        - Windows: A sample rate of 16 KHz, music encoding, mono, and a bitrate of up to 16 Kbps.
        - Android/macOS/iOS: A sample rate of 32 KHz, music encoding, mono, and a bitrate of up to 18 Kbps.
    */
    AUDIO_PROFILE_DEFAULT = 0,
    // use default settings
    /**
     1: A sample rate of 32 KHz, audio encoding, mono, and a bitrate of up to 18 Kbps.
     */
    AUDIO_PROFILE_SPEECH_STANDARD = 1,
    // 32Khz, 18Kbps, mono, speech
    /**
     2: A sample rate of 48 KHz, music encoding, mono, and a bitrate of up to 64 Kbps.
     */
    AUDIO_PROFILE_MUSIC_STANDARD = 2,
    // 48Khz, 48Kbps, mono, music
    /**
     3: A sample rate of 48 KHz, music encoding, stereo, and a bitrate of up to 80 Kbps.
     */
    AUDIO_PROFILE_MUSIC_STANDARD_STEREO = 3,
    // 48Khz, 56Kbps, stereo, music
    /**
     4: A sample rate of 48 KHz, music encoding, mono, and a bitrate of up to 96 Kbps.
     */
    AUDIO_PROFILE_MUSIC_HIGH_QUALITY = 4,
    // 48Khz, 128Kbps, mono, music
    /**
     5: A sample rate of 48 KHz, music encoding, stereo, and a bitrate of up to 128 Kbps.
     */
    AUDIO_PROFILE_MUSIC_HIGH_QUALITY_STEREO = 5,
    // 48Khz, 192Kbps, stereo, music
    /**
     6: A sample rate of 16 KHz, audio encoding, mono, and Acoustic Echo Cancellation (AES) enabled.
     */
    AUDIO_PROFILE_IOT = 6,
    AUDIO_PROFILE_NUM = 7,
}

/** Audio application scenarios.
*/
#[derive(FromPrimitive)]
pub enum AUDIO_SCENARIO_TYPE // set a suitable scenario for your app type
{
    /** 0: Default. */
    AUDIO_SCENARIO_DEFAULT = 0,
    /** 1: Entertainment scenario, supporting voice during gameplay. */
    AUDIO_SCENARIO_CHATROOM_ENTERTAINMENT = 1,
    /** 2: Education scenario, prioritizing smoothness and stability. */
    AUDIO_SCENARIO_EDUCATION = 2,
    /** 3: Live gaming scenario, enabling the gaming audio effects in the speaker mode in the interactive live streaming scenario. Choose this scenario for high-fidelity music playback. */
    AUDIO_SCENARIO_GAME_STREAMING = 3,
    /** 4: Showroom scenario, optimizing the audio quality with external professional equipment. */
    AUDIO_SCENARIO_SHOWROOM = 4,
    /** 5: Gaming scenario. */
    AUDIO_SCENARIO_CHATROOM_GAMING = 5,
    /** 6: Applicable to the IoT scenario. */
    AUDIO_SCENARIO_IOT = 6,
    AUDIO_SCENARIO_NUM = 7,
}

/** Remote video stream types. */
#[derive(FromPrimitive)]
pub enum REMOTE_VIDEO_STREAM_TYPE {
    /** 0: High-stream video. */
    REMOTE_VIDEO_STREAM_HIGH = 0,
    /** 1: Low-stream video. */
    REMOTE_VIDEO_STREAM_LOW = 1,
}

/** Audio recording qualities.
*/
#[derive(FromPrimitive)]
pub enum AUDIO_RECORDING_QUALITY_TYPE {
    /** 0: Low quality. The sample rate is 32 kHz, and the file size is around
     * 1.2 MB after 10 minutes of recording.
    */
    AUDIO_RECORDING_QUALITY_LOW = 0,
    /** 1: Medium quality. The sample rate is 32 kHz, and the file size is
     * around 2 MB after 10 minutes of recording.
    */
    AUDIO_RECORDING_QUALITY_MEDIUM = 1,
    /** 2: High quality. The sample rate is 32 kHz, and the file size is
     * around 3.75 MB after 10 minutes of recording.
    */
    AUDIO_RECORDING_QUALITY_HIGH = 2,
}

/** Video display modes. */
#[derive(FromPrimitive)]
pub enum RENDER_MODE_TYPE {
    /**
  1: Uniformly scale the video until it fills the visible boundaries (cropped). One dimension of the video may have clipped contents.
   */
    RENDER_MODE_HIDDEN = 1,
    /**
2: Uniformly scale the video until one of its dimension fits the boundary (zoomed to fit). Areas that are not filled due to disparity in the aspect ratio are filled with black.
 */
    RENDER_MODE_FIT = 2,
    /** **DEPRECATED** 3: This mode is deprecated.
     */
    RENDER_MODE_ADAPTIVE = 3,
    /**
    4: The fill mode. In this mode, the SDK stretches or zooms the video to fill the display window.
    */
    RENDER_MODE_FILL = 4,
}

/** Video mirror modes. */
#[derive(FromPrimitive)]
pub enum VIDEO_MIRROR_MODE_TYPE {
    /** 0: (Default) The SDK enables the mirror mode.
     */
    VIDEO_MIRROR_MODE_AUTO = 0,
    //determined by SDK
    /** 1: Enable mirror mode. */
    VIDEO_MIRROR_MODE_ENABLED = 1,
    //enabled mirror
    /** 2: Disable mirror mode. */
    VIDEO_MIRROR_MODE_DISABLED = 2, //disable mirror
}

/** Stream fallback options. */
#[derive(FromPrimitive)]
pub enum STREAM_FALLBACK_OPTIONS {
    /** 0: No fallback behavior for the local/remote video stream when the uplink/downlink network conditions are poor. The quality of the stream is not guaranteed. */
    STREAM_FALLBACK_OPTION_DISABLED = 0,
    /** 1: Under poor downlink network conditions, the remote video stream, to which you subscribe, falls back to the low-stream (low resolution and low bitrate) video. You can set this option only in the \ref IRtcEngine::setRemoteSubscribeFallbackOption "setRemoteSubscribeFallbackOption" method. Nothing happens when you set this in the \ref IRtcEngine::setLocalPublishFallbackOption "setLocalPublishFallbackOption" method. */
    STREAM_FALLBACK_OPTION_VIDEO_STREAM_LOW = 1,
    /** 2: Under poor uplink network conditions, the published video stream falls back to audio only.

    Under poor downlink network conditions, the remote video stream, to which you subscribe, first falls back to the low-stream (low resolution and low bitrate) video; and then to an audio-only stream if the network conditions worsen.*/
    STREAM_FALLBACK_OPTION_AUDIO_ONLY = 2,
}

/** Audio equalization band frequencies. */
pub enum AUDIO_EQUALIZATION_BAND_FREQUENCY {
    /** 0: 31 Hz */
    AUDIO_EQUALIZATION_BAND_31 = 0,
    /** 1: 62 Hz */
    AUDIO_EQUALIZATION_BAND_62 = 1,
    /** 2: 125 Hz */
    AUDIO_EQUALIZATION_BAND_125 = 2,
    /** 3: 250 Hz */
    AUDIO_EQUALIZATION_BAND_250 = 3,
    /** 4: 500 Hz */
    AUDIO_EQUALIZATION_BAND_500 = 4,
    /** 5: 1 kHz */
    AUDIO_EQUALIZATION_BAND_1K = 5,
    /** 6: 2 kHz */
    AUDIO_EQUALIZATION_BAND_2K = 6,
    /** 7: 4 kHz */
    AUDIO_EQUALIZATION_BAND_4K = 7,
    /** 8: 8 kHz */
    AUDIO_EQUALIZATION_BAND_8K = 8,
    /** 9: 16 kHz */
    AUDIO_EQUALIZATION_BAND_16K = 9,
}

/** Audio reverberation types. */
pub enum AUDIO_REVERB_TYPE {
    /** 0: The level of the dry signal (db). The value is between -20 and 10. */
    AUDIO_REVERB_DRY_LEVEL = 0,
    // (dB, [-20,10]), the level of the dry signal
    /** 1: The level of the early reflection signal (wet signal) (dB). The value is between -20 and 10. */
    AUDIO_REVERB_WET_LEVEL = 1,
    // (dB, [-20,10]), the level of the early reflection signal (wet signal)
    /** 2: The room size of the reflection. The value is between 0 and 100. */
    AUDIO_REVERB_ROOM_SIZE = 2,
    // ([0,100]), the room size of the reflection
    /** 3: The length of the initial delay of the wet signal (ms). The value is between 0 and 200. */
    AUDIO_REVERB_WET_DELAY = 3,
    // (ms, [0,200]), the length of the initial delay of the wet signal in ms
    /** 4: The reverberation strength. The value is between 0 and 100. */
    AUDIO_REVERB_STRENGTH = 4, // ([0,100]), the strength of the reverberation
}

/**
 * Local voice changer options.
 */
pub enum VOICE_CHANGER_PRESET {
    /**
     * The original voice (no local voice change).
     */
    VOICE_CHANGER_OFF = 0x00000000,
    //Turn off the voice changer
    /**
     * The voice of an old man.
     */
    VOICE_CHANGER_OLDMAN = 0x00000001,
    /**
     * The voice of a little boy.
     */
    VOICE_CHANGER_BABYBOY = 0x00000002,
    /**
     * The voice of a little girl.
     */
    VOICE_CHANGER_BABYGIRL = 0x00000003,
    /**
     * The voice of Zhu Bajie, a character in Journey to the West who has a voice like that of a growling bear.
     */
    VOICE_CHANGER_ZHUBAJIE = 0x00000004,
    /**
     * The ethereal voice.
     */
    VOICE_CHANGER_ETHEREAL = 0x00000005,
    /**
     * The voice of Hulk.
     */
    VOICE_CHANGER_HULK = 0x00000006,
    /**
     * A more vigorous voice.
     */
    VOICE_BEAUTY_VIGOROUS = 0x00100001,
    //7,
    /**
     * A deeper voice.
     */
    VOICE_BEAUTY_DEEP = 0x00100002,
    /**
     * A mellower voice.
     */
    VOICE_BEAUTY_MELLOW = 0x00100003,
    /**
     * Falsetto.
     */
    VOICE_BEAUTY_FALSETTO = 0x00100004,
    /**
     * A fuller voice.
     */
    VOICE_BEAUTY_FULL = 0x00100005,
    /**
     * A clearer voice.
     */
    VOICE_BEAUTY_CLEAR = 0x00100006,
    /**
     * A more resounding voice.
     */
    VOICE_BEAUTY_RESOUNDING = 0x00100007,
    /**
     * A more ringing voice.
     */
    VOICE_BEAUTY_RINGING = 0x00100008,
    /**
     * A more spatially resonant voice.
     */
    VOICE_BEAUTY_SPACIAL = 0x00100009,
    /**
     * (For male only) A more magnetic voice. Do not use it when the speaker is a female; otherwise, voice distortion occurs.
     */
    GENERAL_BEAUTY_VOICE_MALE_MAGNETIC = 0x00200001,
    /**
     * (For female only) A fresher voice. Do not use it when the speaker is a male; otherwise, voice distortion occurs.
     */
    GENERAL_BEAUTY_VOICE_FEMALE_FRESH = 0x00200002,
    /**
     * 	(For female only) A more vital voice. Do not use it when the speaker is a male; otherwise, voice distortion occurs.
     */
    GENERAL_BEAUTY_VOICE_FEMALE_VITALITY = 0x00200003,
}

/** Local voice reverberation presets. */
pub enum AUDIO_REVERB_PRESET {
    /**
     * Turn off local voice reverberation, that is, to use the original voice.
     */
    AUDIO_REVERB_OFF = 0x00000000,
    // Turn off audio reverb
    /**
     * The reverberation style typical of a KTV venue (enhanced).
     */
    AUDIO_REVERB_FX_KTV = 0x00100001,
    /**
     * The reverberation style typical of a concert hall (enhanced).
     */
    AUDIO_REVERB_FX_VOCAL_CONCERT = 0x00100002,
    /**
     * The reverberation style typical of an uncle's voice.
     */
    AUDIO_REVERB_FX_UNCLE = 0x00100003,
    /**
     * The reverberation style typical of a little sister's voice.
     */
    AUDIO_REVERB_FX_SISTER = 0x00100004,
    /**
     * The reverberation style typical of a recording studio (enhanced).
     */
    AUDIO_REVERB_FX_STUDIO = 0x00100005,
    /**
     * The reverberation style typical of popular music (enhanced).
     */
    AUDIO_REVERB_FX_POPULAR = 0x00100006,
    /**
     * The reverberation style typical of R&B music (enhanced).
     */
    AUDIO_REVERB_FX_RNB = 0x00100007,
    /**
     * The reverberation style typical of the vintage phonograph.
     */
    AUDIO_REVERB_FX_PHONOGRAPH = 0x00100008,
    /**
     * The reverberation style typical of popular music.
     */
    AUDIO_REVERB_POPULAR = 0x00000001,
    /**
     * The reverberation style typical of R&B music.
     */
    AUDIO_REVERB_RNB = 0x00000002,
    /**
     * The reverberation style typical of rock music.
     */
    AUDIO_REVERB_ROCK = 0x00000003,
    /**
     * The reverberation style typical of hip-hop music.
     */
    AUDIO_REVERB_HIPHOP = 0x00000004,
    /**
     * The reverberation style typical of a concert hall.
     */
    AUDIO_REVERB_VOCAL_CONCERT = 0x00000005,
    /**
     * The reverberation style typical of a KTV venue.
     */
    AUDIO_REVERB_KTV = 0x00000006,
    /**
     * The reverberation style typical of a recording studio.
     */
    AUDIO_REVERB_STUDIO = 0x00000007,
    /**
     * The reverberation of the virtual stereo. The virtual stereo is an effect that renders the monophonic
     * audio as the stereo audio, so that all users in the channel can hear the stereo voice effect.
     * To achieve better virtual stereo reverberation, Agora recommends setting `profile` in `setAudioProfile`
     * as `AUDIO_PROFILE_MUSIC_HIGH_QUALITY_STEREO(5)`.
     */
    AUDIO_VIRTUAL_STEREO = 0x00200001,
}

/** The use mode of the audio data in the \ref media::IAudioFrameObserver::onRecordAudioFrame "onRecordAudioFrame" or \ref media::IAudioFrameObserver::onPlaybackAudioFrame "onPlaybackAudioFrame" callback.
 */
pub enum RAW_AUDIO_FRAME_OP_MODE_TYPE {
    /** 0: Read-only mode: Users only read the \ref agora::media::IAudioFrameObserver::AudioFrame "AudioFrame" data without modifying anything. For example, when users acquire the data with the Agora SDK, then push the RTMP streams. */
    RAW_AUDIO_FRAME_OP_MODE_READ_ONLY = 0,
    /** 1: Write-only mode: Users replace the \ref agora::media::IAudioFrameObserver::AudioFrame "AudioFrame" data with their own data and pass the data to the SDK for encoding. For example, when users acquire the data. */
    RAW_AUDIO_FRAME_OP_MODE_WRITE_ONLY = 1,
    /** 2: Read and write mode: Users read the data from \ref agora::media::IAudioFrameObserver::AudioFrame "AudioFrame", modify it, and then play it. For example, when users have their own sound-effect processing module and perform some voice pre-processing, such as a voice change. */
    RAW_AUDIO_FRAME_OP_MODE_READ_WRITE = 2,
}


#[derive(Debug)]
pub struct AgoraRtcEngine {
    native_engine: agorartcnative::IRtcEngineBridge_ptr,
}

#[derive(Debug)]
pub struct AgoraRtcChannel {
    native_channel: agorartcnative::IRtcChannelBridge_ptr,
}

impl Drop for AgoraRtcChannel {
    fn drop(&mut self) {
        unsafe {
            agorartcnative::releaseChannel(self.native_channel);
        }
    }
}

unsafe impl std::marker::Sync for AgoraRtcChannel {}

#[derive(Debug)]
pub struct AudioPlaybackDeviceManager {
    native_playback_manager: agorartcnative::IAudioPlaybackDeviceManager_ptr,
}

impl Drop for AudioPlaybackDeviceManager {
    fn drop(&mut self) {
        unsafe {
            agorartcnative::releaseAudioPlaybackDeviceManager(self.native_playback_manager);
        }
    }
}

#[derive(Debug)]
pub struct AudioRecordingDeviceManager {
    native_recording_manager: agorartcnative::IAudioRecordingDeviceManager_ptr,
}

impl Drop for AudioRecordingDeviceManager {
    fn drop(&mut self) {
        unsafe {
            agorartcnative::releaseAudioRecordingDeviceManager(self.native_recording_manager);
        }
    }
}

#[derive(Debug)]
pub struct VideoDeviceManager {
    native_video_manager: agorartcnative::IVideoDeviceManager_ptr,
}

impl Drop for VideoDeviceManager {
    fn drop(&mut self) {
        unsafe {
            agorartcnative::releaseVideoDeviceManager(self.native_video_manager);
        }
    }
}

lazy_static! {
    #[derive(Debug)]
    pub static ref Agora_Rtc_Engine: AgoraRtcEngine = AgoraRtcEngine {
        native_engine: unsafe {agorartcnative::createRtcBridge()},
    };
}

unsafe impl std::marker::Sync for AgoraRtcEngine {}

impl AgoraRtcEngine {
    /** Creates and gets an AgoraRtcChannel instance.

    To join more than one channel, call this method multiple times to create as many AgoraRtcChannel objects as needed, and call the [joinChannelByToken]([AgoraRtcChannel joinChannelByToken:info:uid:options:]) method of each created AgoraRtcChannel instance.

    After joining multiple channels, you can simultaneously subscribe to streams of all the channels, but publish a stream in only one channel at one time.

    @param channelId The unique channel name for an Agora RTC session. It must be in the string format and not exceed 64 bytes in length. Supported character scopes are:

    - All lowercase English letters: a to z.
    - All uppercase English letters: A to Z.
    - All numeric characters: 0 to 9.
    - The space character.
    - Punctuation characters and other symbols, including: "!", "#", "$", "%", "&", "(", ")", "+", "-", ":", ";", "<", "=", ".", ">", "?", "@", "[", "]", "^", "_", " {", "}", "|", "~", ",".

    **Note**

    - This parameter does not have a default value. You must set it.
    - Do not set it as the empty string "". Otherwise, the SDK returns `AgoraErrorCodeRefused`(-5).

    @return - A pointer to the AgoraRtcChannel instance, if the method call succeeds.
    - An empty pointer, if the method call fails.
    - `AgoraErrorCodeRefused`(-5), if you set `channel_id` as the empty string "".
    */
    pub fn create_rtc_channel(&self, channel_id: &str) -> Box<AgoraRtcChannel> {
        unsafe {
            let the_channel_id: &CStr = &CString::new(channel_id).expect("channel_id new failed");
            let channel = agorartcnative::createChannel(self.native_engine, the_channel_id.as_ptr());
            let boxed_channel = Box::new(AgoraRtcChannel {
                native_channel: channel,
            });
            return boxed_channel;
        }
    }

    pub fn create_video_device_manager(&self) -> Box<VideoDeviceManager> {
        unsafe {
            let video_manager = agorartcnative::createVideoDeviceManager(self.native_engine);
            let boxed_video_manager = Box::new(VideoDeviceManager {
                native_video_manager: video_manager,
            });
            return boxed_video_manager;
        }
    }

    /** Initializes the Agora service.
     *
     * Unless otherwise specified, all the methods provided by the IRtcEngine class are executed asynchronously. Agora recommends calling these methods in the same thread.
     *
     * @note Ensure that you call the
     * \ref agora::rtc::IRtcEngine::createAgoraRtcEngine
     * "createAgoraRtcEngine" and \ref agora::rtc::IRtcEngine::initialize
     * "initialize" methods before calling any other APIs.
     *
     * @param context Pointer to the RTC engine context. See RtcEngineContext.
     *
     * @return
     * - 0(ERR_OK): Success.
     * - < 0: Failure.
     *  - -1(ERR_FAILED): A general error occurs (no specified reason).
     *  - -2(ERR_INALID_ARGUMENT): No `IRtcEngineEventHandler` object is specified.
     *  - -7(ERR_NOT_INITIALIZED): The SDK is not initialized. Check whether `context` is properly set.
     *  - -22(ERR_RESOURCE_LIMITED): The resource is limited. The app uses too much of the system resource and fails to allocate any resources.
     *  - -101(ERR_INVALID_APP_ID): The App ID is invalid.
     */
    pub fn initialize(&self, app_id: &str, area_code: AREA_CODE) -> i32 {
        unsafe {
            let the_app_id: &CStr = &CString::new(app_id).expect("app_id new failed");
            return agorartcnative::initialize(self.native_engine, the_app_id.as_ptr(), std::ptr::null_mut(), area_code as u32);
        }
    }

    pub fn add_event_handler(&self, handler: *mut agorartcnative::RtcEventHandler) {
        unsafe {
            agorartcnative::add_C_EventHandler(self.native_engine, handler);
        }
    }

    pub fn remove_event_handler(&self) {
        unsafe {
            agorartcnative::remove_C_EventHandler(self.native_engine);
        }
    }

    /** Releases all IRtcEngine resources.
     *
     * Use this method for apps in which users occasionally make voice or video calls. When users do not make calls, you
     * can free up resources for other operations. Once you call `release` to destroy the created `IRtcEngine` instance,
     * you cannot use any method or callback in the SDK any more. If you want to use the real-time communication functions
     * again, you must call \ref createAgoraRtcEngine "createAgoraRtcEngine" and \ref agora::rtc::IRtcEngine::initialize "initialize"
     * to create a new `IRtcEngine` instance.
     *
     * @note If you want to create a new `IRtcEngine` instance after destroying the current one, ensure that you wait
     * till the `release` method completes executing.
     *
     * @param sync
     * - true: Synchronous call. Agora suggests calling this method in a sub-thread to avoid congestion in the main thread
     * because the synchronous call and the app cannot move on to another task until the execution completes.
     * Besides, you **cannot** call this method in any method or callback of the SDK. Otherwise, the SDK cannot release the
     * resources occupied by the `IRtcEngine` instance until the callbacks return results, which may result in a deadlock.
     * The SDK automatically detects the deadlock and converts this method into an asynchronous call, causing the test to
     * take additional time.
     * - false: Asynchronous call. Do not immediately uninstall the SDK's dynamic library after the call, or it may cause
     * a crash due to the SDK clean-up thread not quitting.
     */
    pub fn release(&self, sync: bool) {
        unsafe {
            let is_sync: i32 = if sync { 1 } else { 0 };
            agorartcnative::release(self.native_engine, is_sync);
        }
    }

    /** Sets the channel profile of the Agora IRtcEngine.
     *
     * The Agora IRtcEngine differentiates channel profiles and applies optimization algorithms accordingly.
     * For example, it prioritizes smoothness and low latency for a video call, and prioritizes video quality for the live interactive video streaming.
     *
     * @warning
     * - To ensure the quality of real-time communication, we recommend that all users in a channel use the same channel profile.
     * - Call this method before calling \ref IRtcEngine::joinChannel "joinChannel" . You cannot set the channel profile once you have joined the channel.
     * - The default audio route and video encoding bitrate are different in different channel profiles. For details, see
     * \ref IRtcEngine::setDefaultAudioRouteToSpeakerphone "setDefaultAudioRouteToSpeakerphone" and \ref IRtcEngine::setVideoEncoderConfiguration "setVideoEncoderConfiguration".
     *
     * @param profile The channel profile of the Agora IRtcEngine. See #CHANNEL_PROFILE_TYPE
     * @return
     * - 0(ERR_OK): Success.
     * - < 0: Failure.
     *  - -2 (ERR_INVALID_ARGUMENT): The parameter is invalid.
     *  - -7(ERR_NOT_INITIALIZED): The SDK is not initialized.
     */
    pub fn set_channel_profile(&self, profile: CHANNEL_PROFILE_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setChannelProfile(self.native_engine, profile as u32);
        }
    }

    /** Sets the role of the user, such as a host or an audience (default), before joining a channel in the live interactive streaming.
     *
     * This method can be used to switch the user role in the live interactive streaming after the user joins a channel.
     *
     * In the `LIVE_BROADCASTING` profile, when a user switches user roles after joining a channel, a successful \ref agora::rtc::IRtcEngine::setClientRole "setClientRole" method call triggers the following callbacks:
     * - The local client: \ref agora::rtc::IRtcEngineEventHandler::onClientRoleChanged "onClientRoleChanged"
     * - The remote client: \ref agora::rtc::IRtcEngineEventHandler::onUserJoined "onUserJoined" or \ref agora::rtc::IRtcEngineEventHandler::onUserOffline "onUserOffline" (BECOME_AUDIENCE)
     *
     * @note
     * This method applies only to the `LIVE_BROADCASTING` profile.
     *
     * @param role Sets the role of the user. See #CLIENT_ROLE_TYPE.
     *
     * @return
     * - 0(ERR_OK): Success.
     * - < 0: Failure.
     *  - -1(ERR_FAILED): A general error occurs (no specified reason).
     *  - -2(ERR_INALID_ARGUMENT): The parameter is invalid.
     *  - -7(ERR_NOT_INITIALIZED): The SDK is not initialized.
     */
    pub fn set_client_role(&self, role: CLIENT_ROLE_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setClientRole(self.native_engine, role as u32);
        }
    }

    /** Joins a channel with the user ID.

     Users in the same channel can talk to each other, and multiple users in the same channel can start a group chat. Users with different App IDs cannot call each other.


     You must call the \ref IRtcEngine::leaveChannel "leaveChannel" method to exit the current call before entering another channel.

     A successful \ref agora::rtc::IRtcEngine::joinChannel "joinChannel" method call triggers the following callbacks:
     - The local client: \ref agora::rtc::IRtcEngineEventHandler::onJoinChannelSuccess "onJoinChannelSuccess"
     - The remote client: \ref agora::rtc::IRtcEngineEventHandler::onUserJoined "onUserJoined" , if the user joining the channel is in the `COMMUNICATION` profile, or is a host in the `LIVE_BROADCASTING` profile.

     When the connection between the client and Agora's server is interrupted due to poor network conditions, the SDK tries reconnecting to the server. When the local client successfully rejoins the channel, the SDK triggers the \ref agora::rtc::IRtcEngineEventHandler::onRejoinChannelSuccess "onRejoinChannelSuccess" callback on the local client.

     @note A channel does not accept duplicate uids, such as two users with the same @p uid. If you set @p uid as 0, the system automatically assigns a @p uid. If you want to join a channel from different devices, ensure that each device has a different uid.
     @warning Ensure that the App ID used for creating the token is the same App ID used by the \ref IRtcEngine::initialize "initialize" method for initializing the RTC engine. Otherwise, the CDN live streaming may fail.

     @param token Pointer to the token generated by the application server. In most circumstances, a static App ID suffices. For added security, use a Channel Key.
     - If the user uses a static App ID, *token* is optional and can be set as NULL.
     - If the user uses a Channel Key, Agora issues an additional App Certificate for you to generate a user key based on the algorithm and App Certificate for user authentication on the server.
     @param channelId Pointer to the unique channel name for the Agora RTC session in the string format smaller than 64 bytes. Supported characters:
     - All lowercase English letters: a to z.
     - All uppercase English letters: A to Z.
     - All numeric characters: 0 to 9.
     - The space character.
     - Punctuation characters and other symbols, including: "!", "#", "$", "%", "&", "(", ")", "+", "-", ":", ";", "<", "=", ".", ">", "?", "@", "[", "]", "^", "_", " {", "}", "|", "~", ",".
     @param info (Optional) Pointer to additional information about the channel. This parameter can be set to NULL or contain channel related information. Other users in the channel will not receive this message.
     @param uid (Optional) User ID. A 32-bit unsigned integer with a value ranging from 1 to 2<sup>32</sup>-1. The @p uid must be unique. If a @p uid is not assigned (or set to 0), the SDK assigns and returns a @p uid in the \ref IRtcEngineEventHandler::onJoinChannelSuccess "onJoinChannelSuccess" callback. Your application must record and maintain the returned *uid* since the SDK does not do so.

     @return
     - 0(ERR_OK): Success.
     - < 0: Failure.
        - -2(ERR_INALID_ARGUMENT): The parameter is invalid.
        - -3(ERR_NOT_READY): The SDK fails to be initialized. You can try re-initializing the SDK.
        - -5(ERR_REFUSED): The request is rejected. This may be caused by the following:
            - You have created an IChannel object with the same channel name.
            - You have joined and published a stream in a channel created by the IChannel object.
     */
    pub fn join_channel(&self, token: &str, channel_id: &str, info: &str, uid: u32) -> i32 {
        unsafe {
            let the_token: &CStr = &CString::new(token).expect("token new failed");
            let the_channel_id: &CStr = &CString::new(channel_id).expect("channel_id new failed");
            let the_info: &CStr = &CString::new(info).expect("info new failed");
            return agorartcnative::joinChannel(self.native_engine, the_token.as_ptr(), the_channel_id.as_ptr(), the_info.as_ptr(), uid);
        }
    }

    /** Switches to a different channel.
     *
     * This method allows the audience of a `LIVE_BROADCASTING` channel to switch
     * to a different channel.
     *
     * After the user successfully switches to another channel, the
     * \ref agora::rtc::IRtcEngineEventHandler::onLeaveChannel "onLeaveChannel"
     *  and \ref agora::rtc::IRtcEngineEventHandler::onJoinChannelSuccess
     * "onJoinChannelSuccess" callbacks are triggered to indicate that the
     * user has left the original channel and joined a new one.
     *
     * @note
     * This method applies to the audience role in a `LIVE_BROADCASTING` channel
     * only.
     *
     * @param token The token generated at your server:
     * - For low-security requirements: You can use the temporary token
     * generated in Console. For details, see
     * [Get a temporary token](https://docs.agora.io/en/Agora%20Platform/token?platfor%20*%20m=All%20Platforms#get-a-temporary-token).
     * - For high-security requirements: Use the token generated at your
     * server. For details, see
     * [Get a token](https://docs.agora.io/en/Agora%20Platform/token?platfor%20*%20m=All%20Platforms#get-a-token).
     * @param channelId Unique channel name for the AgoraRTC session in the
     * string format. The string length must be less than 64 bytes. Supported
     * character scopes are:
     * - All lowercase English letters: a to z.
     * - All uppercase English letters: A to Z.
     * - All numeric characters: 0 to 9.
     * - The space character.
     * - Punctuation characters and other symbols, including: "!", "#", "$", "%", "&", "(", ")", "+", "-", ":", ";", "<", "=", ".", ">", "?", "@", "[", "]", "^", "_", " {", "}", "|", "~", ",".
     *
     * @return
     * - 0(ERR_OK): Success.
     * - < 0: Failure.
     *  - -1(ERR_FAILED): A general error occurs (no specified reason).
     *  - -2(ERR_INALID_ARGUMENT): The parameter is invalid.
     *  - -5(ERR_REFUSED): The request is rejected, probably because the user is not an audience.
     *  - -7(ERR_NOT_INITIALIZED): The SDK is not initialized.
     *  - -102(ERR_INVALID_CHANNEL_NAME): The channel name is invalid.
     *  - -113(ERR_NOT_IN_CHANNEL): The user is not in the channel.
     */
    pub fn switch_channel(&self, token: &str, channel_id: &str) -> i32 {
        unsafe {
            let the_token: &CStr = &CString::new(token).expect("token new failed");
            let the_channel_id: &CStr = &CString::new(channel_id).expect("channel_id new failed");
            return agorartcnative::switchChannel(self.native_engine, the_token.as_ptr(), the_channel_id.as_ptr());
        }
    }

    /** Allows a user to leave a channel, such as hanging up or exiting a call.

     After joining a channel, the user must call the *leaveChannel* method to end the call before joining another channel.

     This method returns 0 if the user leaves the channel and releases all resources related to the call.

     This method call is asynchronous, and the user has not left the channel when the method call returns. Once the user leaves the channel, the SDK triggers the \ref IRtcEngineEventHandler::onLeaveChannel "onLeaveChannel" callback.

     A successful \ref agora::rtc::IRtcEngine::leaveChannel "leaveChannel" method call triggers the following callbacks:
     - The local client: \ref agora::rtc::IRtcEngineEventHandler::onLeaveChannel "onLeaveChannel"
     - The remote client: \ref agora::rtc::IRtcEngineEventHandler::onUserOffline "onUserOffline" , if the user leaving the channel is in the `COMMUNICATION` channel, or is a host in the `LIVE_BROADCASTING` profile.

     @note
     - If you call the \ref IRtcEngine::release "release" method immediately after the *leaveChannel* method, the *leaveChannel* process interrupts, and the \ref IRtcEngineEventHandler::onLeaveChannel "onLeaveChannel" callback is not triggered.
     - If you call the *leaveChannel* method during a CDN live streaming, the SDK triggers the \ref IRtcEngine::removePublishStreamUrl "removePublishStreamUrl" method.

     @return
     - 0(ERR_OK): Success.
     - < 0: Failure.
        - -1(ERR_FAILED): A general error occurs (no specified reason).
        - -2(ERR_INALID_ARGUMENT): The parameter is invalid.
        - -7(ERR_NOT_INITIALIZED): The SDK is not initialized.
     */
    pub fn leave_channel(&self) -> i32 {
        unsafe {
            return agorartcnative::leaveChannel(self.native_engine);
        }
    }

    /** Gets a new token when the current token expires after a period of time.

     The `token` expires after a period of time once the token schema is enabled when:

     - The SDK triggers the \ref IRtcEngineEventHandler::onTokenPrivilegeWillExpire "onTokenPrivilegeWillExpire" callback, or
     - The \ref IRtcEngineEventHandler::onConnectionStateChanged "onConnectionStateChanged" reports CONNECTION_CHANGED_TOKEN_EXPIRED(9).

     The application should call this method to get the new `token`. Failure to do so will result in the SDK disconnecting from the server.

     @param token Pointer to the new token.

     @return
     - 0(ERR_OK): Success.
     - < 0: Failure.
        - -1(ERR_FAILED): A general error occurs (no specified reason).
        - -2(ERR_INALID_ARGUMENT): The parameter is invalid.
        - -7(ERR_NOT_INITIALIZED): The SDK is not initialized.
     */
    pub fn renew_token(&self, token: &str) -> i32 {
        unsafe {
            let the_token: &CStr = &CString::new(token).expect("token new failed");
            return agorartcnative::renewToken(self.native_engine, the_token.as_ptr());
        }
    }

    /** Registers a user account.

     Once registered, the user account can be used to identify the local user when the user joins the channel.
     After the user successfully registers a user account, the SDK triggers the \ref agora::rtc::IRtcEngineEventHandler::onLocalUserRegistered "onLocalUserRegistered" callback on the local client,
     reporting the user ID and user account of the local user.

     To join a channel with a user account, you can choose either of the following:

     - Call the \ref agora::rtc::IRtcEngine::registerLocalUserAccount "registerLocalUserAccount" method to create a user account, and then the \ref agora::rtc::IRtcEngine::joinChannelWithUserAccount "joinChannelWithUserAccount" method to join the channel.
     - Call the \ref agora::rtc::IRtcEngine::joinChannelWithUserAccount "joinChannelWithUserAccount" method to join the channel.

     The difference between the two is that for the former, the time elapsed between calling the \ref agora::rtc::IRtcEngine::joinChannelWithUserAccount "joinChannelWithUserAccount" method
     and joining the channel is shorter than the latter.

     @note
     - Ensure that you set the `userAccount` parameter. Otherwise, this method does not take effect.
     - Ensure that the value of the `userAccount` parameter is unique in the channel.
     - To ensure smooth communication, use the same parameter type to identify the user. For example, if a user joins the channel with a user ID, then ensure all the other users use the user ID too. The same applies to the user account. If a user joins the channel with the Agora Web SDK, ensure that the uid of the user is set to the same parameter type.

     @param appId The App ID of your project.
     @param userAccount The user account. The maximum length of this parameter is 255 bytes. Ensure that you set this parameter and do not set it as null. Supported character scopes are:
     - All lowercase English letters: a to z.
     - All uppercase English letters: A to Z.
     - All numeric characters: 0 to 9.
     - The space character.
     - Punctuation characters and other symbols, including: "!", "#", "$", "%", "&", "(", ")", "+", "-", ":", ";", "<", "=", ".", ">", "?", "@", "[", "]", "^", "_", " {", "}", "|", "~", ",".

     @return
     - 0: Success.
     - < 0: Failure.
    */
    pub fn register_local_user_account(&self, app_id: &str, user_account: &str) -> i32 {
        unsafe {
            let the_app_id = &CString::new(app_id).expect("app_id new failed");
            let the_user_account = &CString::new(user_account).expect("user_account new failed");
            return agorartcnative::registerLocalUserAccount(self.native_engine, the_app_id.as_ptr(), the_user_account.as_ptr());
        }
    }

    /** Joins the channel with a user account.

     After the user successfully joins the channel, the SDK triggers the following callbacks:

     - The local client: \ref agora::rtc::IRtcEngineEventHandler::onLocalUserRegistered "onLocalUserRegistered" and \ref agora::rtc::IRtcEngineEventHandler::onJoinChannelSuccess "onJoinChannelSuccess" .
     The remote client: \ref agora::rtc::IRtcEngineEventHandler::onUserJoined "onUserJoined" and \ref agora::rtc::IRtcEngineEventHandler::onUserInfoUpdated "onUserInfoUpdated" , if the user joining the channel is in the `COMMUNICATION` profile, or is a host in the `LIVE_BROADCASTING` profile.

     @note To ensure smooth communication, use the same parameter type to identify the user. For example, if a user joins the channel with a user ID, then ensure all the other users use the user ID too. The same applies to the user account.
     If a user joins the channel with the Agora Web SDK, ensure that the uid of the user is set to the same parameter type.

     @param token The token generated at your server:
     - For low-security requirements: You can use the temporary token generated at Console. For details, see [Get a temporary toke](https://docs.agora.io/en/Voice/token?platform=All%20Platforms#get-a-temporary-token).
     - For high-security requirements: Set it as the token generated at your server. For details, see [Get a token](https://docs.agora.io/en/Voice/token?platform=All%20Platforms#get-a-token).
     @param channelId The channel name. The maximum length of this parameter is 64 bytes. Supported character scopes are:
     - All lowercase English letters: a to z.
     - All uppercase English letters: A to Z.
     - All numeric characters: 0 to 9.
     - The space character.
     - Punctuation characters and other symbols, including: "!", "#", "$", "%", "&", "(", ")", "+", "-", ":", ";", "<", "=", ".", ">", "?", "@", "[", "]", "^", "_", " {", "}", "|", "~", ",".
     @param userAccount The user account. The maximum length of this parameter is 255 bytes. Ensure that you set this parameter and do not set it as null. Supported character scopes are:
     - All lowercase English letters: a to z.
     - All uppercase English letters: A to Z.
     - All numeric characters: 0 to 9.
     - The space character.
     - Punctuation characters and other symbols, including: "!", "#", "$", "%", "&", "(", ")", "+", "-", ":", ";", "<", "=", ".", ">", "?", "@", "[", "]", "^", "_", " {", "}", "|", "~", ",".

     @return
     - 0: Success.
     - < 0: Failure.
        - #ERR_INVALID_ARGUMENT (-2)
        - #ERR_NOT_READY (-3)
        - #ERR_REFUSED (-5)
     */
    pub fn join_channel_with_user_account(&self, token: &str, channel_id: &str, user_account: &str) -> i32 {
        unsafe {
            let the_token: &CStr = &CString::new(token).expect("token new failed");
            let the_channel_id: &CStr = &CString::new(channel_id).expect("channel_id new failed");
            let the_user_account = &CString::new(user_account).expect("user_account new failed");
            return agorartcnative::joinChannelWithUserAccount(self.native_engine, the_token.as_ptr(), the_channel_id.as_ptr(), the_user_account.as_ptr());
        }
    }

    /** Gets the user information by passing in the user account.

     After a remote user joins the channel, the SDK gets the user ID and user account of the remote user, caches them
     in a mapping table object (`userInfo`), and triggers the \ref agora::rtc::IRtcEngineEventHandler::onUserInfoUpdated "onUserInfoUpdated" callback on the local client.

     After receiving the o\ref agora::rtc::IRtcEngineEventHandler::onUserInfoUpdated "onUserInfoUpdated" callback, you can call this method to get the user ID of the
     remote user from the `userInfo` object by passing in the user account.

     @param userAccount The user account of the user. Ensure that you set this parameter.
     @param [in,out] userInfo  A userInfo object that identifies the user:
     - Input: A userInfo object.
     - Output: A userInfo object that contains the user account and user ID of the user.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn get_user_info_by_user_account(&self, user_account: &str, user_info: *mut agorartcnative::UserInfo) -> i32 {
        unsafe {
            let the_user_account = &CString::new(user_account).expect("user_account new failed");
            return agorartcnative::getUserInfoByUserAccount(self.native_engine, the_user_account.as_ptr(), user_info);
        }
    }

    /** Gets the user information by passing in the user ID.

     After a remote user joins the channel, the SDK gets the user ID and user account of the remote user,
     caches them in a mapping table object (`userInfo`), and triggers the \ref agora::rtc::IRtcEngineEventHandler::onUserInfoUpdated "onUserInfoUpdated" callback on the local client.

     After receiving the \ref agora::rtc::IRtcEngineEventHandler::onUserInfoUpdated "onUserInfoUpdated" callback, you can call this method to get the user account of the remote user
     from the `userInfo` object by passing in the user ID.

     @param uid The user ID of the remote user. Ensure that you set this parameter.
     @param[in,out] userInfo A userInfo object that identifies the user:
     - Input: A userInfo object.
     - Output: A userInfo object that contains the user account and user ID of the user.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn get_user_info_by_uid(&self, uid: u32, user_info: *mut agorartcnative::UserInfo) -> i32 {
        unsafe {
            return agorartcnative::getUserInfoByUid(self.native_engine, uid, user_info);
        }
    }

    /** **DEPRECATED** Starts an audio call test.

     This method is deprecated as of v2.4.0.

     This method starts an audio call test to check whether the audio devices (for example, headset and speaker) and the network connection are working properly.

     To conduct the test:

     - The user speaks and the recording is played back within 10 seconds.
     - If the user can hear the recording within 10 seconds, the audio devices and network connection are working properly.

     @note
     - After calling this method, always call the \ref IRtcEngine::stopEchoTest "stopEchoTest" method to end the test. Otherwise, the application cannot run the next echo test.
     - In the `LIVE_BROADCASTING` profile, only the hosts can call this method. If the user switches from the `COMMUNICATION` to`LIVE_BROADCASTING` profile, the user must call the \ref IRtcEngine::setClientRole "setClientRole" method to change the user role from the audience (default) to the host before calling this method.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn start_echo_test(&self) -> i32 {
        unsafe {
            return agorartcnative::startEchoTest(self.native_engine);
        }
    }

    /** Starts an audio call test.

    This method starts an audio call test to determine whether the audio devices (for example, headset and speaker) and the network connection are working properly.

    In the audio call test, you record your voice. If the recording plays back within the set time interval, the audio devices and the network connection are working properly.

    @note
    - Call this method before joining a channel.
    - After calling this method, call the \ref IRtcEngine::stopEchoTest "stopEchoTest" method to end the test. Otherwise, the app cannot run the next echo test, or call the \ref IRtcEngine::joinChannel "joinChannel" method.
    - In the `LIVE_BROADCASTING` profile, only a host can call this method.
    @param intervalInSeconds The time interval (s) between when you speak and when the recording plays back.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn start_echo_test2(&self, interval_in_seconds: i32) -> i32 {
        unsafe {
            return agorartcnative::startEchoTest2(self.native_engine, interval_in_seconds);
        }
    }

    /** Stops the audio call test.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn stop_echo_test(&self) -> i32 {
        unsafe {
            return agorartcnative::stopEchoTest(self.native_engine);
        }
    }

    /** Enables the video module.

     Call this method either before joining a channel or during a call. If this method is called before joining a channel, the call starts in the video mode. If this method is called during an audio call, the audio mode switches to the video mode. To disable the video module, call the \ref IRtcEngine::disableVideo "disableVideo" method.

     A successful \ref agora::rtc::IRtcEngine::enableVideo "enableVideo" method call triggers the \ref agora::rtc::IRtcEngineEventHandler::onUserEnableVideo "onUserEnableVideo" (true) callback on the remote client.
     @note
     - This method affects the internal engine and can be called after the \ref agora::rtc::IRtcEngine::leaveChannel "leaveChannel" method.
     - This method resets the internal engine and takes some time to take effect. We recommend using the following API methods to control the video engine modules separately:
         - \ref IRtcEngine::enableLocalVideo "enableLocalVideo": Whether to enable the camera to create the local video stream.
         - \ref IRtcEngine::muteLocalVideoStream "muteLocalVideoStream": Whether to publish the local video stream.
         - \ref IRtcEngine::muteRemoteVideoStream "muteRemoteVideoStream": Whether to subscribe to and play the remote video stream.
         - \ref IRtcEngine::muteAllRemoteVideoStreams "muteAllRemoteVideoStreams": Whether to subscribe to and play all remote video streams.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn enable_video(&self) -> i32 {
        unsafe {
            return agorartcnative::enableVideo(self.native_engine);
        }
    }

    /** Disables the video module.

    This method can be called before joining a channel or during a call. If this method is called before joining a channel, the call starts in audio mode. If this method is called during a video call, the video mode switches to the audio mode. To enable the video module, call the \ref IRtcEngine::enableVideo "enableVideo" method.

    A successful \ref agora::rtc::IRtcEngine::disableVideo "disableVideo" method call triggers the \ref agora::rtc::IRtcEngineEventHandler::onUserEnableVideo "onUserEnableVideo" (false) callback on the remote client.
     @note
     - This method affects the internal engine and can be called after the \ref agora::rtc::IRtcEngine::leaveChannel "leaveChannel" method.
     - This method resets the internal engine and takes some time to take effect. We recommend using the following API methods to control the video engine modules separately:
         - \ref IRtcEngine::enableLocalVideo "enableLocalVideo": Whether to enable the camera to create the local video stream.
         - \ref IRtcEngine::muteLocalVideoStream "muteLocalVideoStream": Whether to publish the local video stream.
         - \ref IRtcEngine::muteRemoteVideoStream "muteRemoteVideoStream": Whether to subscribe to and play the remote video stream.
         - \ref IRtcEngine::muteAllRemoteVideoStreams "muteAllRemoteVideoStreams": Whether to subscribe to and play all remote video streams.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn disable_video(&self) -> i32 {
        unsafe {
            return agorartcnative::disableVideo(self.native_engine);
        }
    }

    /** **DEPRECATED** Sets the video profile.

     This method is deprecated as of v2.3. Use the \ref IRtcEngine::setVideoEncoderConfiguration "setVideoEncoderConfiguration" method instead.

     Each video profile includes a set of parameters, such as the resolution, frame rate, and bitrate. If the camera device does not support the specified resolution, the SDK automatically chooses a suitable camera resolution, keeping the encoder resolution specified by the *setVideoProfile* method.

     @note
     - If you do not need to set the video profile after joining the channel, call this method before the \ref IRtcEngine::enableVideo "enableVideo" method to reduce the render time of the first video frame.
     - Always set the video profile before calling the \ref IRtcEngine::joinChannel "joinChannel" or \ref IRtcEngine::startPreview "startPreview" method.

     @param profile Sets the video profile. See #VIDEO_PROFILE_TYPE.
     @param swapWidthAndHeight Sets whether to swap the width and height of the video stream:
     - true: Swap the width and height.
     - false: (Default) Do not swap the width and height.
     The width and height of the output video are consistent with the set video profile.
     @note Since the landscape or portrait mode of the output video can be decided directly by the video profile, We recommend setting *swapWidthAndHeight* to *false* (default).

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_video_profile(&self, profile: VIDEO_PROFILE_TYPE, swap_width_and_height: bool) -> i32 {
        unsafe {
            let swap: i32 = if swap_width_and_height { 1 } else { 0 };
            return agorartcnative::setVideoProfile(self.native_engine, profile as u32, swap);
        }
    }

    /** Sets the video encoder configuration.

     Each video encoder configuration corresponds to a set of video parameters, including the resolution, frame rate, bitrate, and video orientation.

     The parameters specified in this method are the maximum values under ideal network conditions. If the video engine cannot render the video using the specified parameters due to poor network conditions, the parameters further down the list are considered until a successful configuration is found.

     @note If you do not need to set the video encoder configuration after joining the channel, you can call this method before the \ref IRtcEngine::enableVideo "enableVideo" method to reduce the render time of the first video frame.

     @param config Sets the local video encoder configuration. See VideoEncoderConfiguration.
     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_video_encoder_configuation(&self, config: agorartcnative::VideoEncoderConfiguration) -> i32 {
        unsafe {
            return agorartcnative::setVideoEncoderConfiguration(self.native_engine, config);
        }
    }

    /** Sets the camera capture configuration.

     For a video call or the live interactive video streaming, generally the SDK controls the camera output parameters. When the default camera capturer settings do not meet special requirements or cause performance problems, we recommend using this method to set the camera capturer configuration:

     - If the resolution or frame rate of the captured raw video data are higher than those set by \ref IRtcEngine::setVideoEncoderConfiguration "setVideoEncoderConfiguration", processing video frames requires extra CPU and RAM usage and degrades performance. We recommend setting config as CAPTURER_OUTPUT_PREFERENCE_PERFORMANCE = 1 to avoid such problems.
     - If you do not need local video preview or are willing to sacrifice preview quality, we recommend setting config as CAPTURER_OUTPUT_PREFERENCE_PERFORMANCE = 1 to optimize CPU and RAM usage.
     - If you want better quality for the local video preview, we recommend setting config as CAPTURER_OUTPUT_PREFERENCE_PREVIEW = 2.

     @note Call this method before enabling the local camera. That said, you can call this method before calling \ref agora::rtc::IRtcEngine::joinChannel "joinChannel", \ref agora::rtc::IRtcEngine::enableVideo "enableVideo", or \ref IRtcEngine::enableLocalVideo "enableLocalVideo", depending on which method you use to turn on your local camera.

     @param config Sets the camera capturer configuration. See CameraCapturerConfiguration.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_camera_capture_configuration(&self, config: agorartcnative::CameraCapturerConfiguration) -> i32 {
        unsafe {
            return agorartcnative::setCameraCapturerConfiguration(self.native_engine, config);
        }
    }

    /** Initializes the local video view.

     This method initializes the video view of a local stream on the local device. It affects only the video view that the local user sees, not the published local video stream.

     Call this method to bind the local video stream to a video view and to set the rendering and mirror modes of the video view.
     The binding is still valid after the user leaves the channel, which means that the window still displays. To unbind the view, set the *view* in VideoCanvas to NULL.

     @note
     - Call this method before joining a channel.
     - During a call, you can call this method as many times as necessary to update the display mode of the local video view.
     @param canvas Pointer to the local video view and settings. See VideoCanvas.
     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn setup_local_video(&self, canvas: agorartcnative::VideoCanvas) -> i32 {
        unsafe {
            return agorartcnative::setupLocalVideo(self.native_engine, canvas);
        }
    }

    /** Initializes the video view of a remote user.

     This method initializes the video view of a remote stream on the local device. It affects only the video view that the local user sees.

     Call this method to bind the remote video stream to a video view and to set the rendering and mirror modes of the video view.

     The application specifies the uid of the remote video in this method before the remote user joins the channel. If the remote uid is unknown to the application, set it after the application receives the \ref IRtcEngineEventHandler::onUserJoined "onUserJoined" callback.
     If the Video Recording function is enabled, the Video Recording Service joins the channel as a dummy client, causing other clients to also receive the \ref IRtcEngineEventHandler::onUserJoined "onUserJoined" callback. Do not bind the dummy client to the application view because the dummy client does not send any video streams. If your application does not recognize the dummy client, bind the remote user to the view when the SDK triggers the \ref IRtcEngineEventHandler::onFirstRemoteVideoDecoded "onFirstRemoteVideoDecoded" callback.
     To unbind the remote user from the view, set the view in VideoCanvas to NULL. Once the remote user leaves the channel, the SDK unbinds the remote user.

     @note To update the rendering or mirror mode of the remote video view during a call, use the \ref IRtcEngine::setRemoteRenderMode "setRemoteRenderMode" method.

     @param canvas Pointer to the remote video view and settings. See VideoCanvas.
     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn setup_remote_video(&self, canvas: agorartcnative::VideoCanvas) -> i32 {
        unsafe {
            return agorartcnative::setupRemoteVideo(self.native_engine, canvas);
        }
    }

    /** Starts the local video preview before joining the channel.

     Before calling this method, you must:

     - Call the \ref IRtcEngine::setupLocalVideo "setupLocalVideo" method to set up the local preview window and configure the attributes.
     - Call the \ref IRtcEngine::enableVideo "enableVideo" method to enable video.

     @note Once the startPreview method is called to start the local video preview, if you leave the channel by calling the \ref IRtcEngine::leaveChannel "leaveChannel" method, the local video preview remains until you call the \ref IRtcEngine::stopPreview "stopPreview" method to disable it.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn start_preview(&self) -> i32 {
        unsafe {
            return agorartcnative::startPreview(self.native_engine);
        }
    }

    /** Prioritizes a remote user's stream.

    Use this method with the \ref IRtcEngine::setRemoteSubscribeFallbackOption "setRemoteSubscribeFallbackOption" method. If the fallback function is enabled for a subscribed stream, the SDK ensures the high-priority user gets the best possible stream quality.

    @note The Agora SDK supports setting @p userPriority as high for one user only.

    @param  uid  The ID of the remote user.
    @param  userPriority Sets the priority of the remote user. See #PRIORITY_TYPE.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_remote_user_priority(&self, uid: u32, user_priority: PRIORITY_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setRemoteUserPriority(self.native_engine, uid, user_priority as u32);
        }
    }

    /** Stops the local video preview and disables video.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn stop_preview(&self) -> i32 {
        unsafe {
            return agorartcnative::stopPreview(self.native_engine);
        }
    }

    /** Enables the audio module.

    The audio mode is enabled by default.

     @note
     - This method affects the internal engine and can be called after the \ref agora::rtc::IRtcEngine::leaveChannel "leaveChannel" method. You can call this method either before or after joining a channel.
     - This method resets the internal engine and takes some time to take effect. We recommend using the following API methods to control the audio engine modules separately:
         - \ref IRtcEngine::enableLocalAudio "enableLocalAudio": Whether to enable the microphone to create the local audio stream.
         - \ref IRtcEngine::muteLocalAudioStream "muteLocalAudioStream": Whether to publish the local audio stream.
         - \ref IRtcEngine::muteRemoteAudioStream "muteRemoteAudioStream": Whether to subscribe to and play the remote audio stream.
         - \ref IRtcEngine::muteAllRemoteAudioStreams "muteAllRemoteAudioStreams": Whether to subscribe to and play all remote audio streams.

    @return
    - 0: Success.
    - < 0: Failure.
    */
    pub fn enable_audio(&self) -> i32 {
        unsafe {
            return agorartcnative::enableAudio(self.native_engine);
        }
    }

    /** Disables/Re-enables the local audio function.

     The audio function is enabled by default. This method disables or re-enables the local audio function, that is, to stop or restart local audio capturing.

     This method does not affect receiving or playing the remote audio streams,and enableLocalAudio(false) is applicable to scenarios where the user wants to
     receive remote audio streams without sending any audio stream to other users in the channel.

     Once the local audio function is disabled or re-enabled, the SDK triggers the \ref agora::rtc::IRtcEngineEventHandler::onLocalAudioStateChanged "onLocalAudioStateChanged" callback,
     which reports `LOCAL_AUDIO_STREAM_STATE_STOPPED(0)` or `LOCAL_AUDIO_STREAM_STATE_RECORDING(1)`.

     @note
     This method is different from the \ref agora::rtc::IRtcEngine::muteLocalAudioStream "muteLocalAudioStream" method:
        - \ref agora::rtc::IRtcEngine::enableLocalAudio "enableLocalAudio": Disables/Re-enables the local audio capturing and processing.
        If you disable or re-enable local audio recording using the `enableLocalAudio` method, the local user may hear a pause in the remote audio playback.
        - \ref agora::rtc::IRtcEngine::muteLocalAudioStream "muteLocalAudioStream": Sends/Stops sending the local audio streams.

     @param enabled Sets whether to disable/re-enable the local audio function:
     - true: (Default) Re-enable the local audio function, that is, to start the local audio capturing device (for example, the microphone).
     - false: Disable the local audio function, that is, to stop local audio capturing.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn enable_local_audio(&self, enabled: bool) -> i32 {
        unsafe {
            let e: i32 = if enabled { 1 } else { 0 };
            return agorartcnative::enableLocalAudio(self.native_engine, e);
        }
    }

    /** Disables the audio module.

     @note
     - This method affects the internal engine and can be called after the \ref agora::rtc::IRtcEngine::leaveChannel "leaveChannel" method. You can call this method either before or after joining a channel.
     - This method resets the internal engine and takes some time to take effect. We recommend using the \ref agora::rtc::IRtcEngine::enableLocalAudio "enableLocalAudio" and \ref agora::rtc::IRtcEngine::muteLocalAudioStream "muteLocalAudioStream" methods to capture, process, and send the local audio streams.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn disable_audio(&self) -> i32 {
        unsafe {
            return agorartcnative::disableAudio(self.native_engine);
        }
    }

    /** Sets the audio parameters and application scenarios.

     @note
     - The *setAudioProfile* method must be called before the \ref IRtcEngine::joinChannel "joinChannel" method.
     - In the `COMMUNICATION` and `LIVE_BROADCASTING` profiles, the bitrate may be different from your settings due to network self-adaptation.
     - In scenarios requiring high-quality audio, for example, a music teaching scenario, we recommend setting profile as AUDIO_PROFILE_MUSIC_HIGH_QUALITY (4) and  scenario as AUDIO_SCENARIO_GAME_STREAMING (3).

     @param  profile Sets the sample rate, bitrate, encoding mode, and the number of channels. See #AUDIO_PROFILE_TYPE.
     @param  scenario Sets the audio application scenario. See #AUDIO_SCENARIO_TYPE.
     Under different audio scenarios, the device uses different volume tracks,
     i.e. either the in-call volume or the media volume. For details, see
     [What is the difference between the in-call volume and the media volume?](https://docs.agora.io/en/faq/system_volume).

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_audio_profile(&self, profile: AUDIO_PROFILE_TYPE, scenario: AUDIO_SCENARIO_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setAudioProfile(self.native_engine, profile as u32, scenario as u32);
        }
    }

    /** Stops/Resumes sending the local audio stream.

     A successful \ref agora::rtc::IRtcEngine::muteLocalAudioStream "muteLocalAudioStream" method call triggers the \ref agora::rtc::IRtcEngineEventHandler::onUserMuteAudio "onUserMuteAudio" callback on the remote client.
     @note
     - When @p mute is set as @p true, this method does not disable the microphone, which does not affect any ongoing recording.
     - If you call \ref agora::rtc::IRtcEngine::setChannelProfile "setChannelProfile" after this method, the SDK resets whether or not to mute the local audio according to the channel profile and user role. Therefore, we recommend calling this method after the `setChannelProfile` method.

     @param mute Sets whether to send/stop sending the local audio stream:
     - true: Stops sending the local audio stream.
     - false: (Default) Sends the local audio stream.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn mute_local_audio_stream(&self, mute: bool) -> i32 {
        unsafe {
            let m: i32 = if mute { 1 } else { 0 };
            return agorartcnative::muteLocalAudioStream(self.native_engine, m);
        }
    }

    /** Stops/Resumes receiving all remote users' audio streams.

     @param mute Sets whether to receive/stop receiving all remote users' audio streams.
     - true: Stops receiving all remote users' audio streams.
     - false: (Default) Receives all remote users' audio streams.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn mute_all_remote_audio_streams(&self, mute: bool) -> i32 {
        unsafe {
            let m: i32 = if mute { 1 } else { 0 };
            return agorartcnative::muteAllRemoteAudioStreams(self.native_engine, m);
        }
    }

    /** Stops/Resumes receiving all remote users' audio streams by default.

     You can call this method either before or after joining a channel. If you call `setDefaultMuteAllRemoteAudioStreams (true)` after joining a channel, the remote audio streams of all subsequent users are not received.

     @note If you want to resume receiving the audio stream, call \ref agora::rtc::IRtcEngine::muteRemoteAudioStream "muteRemoteAudioStream (false)",
     and specify the ID of the remote user whose audio stream you want to receive.
     To receive the audio streams of multiple remote users, call `muteRemoteAudioStream (false)` as many times.
     Calling `setDefaultMuteAllRemoteAudioStreams (false)` resumes receiving the audio streams of subsequent users only.

     @param mute Sets whether to receive/stop receiving all remote users' audio streams by default:
     - true:  Stops receiving all remote users' audio streams by default.
     - false: (Default) Receives all remote users' audio streams by default.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_default_mute_all_remote_video_streams(&self, mute: bool) -> i32 {
        unsafe {
            let m: i32 = if mute { 1 } else { 0 };
            return agorartcnative::setDefaultMuteAllRemoteVideoStreams(self.native_engine, m);
        }
    }

    /** Adjusts the playback volume of a specified remote user.

     You can call this method as many times as necessary to adjust the playback volume of different remote users, or to repeatedly adjust the playback volume of the same remote user.

     @note
     - Call this method after joining a channel.
     - The playback volume here refers to the mixed volume of a specified remote user.
     - This method can only adjust the playback volume of one specified remote user at a time. To adjust the playback volume of different remote users, call the method as many times, once for each remote user.

     @param uid The ID of the remote user.
     @param volume The playback volume of the specified remote user. The value ranges from 0 to 100:
     - 0: Mute.
     - 100: Original volume.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn adjust_user_playback_signal_volume(&self, uid: u32, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::adjustUserPlaybackSignalVolume(self.native_engine, uid, volume);
        }
    }

    /** Stops/Resumes receiving a specified remote user's audio stream.

     @note If you called the \ref agora::rtc::IRtcEngine::muteAllRemoteAudioStreams "muteAllRemoteAudioStreams" method and set @p mute as @p true to stop receiving all remote users' audio streams, call the *muteAllRemoteAudioStreams* method and set @p mute as @p false before calling this method. The *muteAllRemoteAudioStreams* method sets all remote audio streams, while the *muteRemoteAudioStream* method sets a specified remote audio stream.

     @param userId User ID of the specified remote user sending the audio.
     @param mute Sets whether to receive/stop receiving a specified remote user's audio stream:
     - true: Stops receiving the specified remote user's audio stream.
     - false: (Default) Receives the specified remote user's audio stream.

     @return
     - 0: Success.
     - < 0: Failure.

     */
    pub fn mute_remote_audio_stream(&self, uid: u32, mute: bool) -> i32 {
        unsafe {
            let m: i32 = if mute { 1 } else { 0 };
            return agorartcnative::muteRemoteAudioStream(self.native_engine, uid, m);
        }
    }

    /** Stops/Resumes sending the local video stream.

     A successful \ref agora::rtc::IRtcEngine::muteLocalVideoStream "muteLocalVideoStream" method call triggers the \ref agora::rtc::IRtcEngineEventHandler::onUserMuteVideo "onUserMuteVideo" callback on the remote client.

     @note
     - When set to *true*, this method does not disable the camera which does not affect the retrieval of the local video streams. This method executes faster than the \ref agora::rtc::IRtcEngine::enableLocalVideo "enableLocalVideo" method which controls the sending of the local video stream.
     - If you call \ref agora::rtc::IRtcEngine::setChannelProfile "setChannelProfile" after this method, the SDK resets whether or not to mute the local video according to the channel profile and user role. Therefore, we recommend calling this method after the `setChannelProfile` method.

     @param mute Sets whether to send/stop sending the local video stream:
     - true: Stop sending the local video stream.
     - false: (Default) Send the local video stream.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn mute_local_video_stream(&self, mute: bool) -> i32 {
        unsafe {
            let m: i32 = if mute { 1 } else { 0 };
            return agorartcnative::muteLocalVideoStream(self.native_engine, m);
        }
    }

    /** Enables/Disables the local video capture.

     This method disables or re-enables the local video capturer, and does not affect receiving the remote video stream.

     After you call the \ref agora::rtc::IRtcEngine::enableVideo "enableVideo" method, the local video capturer is enabled by default. You can call \ref agora::rtc::IRtcEngine::enableLocalVideo "enableLocalVideo(false)" to disable the local video capturer. If you want to re-enable it, call \ref agora::rtc::IRtcEngine::enableLocalVideo "enableLocalVideo(true)".

     After the local video capturer is successfully disabled or re-enabled, the SDK triggers the \ref agora::rtc::IRtcEngineEventHandler::onUserEnableLocalVideo "onUserEnableLocalVideo" callback on the remote client.

     @note This method affects the internal engine and can be called after the \ref agora::rtc::IRtcEngine::leaveChannel "leaveChannel" method.

     @param enabled Sets whether to disable/re-enable the local video, including the capturer, renderer, and sender:
     - true: (Default) Re-enable the local video.
     - false: Disable the local video. Once the local video is disabled, the remote users can no longer receive the video stream of this user, while this user can still receive the video streams of the other remote users.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn enable_local_video(&self, enabled: bool) -> i32 {
        unsafe {
            let e: i32 = if enabled { 1 } else { 0 };
            return agorartcnative::enableLocalVideo(self.native_engine, e);
        }
    }

    /** Stops/Resumes receiving all video stream from a specified remote user.

     @param  mute Sets whether to receive/stop receiving all remote users' video streams:
     - true: Stop receiving all remote users' video streams.
     - false: (Default) Receive all remote users' video streams.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn mute_all_remote_video_streams(&self, mute: bool) -> i32 {
        unsafe {
            let m: i32 = if mute { 1 } else { 0 };
            return agorartcnative::muteAllRemoteVideoStreams(self.native_engine, m);
        }
    }

    /** Stops/Resumes receiving all remote users' video streams by default.

     You can call this method either before or after joining a channel. If you call `setDefaultMuteAllRemoteVideoStreams (true)` after joining a channel, the remote video streams of all subsequent users are not received.

     @note If you want to resume receiving the video stream, call \ref agora::rtc::IRtcEngine::muteRemoteVideoStream "muteRemoteVideoStream (false)", and specify the ID of the remote user whose video stream you want to receive. To receive the video streams of multiple remote users, call `muteRemoteVideoStream (false)` as many times. Calling `setDefaultMuteAllRemoteVideoStreams (false)` resumes receiving the video streams of subsequent users only.

     @param mute Sets whether to receive/stop receiving all remote users' video streams by default:
     - true: Stop receiving all remote users' video streams by default.
     - false: (Default) Receive all remote users' video streams by default.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_default_mute_all_remote_audio_streams(&self, mute: bool) -> i32 {
        unsafe {
            let m: i32 = if mute { 1 } else { 0 };
            return agorartcnative::setDefaultMuteAllRemoteAudioStreams(self.native_engine, m);
        }
    }

    /** Stops/Resumes receiving the video stream from a specified remote user.

     @note If you called the \ref agora::rtc::IRtcEngine::muteAllRemoteVideoStreams "muteAllRemoteVideoStreams" method and set @p mute as @p true to stop receiving all remote video streams, call the *muteAllRemoteVideoStreams* method and set @p mute as @p false before calling this method.

     @param userId User ID of the specified remote user.
     @param mute Sets whether to stop/resume receiving the video stream from a specified remote user:
     - true: Stop receiving the specified remote user's video stream.
     - false: (Default) Receive the specified remote user's video stream.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn mute_remote_video_stream(&self, uid: u32, mute: bool) -> i32 {
        unsafe {
            let m: i32 = if mute { 1 } else { 0 };
            return agorartcnative::muteRemoteVideoStream(self.native_engine, uid, m);
        }
    }

    /** Sets the stream type of the remote video.

     Under limited network conditions, if the publisher has not disabled the dual-stream mode using `enableDualStreamMode(false)`,
     the receiver can choose to receive either the high-quality video stream (the high resolution, and high bitrate video stream) or
     the low-video stream (the low resolution, and low bitrate video stream).

     By default, users receive the high-quality video stream. Call this method if you want to switch to the low-video stream.
     This method allows the app to adjust the corresponding video stream type based on the size of the video window to
     reduce the bandwidth and resources.

     The aspect ratio of the low-video stream is the same as the high-quality video stream. Once the resolution of the high-quality video
     stream is set, the system automatically sets the resolution, frame rate, and bitrate of the low-video stream.

     The method result returns in the \ref agora::rtc::IRtcEngineEventHandler::onApiCallExecuted "onApiCallExecuted" callback.

     @param userId ID of the remote user sending the video stream.
     @param streamType  Sets the video-stream type. See #REMOTE_VIDEO_STREAM_TYPE.
     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_remote_video_stream_type(&self, uid: u32, stream_type: REMOTE_VIDEO_STREAM_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setRemoteVideoStreamType(self.native_engine, uid, stream_type as u32);
        }
    }

    /** Sets the default stream type of remote videos.

     Under limited network conditions, if the publisher has not disabled the dual-stream mode using `enableDualStreamMode(false)`,
     the receiver can choose to receive either the high-quality video stream (the high resolution, and high bitrate video stream) or
     the low-video stream (the low resolution, and low bitrate video stream).

     By default, users receive the high-quality video stream. Call this method if you want to switch to the low-video stream.
     This method allows the app to adjust the corresponding video stream type based on the size of the video window to
     reduce the bandwidth and resources. The aspect ratio of the low-video stream is the same as the high-quality video stream.
     Once the resolution of the high-quality video
     stream is set, the system automatically sets the resolution, frame rate, and bitrate of the low-video stream.

     The method result returns in the \ref agora::rtc::IRtcEngineEventHandler::onApiCallExecuted "onApiCallExecuted" callback.

     @param streamType Sets the default video-stream type. See #REMOTE_VIDEO_STREAM_TYPE.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_remote_default_video_stream_type(&self, stream_type: REMOTE_VIDEO_STREAM_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setRemoteDefaultVideoStreamType(self.native_engine, stream_type as u32);
        }
    }

    /** Enables the \ref agora::rtc::IRtcEngineEventHandler::onAudioVolumeIndication "onAudioVolumeIndication" callback at a set time interval to report on which users are speaking and the speakers' volume.

     Once this method is enabled, the SDK returns the volume indication in the \ref agora::rtc::IRtcEngineEventHandler::onAudioVolumeIndication "onAudioVolumeIndication" callback at the set time interval, whether or not any user is speaking in the channel.

     @param interval Sets the time interval between two consecutive volume indications:
     - &le; 0: Disables the volume indication.
     - > 0: Time interval (ms) between two consecutive volume indications. We recommend setting @p interval &gt; 200 ms. Do not set @p interval &lt; 10 ms, or the *onAudioVolumeIndication* callback will not be triggered.
     @param smooth  Smoothing factor sets the sensitivity of the audio volume indicator. The value ranges between 0 and 10. The greater the value, the more sensitive the indicator. The recommended value is 3.
     @param report_vad

     - true: Enable the voice activity detection of the local user. Once it is enabled, the `vad` parameter of the `onAudioVolumeIndication` callback reports the voice activity status of the local user.
     - false: (Default) Disable the voice activity detection of the local user. Once it is disabled, the `vad` parameter of the `onAudioVolumeIndication` callback does not report the voice activity status of the local user, except for the scenario where the engine automatically detects the voice activity of the local user.
     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn enable_audio_volume_indication(&self, interval: i32, smooth: i32, report_vad: bool) -> i32 {
        unsafe {
            let r: i32 = if report_vad { 1 } else { 0 };
            return agorartcnative::enableAudioVolumeIndication(self.native_engine, interval, smooth, r);
        }
    }

    /** @deprecated Starts an audio recording.

     Use \ref IRtcEngine::startAudioRecording(const char* filePath, int sampleRate, AUDIO_RECORDING_QUALITY_TYPE quality) "startAudioRecording"2 instead.

     The SDK allows recording during a call. Supported formats:

     - .wav: Large file size with high fidelity.
     - .aac: Small file size with low fidelity.

     This method has a fixed sample rate of 32 kHz.

     Ensure that the directory to save the recording file exists and is writable.
     This method is usually called after the \ref agora::rtc::IRtcEngine::joinChannel "joinChannel" method.
     The recording automatically stops when the \ref agora::rtc::IRtcEngine::leaveChannel "leaveChannel" method is called.

     @param filePath Pointer to the absolute file path of the recording file. The string of the file name is in UTF-8.
     @param quality Sets the audio recording quality. See #AUDIO_RECORDING_QUALITY_TYPE.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn start_audio_recoding(&self, file_path: &str, quality: AUDIO_RECORDING_QUALITY_TYPE) -> i32 {
        unsafe {
            let the_file_path: &CStr = &CString::new(file_path).expect("file_path new failed");
            return agorartcnative::startAudioRecording(self.native_engine, the_file_path.as_ptr(), quality as u32);
        }
    }

    /** Starts an audio recording on the client.
     *
     * The SDK allows recording during a call. After successfully calling this method, you can record the audio of all the users in the channel and get an audio recording file.
     * Supported formats of the recording file are as follows:
     * - .wav: Large file size with high fidelity.
     * - .aac: Small file size with low fidelity.
     *
     * @note
     * - Ensure that the directory you use to save the recording file exists and is writable.
     * - This method is usually called after the `joinChannel` method. The recording automatically stops when you call the `leaveChannel` method.
     * - For better recording effects, set quality as #AUDIO_RECORDING_QUALITY_MEDIUM or #AUDIO_RECORDING_QUALITY_HIGH when `sampleRate` is 44.1 kHz or 48 kHz.
     *
     * @param filePath Pointer to the absolute file path of the recording file. The string of the file name is in UTF-8, such as c:/music/audio.aac.
     * @param sampleRate Sample rate (kHz) of the recording file. Supported values are as follows:
     * - 16
     * - (Default) 32
     * - 44.1
     * - 48
     * @param quality Sets the audio recording quality. See #AUDIO_RECORDING_QUALITY_TYPE.
     *
     * @return
     * - 0: Success.
     * - < 0: Failure.
     */
    pub fn start_audio_recoding2(&self, file_path: &str, sample_rate: i32, quality: AUDIO_RECORDING_QUALITY_TYPE) -> i32 {
        unsafe {
            let the_file_path: &CStr = &CString::new(file_path).expect("file_path new failed");
            return agorartcnative::startAudioRecording2(self.native_engine, the_file_path.as_ptr(), sample_rate, quality as u32);
        }
    }

    /** Stops an audio recording on the client.

     You can call this method before calling the \ref agora::rtc::IRtcEngine::leaveChannel "leaveChannel" method else, the recording automatically stops when the \ref agora::rtc::IRtcEngine::leaveChannel "leaveChannel" method is called.

     @return
     - 0: Success
     - < 0: Failure.
     */
    pub fn stop_audio_recoding(&self) -> i32 {
        unsafe {
            return agorartcnative::stopAudioRecording(self.native_engine);
        }
    }

    /** Sets the sound position and gain of a remote user.

     When the local user calls this method to set the sound position of a remote user, the sound difference between the left and right channels allows the local user to track the real-time position of the remote user, creating a real sense of space. This method applies to massively multiplayer online games, such as Battle Royale games.

     @note
     - For this method to work, enable stereo panning for remote users by calling the \ref agora::rtc::IRtcEngine::enableSoundPositionIndication "enableSoundPositionIndication" method before joining a channel.
     - This method requires hardware support. For the best sound positioning, we recommend using a stereo speaker.

     @param uid The ID of the remote user.
     @param pan The sound position of the remote user. The value ranges from -1.0 to 1.0:
     - 0.0: the remote sound comes from the front.
     - -1.0: the remote sound comes from the left.
     - 1.0: the remote sound comes from the right.
     @param gain Gain of the remote user. The value ranges from 0.0 to 100.0. The default value is 100.0 (the original gain of the remote user). The smaller the value, the less the gain.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_remote_voice_position(&self, uid: u32, pan: f64, gain: f64) -> i32 {
        unsafe {
            return agorartcnative::setRemoteVoicePosition(self.native_engine, uid, pan, gain);
        }
    }

    /** Sets the log files that the SDK outputs.
     *
     * By default, the SDK outputs five log files, `agorasdk.log`, `agorasdk_1.log`, `agorasdk_2.log`, `agorasdk_3.log`, `agorasdk_4.log`, each with a default size of 1024 KB.
     * These log files are encoded in UTF-8. The SDK writes the latest logs in `agorasdk.log`. When `agorasdk.log` is full, the SDK deletes the log file with the earliest
     * modification time among the other four, renames `agorasdk.log` to the name of the deleted log file, and create a new `agorasdk.log` to record latest logs.
     *
     * @note Ensure that you call this method immediately after calling \ref agora::rtc::IRtcEngine::initialize "initialize" , otherwise the output logs may not be complete.
     *
     * @see \ref IRtcEngine::setLogFileSize "setLogFileSize"
     * @see \ref IRtcEngine::setLogFilter "setLogFilter"
     *
     * @param filePath The absolute path of log files. The default file path is `C: \Users\<user_name>\AppData\Local\Agora\<process_name>\agorasdk.log`.
     * Ensure that the directory for the log files exists and is writable. You can use this parameter to rename the log files.
     *
     * @return
     * - 0: Success.
     * - < 0: Failure.
     */
    pub fn set_log_file(&self, file: &str) -> i32 {
        unsafe {
            let the_file: &CStr = &CString::new(file).expect("file new failed");
            return agorartcnative::setLogFile(self.native_engine, the_file.as_ptr());
        }
    }

    /** Sets the output log level of the SDK.

     You can use one or a combination of the log filter levels. The log level follows the sequence of OFF, CRITICAL, ERROR, WARNING, INFO, and DEBUG. Choose a level to see the logs preceding that level.

     If you set the log level to WARNING, you see the logs within levels CRITICAL, ERROR, and WARNING.

     @see \ref IRtcEngine::setLogFile "setLogFile"
     @see \ref IRtcEngine::setLogFileSize "setLogFileSize"

     @param filter Sets the log filter level. See #LOG_FILTER_TYPE.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_log_filter(&self, filter: u32) -> i32 {
        unsafe {
            return agorartcnative::setLogFilter(self.native_engine, filter);
        }
    }

    /** Sets the size of a log file that the SDK outputs.
     *
     * By default, the SDK outputs five log files, `agorasdk.log`, `agorasdk_1.log`, `agorasdk_2.log`, `agorasdk_3.log`, `agorasdk_4.log`, each with a default size of 1024 KB.
     * These log files are encoded in UTF-8. The SDK writes the latest logs in `agorasdk.log`. When `agorasdk.log` is full, the SDK deletes the log file with the earliest
     * modification time among the other four, renames `agorasdk.log` to the name of the deleted log file, and create a new `agorasdk.log` to record latest logs.
     *
     * @see \ref IRtcEngine::setLogFile "setLogFile"
     * @see \ref IRtcEngine::setLogFilter "setLogFilter"
     *
     * @param fileSizeInKBytes The size (KB) of a log file. The default value is 1024 KB. If you set `fileSizeInKByte` to 1024 KB,
     * the SDK outputs at most 5 MB log files; if you set it to less than 1024 KB, the maximum size of a log file is still 1024 KB.
     *
     * @return
     * - 0: Success.
     * - < 0: Failure.
     */
    pub fn set_log_file_size(&self, file_size_in_kbytes: u32) -> i32 {
        unsafe {
            return agorartcnative::setLogFileSize(self.native_engine, file_size_in_kbytes);
        }
    }

    /**
     @deprecated This method is deprecated, use the \ref IRtcEngine::setLocalRenderMode(RENDER_MODE_TYPE renderMode, VIDEO_MIRROR_MODE_TYPE mirrorMode) "setLocalRenderMode"2 method instead.
     Sets the local video display mode.

     This method can be called multiple times during a call to change the display mode.

     @param renderMode  Sets the local video display mode. See #RENDER_MODE_TYPE.
     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_local_render_mode(&self, render_mode: RENDER_MODE_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setLocalRenderMode(self.native_engine, render_mode as u32);
        }
    }

    /** Updates the display mode of the local video view.

     @since v3.0.0

     After initializing the local video view, you can call this method to update its rendering and mirror modes. It affects only the video view that the local user sees, not the published local video stream.

     @note
     - Ensure that you have called the \ref IRtcEngine::setupLocalVideo "setupLocalVideo" method to initialize the local video view before calling this method.
     - During a call, you can call this method as many times as necessary to update the display mode of the local video view.
     @param renderMode The rendering mode of the local video view. See #RENDER_MODE_TYPE.
     @param mirrorMode
     - The mirror mode of the local video view. See #VIDEO_MIRROR_MODE_TYPE.
     - **Note**: If you use a front camera, the SDK enables the mirror mode by default; if you use a rear camera, the SDK disables the mirror mode by default.
     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_local_render_mode2(&self, render_mode: RENDER_MODE_TYPE, mirror_mode: VIDEO_MIRROR_MODE_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setLocalRenderMode2(self.native_engine, render_mode as u32, mirror_mode as u32);
        }
    }

    /**
     @deprecated This method is deprecated, use the \ref IRtcEngine::setRemoteRenderMode(uid_t userId, RENDER_MODE_TYPE renderMode, VIDEO_MIRROR_MODE_TYPE mirrorMode) "setRemoteRenderMode"2 method instead.
     Sets the video display mode of a specified remote user.

     This method can be called multiple times during a call to change the display mode.

     @param userId ID of the remote user.
     @param renderMode  Sets the video display mode. See #RENDER_MODE_TYPE.
     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_remote_render_mode(&self, uid: u32, render_mode: RENDER_MODE_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setRemoteRenderMode(self.native_engine, uid, render_mode as u32);
        }
    }

    /** Updates the display mode of the video view of a remote user.

     @since v3.0.0
     After initializing the video view of a remote user, you can call this method to update its rendering and mirror modes. This method affects only the video view that the local user sees.

     @note
     - Ensure that you have called the \ref IRtcEngine::setupRemoteVideo "setupRemoteVideo" method to initialize the remote video view before calling this method.
     - During a call, you can call this method as many times as necessary to update the display mode of the video view of a remote user.

     @param userId The ID of the remote user.
     @param renderMode The rendering mode of the remote video view. See #RENDER_MODE_TYPE.
     @param mirrorMode
     - The mirror mode of the remote video view. See #VIDEO_MIRROR_MODE_TYPE.
     - **Note**: The SDK disables the mirror mode by default.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_remote_render_mode2(&self, uid: u32, render_mode: RENDER_MODE_TYPE, mirror_mode: VIDEO_MIRROR_MODE_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setRemoteRenderMode2(self.native_engine, uid, render_mode as u32, mirror_mode as u32);
        }
    }

    /**
     @deprecated This method is deprecated, use the \ref IRtcEngine::setupLocalVideo "setupLocalVideo"
     or \ref IRtcEngine::setLocalRenderMode(RENDER_MODE_TYPE renderMode, VIDEO_MIRROR_MODE_TYPE mirrorMode) "setLocalRenderMode" method instead.

     Sets the local video mirror mode.

     You must call this method before calling the \ref agora::rtc::IRtcEngine::startPreview "startPreview" method, otherwise the mirror mode will not work.

     @warning
     - Call this method after calling the \ref agora::rtc::IRtcEngine::setupLocalVideo "setupLocalVideo" method to initialize the local video view.
     - During a call, you can call this method as many times as necessary to update the mirror mode of the local video view.

     @param mirrorMode Sets the local video mirror mode. See #VIDEO_MIRROR_MODE_TYPE.
     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_local_video_mirror_mode(&self, mirror_mode: VIDEO_MIRROR_MODE_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setLocalVideoMirrorMode(self.native_engine, mirror_mode as u32);
        }
    }

    /** Sets the stream mode to the single-stream (default) or dual-stream mode. (`LIVE_BROADCASTING` only.)

     If the dual-stream mode is enabled, the receiver can choose to receive the high stream (high-resolution and high-bitrate video stream), or the low stream (low-resolution and low-bitrate video stream).

     @param enabled Sets the stream mode:
     - true: Dual-stream mode.
     - false: Single-stream mode.
     */
    pub fn enable_dual_stream_mode(&self, enabled: bool) -> i32 {
        unsafe {
            let e: i32 = if enabled { 1 } else { 0 };
            return agorartcnative::enableDualStreamMode(self.native_engine, e);
        }
    }

    /** Adjusts the recording volume.

     @param volume Recording volume. To avoid echoes and
     improve call quality, Agora recommends setting the value of volume between
     0 and 100. If you need to set the value higher than 100, contact
     support@agora.io first.
     - 0: Mute.
     - 100: Original volume.


     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn adjust_recoding_signal_volume(&self, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::adjustRecordingSignalVolume(self.native_engine, volume);
        }
    }

    /** Adjusts the playback volume of all remote users.

     @note
     - This method adjusts the playback volume that is the mixed volume of all remote users.
     - (Since v2.3.2) To mute the local audio playback, call both the `adjustPlaybackSignalVolume` and \ref IRtcEngine::adjustAudioMixingVolume "adjustAudioMixingVolume" methods and set the volume as `0`.

     @param volume The playback volume of all remote users. To avoid echoes and
     improve call quality, Agora recommends setting the value of volume between
     0 and 100. If you need to set the value higher than 100, contact
     support@agora.io first.
     - 0: Mute.
     - 100: Original volume.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn adjust_playback_signal_volume(&self, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::adjustPlaybackSignalVolume(self.native_engine, volume);
        }
    }

    /**
     @deprecated This method is deprecated. As of v3.0.0, the Native SDK automatically enables interoperability with the Web SDK, so you no longer need to call this method.
     Enables interoperability with the Agora Web SDK.

     @note
     - This method applies only to the `LIVE_BROADCASTING` profile. In the `COMMUNICATION` profile, interoperability with the Agora Web SDK is enabled by default.
     - If the channel has Web SDK users, ensure that you call this method, or the video of the Native user will be a black screen for the Web user.

     @param enabled Sets whether to enable/disable interoperability with the Agora Web SDK:
     - true: Enable.
     - false: (Default) Disable.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn enable_web_sdk_interoperability(&self, enabled: bool) -> i32 {
        unsafe {
            let e: i32 = if enabled { 1 } else { 0 };
            return agorartcnative::enableWebSdkInteroperability(self.native_engine, e);
        }
    }

    //only for live broadcast
    /** **DEPRECATED** Sets the preferences for the high-quality video. (`LIVE_BROADCASTING` only).

     This method is deprecated as of v2.4.0.

     @param preferFrameRateOverImageQuality Sets the video quality preference:
     - true: Frame rate over image quality.
     - false: (Default) Image quality over frame rate.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_video_quality_parameters(&self, prefer_frame_rate_over_image_quality: bool) -> i32 {
        unsafe {
            let prefer: i32 = if prefer_frame_rate_over_image_quality { 1 } else { 0 };
            return agorartcnative::setVideoQualityParameters(self.native_engine, prefer);
        }
    }

    /** Sets the fallback option for the published video stream based on the network conditions.

     If `option` is set as #STREAM_FALLBACK_OPTION_AUDIO_ONLY (2), the SDK will:

     - Disable the upstream video but enable audio only when the network conditions deteriorate and cannot support both video and audio.
     - Re-enable the video when the network conditions improve.

     When the published video stream falls back to audio only or when the audio-only stream switches back to the video, the SDK triggers the \ref agora::rtc::IRtcEngineEventHandler::onLocalPublishFallbackToAudioOnly "onLocalPublishFallbackToAudioOnly" callback.

     @note Agora does not recommend using this method for CDN live streaming, because the remote CDN live user will have a noticeable lag when the published video stream falls back to audio only.

     @param option Sets the fallback option for the published video stream:
     - #STREAM_FALLBACK_OPTION_DISABLED (0): (Default) No fallback behavior for the published video stream when the uplink network condition is poor. The stream quality is not guaranteed.
     - #STREAM_FALLBACK_OPTION_AUDIO_ONLY (2): The published video stream falls back to audio only when the uplink network condition is poor.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_local_publish_fallback_option(&self, option: STREAM_FALLBACK_OPTIONS) -> i32 {
        unsafe {
            return agorartcnative::setLocalPublishFallbackOption(self.native_engine, option as u32);
        }
    }

    /** Sets the fallback option for the remotely subscribed video stream based on the network conditions.

     The default setting for `option` is #STREAM_FALLBACK_OPTION_VIDEO_STREAM_LOW (1), where the remotely subscribed video stream falls back to the low-stream video (low resolution and low bitrate) under poor downlink network conditions.

     If `option` is set as #STREAM_FALLBACK_OPTION_AUDIO_ONLY (2), the SDK automatically switches the video from a high-stream to a low-stream, or disables the video when the downlink network conditions cannot support both audio and video to guarantee the quality of the audio. The SDK monitors the network quality and restores the video stream when the network conditions improve.

     When the remotely subscribed video stream falls back to audio only or when the audio-only stream switches back to the video stream, the SDK triggers the \ref agora::rtc::IRtcEngineEventHandler::onRemoteSubscribeFallbackToAudioOnly "onRemoteSubscribeFallbackToAudioOnly" callback.

     @param  option  Sets the fallback option for the remotely subscribed video stream. See #STREAM_FALLBACK_OPTIONS.
     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_remote_subscribe_fallback_option(&self, option: STREAM_FALLBACK_OPTIONS) -> i32 {
        unsafe {
            return agorartcnative::setRemoteSubscribeFallbackOption(self.native_engine, option as u32);
        }
    }

    /** Retrieves the current call ID.

     When a user joins a channel on a client, a @p callId is generated to identify the call from the client. Feedback methods, such as \ref IRtcEngine::rate "rate" and \ref IRtcEngine::complain "complain", must be called after the call ends to submit feedback to the SDK.

     The \ref IRtcEngine::rate "rate" and \ref IRtcEngine::complain "complain" methods require the @p callId parameter retrieved from the *getCallId* method during a call. @p callId is passed as an argument into the \ref IRtcEngine::rate "rate" and \ref IRtcEngine::complain "complain" methods after the call ends.

     @param callId Pointer to the current call ID.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn get_call_id(&self) -> String {
        unsafe {
            let s = agorartcnative::getCallId(self.native_engine);
            let r = CStr::from_ptr(s).to_string_lossy().into_owned();
            return r;
        }
    }

    /** Allows a user to rate a call after the call ends.

     @param callId Pointer to the ID of the call, retrieved from the \ref IRtcEngine::getCallId "getCallId" method.
     @param rating  Rating of the call. The value is between 1 (lowest score) and 5 (highest score). If you set a value out of this range, the #ERR_INVALID_ARGUMENT (2) error returns.
     @param description (Optional) Pointer to the description of the rating, with a string length of less than 800 bytes.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn rate(&self, call_id: &str, rating: i32, description: &str) -> i32 {
        unsafe {
            let the_call_id: &CStr = &CString::new(call_id).expect("call_id new failed");
            let the_description: &CStr = &CString::new(description).expect("description new failed");
            return agorartcnative::rate(self.native_engine, the_call_id.as_ptr(), rating, the_description.as_ptr());
        }
    }

    /** Allows a user to complain about the call quality after a call ends.

    @param callId Pointer to the ID of the call, retrieved from the \ref IRtcEngine::getCallId "getCallId" method.
    @param description (Optional) Pointer to the description of the complaint, with a string length of less than 800 bytes.

    @return
    - 0: Success.
    - < 0: Failure.

    */
    pub fn complain(&self, call_id: &str, description: &str) -> i32 {
        unsafe {
            let the_call_id: &CStr = &CString::new(call_id).expect("call_id new failed");
            let the_description: &CStr = &CString::new(description).expect("description new failed");
            return agorartcnative::complain(self.native_engine, the_call_id.as_ptr(), the_description.as_ptr());
        }
    }

    /** Retrieves the SDK version number.

     @param build Pointer to the build number.
     @return The version of the current SDK in the string format. For example, 2.3.1.
     */
    pub fn get_version(&self) -> String {
        unsafe {
            let s = agorartcnative::getVersion(self.native_engine);
            let r = CStr::from_ptr(s).to_string_lossy().into_owned();
            return r;
        }
    }

    /**  Enables the network connection quality test.

     This method tests the quality of the users' network connections and is disabled by default.

     Before a user joins a channel or before an audience switches to a host, call this method to check the uplink network quality.

     This method consumes additional network traffic, and hence may affect communication quality.

     Call the \ref IRtcEngine::disableLastmileTest "disableLastmileTest" method to disable this test after receiving the \ref IRtcEngineEventHandler::onLastmileQuality "onLastmileQuality" callback, and before joining a channel.

     @note
     - Do not call any other methods before receiving the \ref IRtcEngineEventHandler::onLastmileQuality "onLastmileQuality" callback. Otherwise, the callback may be interrupted by other methods, and hence may not be triggered.
     - A host should not call this method after joining a channel (when in a call).
     - If you call this method to test the last-mile quality, the SDK consumes the bandwidth of a video stream, whose bitrate corresponds to the bitrate you set in the \ref agora::rtc::IRtcEngine::setVideoEncoderConfiguration "setVideoEncoderConfiguration" method. After you join the channel, whether you have called the `disableLastmileTest` method or not, the SDK automatically stops consuming the bandwidth.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn enable_lastmile_test(&self) -> i32 {
        unsafe {
            return agorartcnative::enableLastmileTest(self.native_engine);
        }
    }

    /** Disables the network connection quality test.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn disable_lastmile_test(&self) -> i32 {
        unsafe {
            return agorartcnative::disableLastmileTest(self.native_engine);
        }
    }

    /** Starts the last-mile network probe test.

    This method starts the last-mile network probe test before joining a channel to get the uplink and downlink last-mile network statistics, including the bandwidth, packet loss, jitter, and round-trip time (RTT).

    Call this method to check the uplink network quality before users join a channel or before an audience switches to a host.
    Once this method is enabled, the SDK returns the following callbacks:
    - \ref IRtcEngineEventHandler::onLastmileQuality "onLastmileQuality": the SDK triggers this callback within two seconds depending on the network conditions. This callback rates the network conditions and is more closely linked to the user experience.
    - \ref IRtcEngineEventHandler::onLastmileProbeResult "onLastmileProbeResult": the SDK triggers this callback within 30 seconds depending on the network conditions. This callback returns the real-time statistics of the network conditions and is more objective.

    @note
    - This method consumes extra network traffic and may affect communication quality. We do not recommend calling this method together with enableLastmileTest.
    - Do not call other methods before receiving the \ref IRtcEngineEventHandler::onLastmileQuality "onLastmileQuality" and \ref IRtcEngineEventHandler::onLastmileProbeResult "onLastmileProbeResult" callbacks. Otherwise, the callbacks may be interrupted.
    - In the `LIVE_BROADCASTING` profile, a host should not call this method after joining a channel.

    @param config Sets the configurations of the last-mile network probe test. See LastmileProbeConfig.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn start_lastmile_probe_test(&self, config: agorartcnative::LastmileProbeConfig) -> i32 {
        unsafe {
            return agorartcnative::startLastmileProbeTest(self.native_engine, config);
        }
    }

    /** Stops the last-mile network probe test. */
    pub fn stop_lastmile_probe_test(&self) -> i32 {
        unsafe {
            return agorartcnative::stopLastmileProbeTest(self.native_engine);
        }
    }

    /** Retrieves the warning or error description.

     @param code Warning code or error code returned in the \ref agora::rtc::IRtcEngineEventHandler::onWarning "onWarning" or \ref agora::rtc::IRtcEngineEventHandler::onError "onError" callback.

     @return #WARN_CODE_TYPE or #ERROR_CODE_TYPE.
     */
    pub fn get_error_description(&self, code: i32) -> String {
        unsafe {
            let s = agorartcnative::getErrorDescription(self.native_engine, code);
            let r = CStr::from_ptr(s).to_string_lossy().into_owned();
            return r;
        }
    }

    /** **DEPRECATED** Enables built-in encryption with an encryption password before users join a channel.

     Deprecated as of v3.1.0. Use the \ref agora::rtc::IRtcEngine::enableEncryption "enableEncryption" instead.

     All users in a channel must use the same encryption password. The encryption password is automatically cleared once a user leaves the channel.

     If an encryption password is not specified, the encryption functionality will be disabled.

     @note
     - Do not use this method for CDN live streaming.
     - For optimal transmission, ensure that the encrypted data size does not exceed the original data size + 16 bytes. 16 bytes is the maximum padding size for AES encryption.

     @param secret Pointer to the encryption password.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_encryption_secret(&self, secret: &str) -> i32 {
        unsafe {
            let the_secret: &CStr = &CString::new(secret).expect("secret new failed");
            return agorartcnative::setEncryptionSecret(self.native_engine, the_secret.as_ptr());
        }
    }

    /** **DEPRECATED** Sets the built-in encryption mode.

     @deprecated Deprecated as of v3.1.0. Use the \ref agora::rtc::IRtcEngine::enableEncryption "enableEncryption" instead.

     The Agora SDK supports built-in encryption, which is set to the @p aes-128-xts mode by default. Call this method to use other encryption modes.

     All users in the same channel must use the same encryption mode and password.

     Refer to the information related to the AES encryption algorithm on the differences between the encryption modes.

     @note Call the \ref IRtcEngine::setEncryptionSecret "setEncryptionSecret" method to enable the built-in encryption function before calling this method.

     @param encryptionMode Pointer to the set encryption mode:
     - "aes-128-xts": (Default) 128-bit AES encryption, XTS mode.
     - "aes-128-ecb": 128-bit AES encryption, ECB mode.
     - "aes-256-xts": 256-bit AES encryption, XTS mode.
     - "": When encryptionMode is set as NULL, the encryption mode is set as "aes-128-xts" by default.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_encryption_mode(&self, encryption_mode: &str) -> i32 {
        unsafe {
            let the_encryption_mode: &CStr = &CString::new(encryption_mode).expect("encryption_mode new failed");
            return agorartcnative::setEncryptionMode(self.native_engine, the_encryption_mode.as_ptr());
        }
    }

    /** Creates a data stream.

     Each user can create up to five data streams during the lifecycle of the IRtcEngine.

     @note Set both the @p reliable and @p ordered parameters to true or false. Do not set one as true and the other as false.

     @param streamId Pointer to the ID of the created data stream.
     @param reliable Sets whether or not the recipients are guaranteed to receive the data stream from the sender within five seconds:
     - true: The recipients receive the data stream from the sender within five seconds. If the recipient does not receive the data stream within five seconds, an error is reported to the application.
     - false: There is no guarantee that the recipients receive the data stream within five seconds and no error message is reported for any delay or missing data stream.
     @param ordered Sets whether or not the recipients receive the data stream in the sent order:
     - true: The recipients receive the data stream in the sent order.
     - false: The recipients do not receive the data stream in the sent order.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn create_data_stream(&self, stream_id: *mut i32, reliable: bool, ordered: bool) -> i32 {
        unsafe {
            let is_reliable: i32 = if reliable { 1 } else { 0 };
            let is_ordered: i32 = if ordered { 1 } else { 0 };
            return agorartcnative::createDataStream(self.native_engine, stream_id, is_reliable, is_ordered);
        }
    }

    /** Sends data stream messages to all users in a channel.

     The SDK has the following restrictions on this method:
     - Up to 30 packets can be sent per second in a channel with each packet having a maximum size of 1 kB.
     - Each client can send up to 6 kB of data per second.
     - Each user can have up to five data streams simultaneously.

     A successful \ref agora::rtc::IRtcEngine::sendStreamMessage "sendStreamMessage" method call triggers the
     \ref agora::rtc::IRtcEngineEventHandler::onStreamMessage "onStreamMessage" callback on the remote client, from which the remote user gets the stream message.

     A failed \ref agora::rtc::IRtcEngine::sendStreamMessage "sendStreamMessage" method call triggers the
      \ref agora::rtc::IRtcEngineEventHandler::onStreamMessage "onStreamMessage" callback on the remote client.
     @note This method applies only to the `COMMUNICATION` profile or to the hosts in the `LIVE_BROADCASTING` profile. If an audience in the `LIVE_BROADCASTING` profile calls this method, the audience may be switched to a host.
     @param  streamId  ID of the sent data stream, returned in the \ref IRtcEngine::createDataStream "createDataStream" method.
     @param data Pointer to the sent data.
     @param length Length of the sent data.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn send_stream_message(&self, stream_id: i32, data: &str, length: i64) -> i32 {
        unsafe {
            let the_data: &CStr = &CString::new(data).expect("data new failed");
            return agorartcnative::sendStreamMessage(self.native_engine, stream_id, the_data.as_ptr(), length);
        }
    }

    /** Publishes the local stream to a specified CDN live RTMP address.  (CDN live only.)

     The SDK returns the result of this method call in the \ref IRtcEngineEventHandler::onStreamPublished "onStreamPublished" callback.

     The \ref agora::rtc::IRtcEngine::addPublishStreamUrl "addPublishStreamUrl" method call triggers the \ref agora::rtc::IRtcEngineEventHandler::onRtmpStreamingStateChanged "onRtmpStreamingStateChanged" callback on the local client to report the state of adding a local stream to the CDN.
     @note
     - Ensure that the user joins the channel before calling this method.
     - Ensure that you enable the RTMP Converter service before using this function. See  *Prerequisites* in the advanced guide *Push Streams to CDN*.
     - This method adds only one stream RTMP URL address each time it is called.
     - This method applies to `LIVE_BROADCASTING` only.

     @param url The CDN streaming URL in the RTMP format. The maximum length of this parameter is 1024 bytes. The RTMP URL address must not contain special characters, such as Chinese language characters.
     @param  transcodingEnabled Sets whether transcoding is enabled/disabled:
     - true: Enable transcoding. To [transcode](https://docs.agora.io/en/Agora%20Platform/terms?platform=All%20Platforms#transcoding) the audio or video streams when publishing them to CDN live, often used for combining the audio and video streams of multiple hosts in CDN live. If you set this parameter as `true`, ensure that you call the \ref IRtcEngine::setLiveTranscoding "setLiveTranscoding" method before this method.
     - false: Disable transcoding.

     @return
     - 0: Success.
     - < 0: Failure.
          - #ERR_INVALID_ARGUMENT (2): The RTMP URL address is NULL or has a string length of 0.
          - #ERR_NOT_INITIALIZED (7): You have not initialized the RTC engine when publishing the stream.
     */
    pub fn add_publish_stream_url(&self, url: &str, transcoding_enabled: bool) -> i32 {
        unsafe {
            let the_url: &CStr = &CString::new(url).expect("url new failed");
            let is_transcoding_enabled: i32 = if transcoding_enabled { 1 } else { 0 };
            return agorartcnative::addPublishStreamUrl(self.native_engine, the_url.as_ptr(), is_transcoding_enabled);
        }
    }

    /** Removes an RTMP stream from the CDN. (CDN live only.)

     This method removes the RTMP URL address (added by the \ref IRtcEngine::addPublishStreamUrl "addPublishStreamUrl" method) from a CDN live stream. The SDK returns the result of this method call in the \ref IRtcEngineEventHandler::onStreamUnpublished "onStreamUnpublished" callback.

     The \ref agora::rtc::IRtcEngine::removePublishStreamUrl "removePublishStreamUrl" method call triggers the \ref agora::rtc::IRtcEngineEventHandler::onRtmpStreamingStateChanged "onRtmpStreamingStateChanged" callback on the local client to report the state of removing an RTMP stream from the CDN.
     @note
     - This method removes only one RTMP URL address each time it is called.
     - The RTMP URL address must not contain special characters, such as Chinese language characters.
     - This method applies to `LIVE_BROADCASTING` only.

     @param url The RTMP URL address to be removed. The maximum length of this parameter is 1024 bytes.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn remove_publish_stream_url(&self, url: &str) -> i32 {
        unsafe {
            let the_url: &CStr = &CString::new(url).expect("url new failed");
            return agorartcnative::removePublishStreamUrl(self.native_engine, the_url.as_ptr());
        }
    }

    /** Sets the video layout and audio settings for CDN live. (CDN live only.)

     The SDK triggers the \ref agora::rtc::IRtcEngineEventHandler::onTranscodingUpdated "onTranscodingUpdated" callback when you call the `setLiveTranscoding` method to update the transcoding setting.

     @note
     - This method applies to `LIVE_BROADCASTING` only.
     - Ensure that you enable the RTMP Converter service before using this function. See *Prerequisites* in the advanced guide *Push Streams to CDN*.
     - If you call the `setLiveTranscoding` method to update the transcoding setting for the first time, the SDK does not trigger the `onTranscodingUpdated` callback.

     @param transcoding Sets the CDN live audio/video transcoding settings. See LiveTranscoding.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_live_transcoding(&self, transcoding: *const agorartcnative::LiveTranscoding) -> i32 {
        unsafe {
            return agorartcnative::setLiveTranscoding(self.native_engine, transcoding);
        }
    }

    /** **DEPRECATED** Adds a watermark image to the local video or CDN live stream.

     This method is deprecated from v2.9.1. Use \ref agora::rtc::IRtcEngine::addVideoWatermark(const char* watermarkUrl, const WatermarkOptions& options) "addVideoWatermark"2 instead.

     This method adds a PNG watermark image to the local video stream for the recording device, channel audience, and CDN live audience to view and capture.

     To add the PNG file to the CDN live publishing stream, see the \ref IRtcEngine::setLiveTranscoding "setLiveTranscoding" method.

     @param watermark Pointer to the watermark image to be added to the local video stream. See RtcImage.

     @note
     - The URL descriptions are different for the local video and CDN live streams:
        - In a local video stream, `url` in RtcImage refers to the absolute path of the added watermark image file in the local video stream.
        - In a CDN live stream, `url` in RtcImage refers to the URL address of the added watermark image in the CDN live streaming.
     - The source file of the watermark image must be in the PNG file format. If the width and height of the PNG file differ from your settings in this method, the PNG file will be cropped to conform to your settings.
     - The Agora SDK supports adding only one watermark image onto a local video or CDN live stream. The newly added watermark image replaces the previous one.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn add_video_watermark(&self, watermark: agorartcnative::RtcImage) -> i32 {
        unsafe {
            return agorartcnative::addVideoWatermark(self.native_engine, watermark);
        }
    }

    /** Adds a watermark image to the local video.

     This method adds a PNG watermark image to the local video in the live streaming. Once the watermark image is added, all the audience in the channel (CDN audience included),
     and the recording device can see and capture it. Agora supports adding only one watermark image onto the local video, and the newly watermark image replaces the previous one.

     The watermark position depends on the settings in the \ref IRtcEngine::setVideoEncoderConfiguration "setVideoEncoderConfiguration" method:
     - If the orientation mode of the encoding video is #ORIENTATION_MODE_FIXED_LANDSCAPE, or the landscape mode in #ORIENTATION_MODE_ADAPTIVE, the watermark uses the landscape orientation.
     - If the orientation mode of the encoding video is #ORIENTATION_MODE_FIXED_PORTRAIT, or the portrait mode in #ORIENTATION_MODE_ADAPTIVE, the watermark uses the portrait orientation.
     - When setting the watermark position, the region must be less than the dimensions set in the `setVideoEncoderConfiguration` method. Otherwise, the watermark image will be cropped.

     @note
     - Ensure that you have called the \ref agora::rtc::IRtcEngine::enableVideo "enableVideo" method to enable the video module before calling this method.
     - If you only want to add a watermark image to the local video for the audience in the CDN live streaming channel to see and capture, you can call this method or the \ref agora::rtc::IRtcEngine::setLiveTranscoding "setLiveTranscoding" method.
     - This method supports adding a watermark image in the PNG file format only. Supported pixel formats of the PNG image are RGBA, RGB, Palette, Gray, and Alpha_gray.
     - If the dimensions of the PNG image differ from your settings in this method, the image will be cropped or zoomed to conform to your settings.
     - If you have enabled the local video preview by calling the \ref agora::rtc::IRtcEngine::startPreview "startPreview" method, you can use the `visibleInPreview` member in the WatermarkOptions class to set whether or not the watermark is visible in preview.
     - If you have enabled the mirror mode for the local video, the watermark on the local video is also mirrored. To avoid mirroring the watermark, Agora recommends that you do not use the mirror and watermark functions for the local video at the same time. You can implement the watermark function in your application layer.

     @param watermarkUrl The local file path of the watermark image to be added. This method supports adding a watermark image from the local absolute or relative file path.
     @param options Pointer to the watermark's options to be added. See WatermarkOptions for more infomation.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn add_video_watermark2(&self, watermark_url: &str, options: agorartcnative::WatermarkOptions) -> i32 {
        unsafe {
            let the_url: &CStr = &CString::new(watermark_url).expect("watermark_url new failed");
            return agorartcnative::addVideoWatermark2(self.native_engine, the_url.as_ptr(), options);
        }
    }

    /** Removes the watermark image from the video stream added by the \ref agora::rtc::IRtcEngine::addVideoWatermark(const char* watermarkUrl, const WatermarkOptions& options) "addVideoWatermark" method.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn clear_video_watermarks(&self) -> i32 {
        unsafe {
            return agorartcnative::clearVideoWatermarks(self.native_engine);
        }
    }

    /** @since v3.0.0

     Enables/Disables image enhancement and sets the options.

    @note
    - Call this method after calling the enableVideo method.
    - Currently this method does not apply for macOS.

    @param enabled Sets whether or not to enable image enhancement:
    - true: enables image enhancement.
    - false: disables image enhancement.
    @param options Sets the image enhancement option. See BeautyOptions.
    */
    pub fn set_beauty_effect_options(&self, enabled: bool, options: agorartcnative::BeautyOptions) -> i32 {
        unsafe {
            let is_enabled: i32 = if enabled { 1 } else { 0 };
            return agorartcnative::setBeautyEffectOptions(self.native_engine, is_enabled, options);
        }
    }

    /** Adds a voice or video stream URL address to the live streaming.

    The \ref IRtcEngineEventHandler::onStreamPublished "onStreamPublished" callback returns the inject status. If this method call is successful, the server pulls the voice or video stream and injects it into a live channel. This is applicable to scenarios where all audience members in the channel can watch a live show and interact with each other.

     The \ref agora::rtc::IRtcEngine::addInjectStreamUrl "addInjectStreamUrl" method call triggers the following callbacks:
    - The local client:
      - \ref agora::rtc::IRtcEngineEventHandler::onStreamInjectedStatus "onStreamInjectedStatus" , with the state of the injecting the online stream.
      - \ref agora::rtc::IRtcEngineEventHandler::onUserJoined "onUserJoined" (uid: 666), if the method call is successful and the online media stream is injected into the channel.
    - The remote client:
      - \ref agora::rtc::IRtcEngineEventHandler::onUserJoined "onUserJoined" (uid: 666), if the method call is successful and the online media stream is injected into the channel.

     @note
     - Ensure that you enable the RTMP Converter service before using this function. See *Prerequisites* in the advanced guide *Push Streams to CDN*.
     - This method applies to the Native SDK v2.4.1 and later.
     - This method applies to the `LIVE_BROADCASTING` profile only.
     - You can inject only one media stream into the channel at the same time.

     @param url Pointer to the URL address to be added to the ongoing streaming. Valid protocols are RTMP, HLS, and HTTP-FLV.
     - Supported audio codec type: AAC.
     - Supported video codec type: H264 (AVC).
     @param config Pointer to the InjectStreamConfig object that contains the configuration of the added voice or video stream.

     @return
     - 0: Success.
     - < 0: Failure.
        - #ERR_INVALID_ARGUMENT (2): The injected URL does not exist. Call this method again to inject the stream and ensure that the URL is valid.
        - #ERR_NOT_READY (3): The user is not in the channel.
        - #ERR_NOT_SUPPORTED (4): The channel profile is not `LIVE_BROADCASTING`. Call the \ref agora::rtc::IRtcEngine::setChannelProfile "setChannelProfile" method and set the channel profile to `LIVE_BROADCASTING` before calling this method.
        - #ERR_NOT_INITIALIZED (7): The SDK is not initialized. Ensure that the IRtcEngine object is initialized before calling this method.
     */
    pub fn add_inject_stream_url(&self, url: &str, config: agorartcnative::InjectStreamConfig) -> i32 {
        unsafe {
            let the_url: &CStr = &CString::new(url).expect("url new failed");
            return agorartcnative::addInjectStreamUrl(self.native_engine, the_url.as_ptr(), config);
        }
    }

    /** Starts to relay media streams across channels.
     *
     * After a successful method call, the SDK triggers the
     * \ref agora::rtc::IRtcEngineEventHandler::onChannelMediaRelayStateChanged
     *  "onChannelMediaRelayStateChanged" and
     * \ref agora::rtc::IRtcEngineEventHandler::onChannelMediaRelayEvent
     * "onChannelMediaRelayEvent" callbacks, and these callbacks return the
     * state and events of the media stream relay.
     * - If the
     * \ref agora::rtc::IRtcEngineEventHandler::onChannelMediaRelayStateChanged
     *  "onChannelMediaRelayStateChanged" callback returns
     * #RELAY_STATE_RUNNING (2) and #RELAY_OK (0), and the
     * \ref agora::rtc::IRtcEngineEventHandler::onChannelMediaRelayEvent
     * "onChannelMediaRelayEvent" callback returns
     * #RELAY_EVENT_PACKET_SENT_TO_DEST_CHANNEL (4), the host starts
     * sending data to the destination channel.
     * - If the
     * \ref agora::rtc::IRtcEngineEventHandler::onChannelMediaRelayStateChanged
     *  "onChannelMediaRelayStateChanged" callback returns
     * #RELAY_STATE_FAILURE (3), an exception occurs during the media stream
     * relay.
     *
     * @note
     * - Call this method after the \ref joinChannel() "joinChannel" method.
     * - This method takes effect only when you are a host in a
     * `LIVE_BROADCASTING` channel.
     * - After a successful method call, if you want to call this method
     * again, ensure that you call the
     * \ref stopChannelMediaRelay() "stopChannelMediaRelay" method to quit the
     * current relay.
     * - Contact sales-us@agora.io before implementing this function.
     * - We do not support string user accounts in this API.
     *
     * @param configuration The configuration of the media stream relay:
     * ChannelMediaRelayConfiguration.
     *
     * @return
     * - 0: Success.
     * - < 0: Failure.
     */
    pub fn start_channel_media_relay(&self, config: agorartcnative::ChannelMediaRelayConfiguration) -> i32 {
        unsafe {
            return agorartcnative::startChannelMediaRelay(self.native_engine, config);
        }
    }

    /** Updates the channels for media stream relay. After a successful
     * \ref startChannelMediaRelay() "startChannelMediaRelay" method call, if
     * you want to relay the media stream to more channels, or leave the
     * current relay channel, you can call the
     * \ref updateChannelMediaRelay() "updateChannelMediaRelay" method.
     *
     * After a successful method call, the SDK triggers the
     * \ref agora::rtc::IRtcEngineEventHandler::onChannelMediaRelayEvent
     *  "onChannelMediaRelayEvent" callback with the
     * #RELAY_EVENT_PACKET_UPDATE_DEST_CHANNEL (7) state code.
     *
     * @note
     * Call this method after the
     * \ref startChannelMediaRelay() "startChannelMediaRelay" method to update
     * the destination channel.
     *
     * @param configuration The media stream relay configuration:
     * ChannelMediaRelayConfiguration.
     *
     * @return
     * - 0: Success.
     * - < 0: Failure.
     */
    pub fn update_channel_media_relay(&self, config: agorartcnative::ChannelMediaRelayConfiguration) -> i32 {
        unsafe {
            return agorartcnative::updateChannelMediaRelay(self.native_engine, config);
        }
    }

    /** Stops the media stream relay.
     *
     * Once the relay stops, the host quits all the destination
     * channels.
     *
     * After a successful method call, the SDK triggers the
     * \ref agora::rtc::IRtcEngineEventHandler::onChannelMediaRelayStateChanged
     *  "onChannelMediaRelayStateChanged" callback. If the callback returns
     * #RELAY_STATE_IDLE (0) and #RELAY_OK (0), the host successfully
     * stops the relay.
     *
     * @note
     * If the method call fails, the SDK triggers the
     * \ref agora::rtc::IRtcEngineEventHandler::onChannelMediaRelayStateChanged
     *  "onChannelMediaRelayStateChanged" callback with the
     * #RELAY_ERROR_SERVER_NO_RESPONSE (2) or
     * #RELAY_ERROR_SERVER_CONNECTION_LOST (8) state code. You can leave the
     * channel by calling the \ref leaveChannel() "leaveChannel" method, and
     * the media stream relay automatically stops.
     *
     * @return
     * - 0: Success.
     * - < 0: Failure.
     */
    pub fn stop_channel_media_relay(&self) -> i32 {
        unsafe {
            return agorartcnative::stopChannelMediaRelay(self.native_engine);
        }
    }

    /** Removes the voice or video stream URL address from the live streaming.

     This method removes the URL address (added by the \ref IRtcEngine::addInjectStreamUrl "addInjectStreamUrl" method) from the live streaming.

     @note If this method is called successfully, the SDK triggers the \ref IRtcEngineEventHandler::onUserOffline "onUserOffline" callback and returns a stream uid of 666.

     @param url Pointer to the URL address of the injected stream to be removed.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn remove_inject_stream_url(&self, url: &str) -> i32 {
        unsafe {
            let the_url: &CStr = &CString::new(url).expect("url new failed");
            return agorartcnative::removeInjectStreamUrl(self.native_engine, the_url.as_ptr());
        }
    }

    /** Gets the current connection state of the SDK.

     @return #CONNECTION_STATE_TYPE.
     */
    pub fn get_connection_state(&self) -> u32 {
        unsafe {
            return agorartcnative::getConnectionState(self.native_engine);
        }
    }

    /** Provides technical preview functionalities or special customizations by configuring the SDK with JSON options.

     The JSON options are not public by default. Agora is working on making commonly used JSON options public in a standard way.

     @param parameters Sets the parameter as a JSON string in the specified format.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_parameters(&self, parameters: &str) -> i32 {
        unsafe {
            let param: &CStr = &CString::new(parameters).expect("parameters new failed");
            return agorartcnative::setParameters(self.native_engine, param.as_ptr());
        }
    }

    /** Sets the volume of the audio playback device.

     @param volume Sets the volume of the audio playback device. The value ranges between 0 (lowest volume) and 255 (highest volume).
     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_playback_device_volume(&self, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::setPlaybackDeviceVolume(self.native_engine, volume);
        }
    }

    /** Starts playing and mixing the music file.

     This method mixes the specified local audio file with the audio stream from the microphone, or replaces the microphone's audio stream with the specified local audio file. You can choose whether the other user can hear the local audio playback and specify the number of playback loops. This method also supports online music playback.

     When the audio mixing file playback finishes after calling this method, the SDK triggers the \ref agora::rtc::IRtcEngineEventHandler::onAudioMixingFinished "onAudioMixingFinished" callback.

     A successful \ref agora::rtc::IRtcEngine::startAudioMixing "startAudioMixing" method call triggers the \ref agora::rtc::IRtcEngineEventHandler::onAudioMixingStateChanged "onAudioMixingStateChanged" (PLAY) callback on the local client.

     When the audio mixing file playback finishes, the SDK triggers the \ref agora::rtc::IRtcEngineEventHandler::onAudioMixingStateChanged "onAudioMixingStateChanged" (STOPPED) callback on the local client.
     @note
     - Call this method after joining a channel, otherwise issues may occur.
     - If the local audio mixing file does not exist, or if the SDK does not support the file format or cannot access the music file URL, the SDK returns WARN_AUDIO_MIXING_OPEN_ERROR = 701.
     - If you want to play an online music file, ensure that the time interval between calling this method is more than 100 ms, or the AUDIO_MIXING_ERROR_TOO_FREQUENT_CALL(702) error code occurs.
     @param filePath Pointer to the absolute path (including the suffixes of the filename) of the local or online audio file to mix, for example, c:/music/audio.mp4. Supported audio formats: 3GP, ASF, ADTS, AVI, MP3, MP4, MPEG-4, SAMI, and WAVE. For more information, see [Supported Media Formats in Media Foundation](https://docs.microsoft.com/en-us/windows/desktop/medfound/supported-media-formats-in-media-foundation).
     @param loopback Sets which user can hear the audio mixing:
     - true: Only the local user can hear the audio mixing.
     - false: Both users can hear the audio mixing.
     @param replace Sets the audio mixing content:
     - true: Only publish the specified audio file. The audio stream from the microphone is not published.
     - false: The local audio file is mixed with the audio stream from the microphone.
     @param cycle Sets the number of playback loops:
     - Positive integer: Number of playback loops.
     - `-1`: Infinite playback loops.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn start_audio_mixing(&self, file_path: &str, loopback: bool, replace: bool, cycle: i32) -> i32 {
        unsafe {
            let the_file_path: &CStr = &CString::new(file_path).expect("file_path new failed");
            let is_loopback: i32 = if loopback { 1 } else { 0 };
            let is_replace: i32 = if replace { 1 } else { 0 };
            return agorartcnative::startAudioMixing(self.native_engine, the_file_path.as_ptr(), is_loopback, is_replace, cycle);
        }
    }

    /** Stops playing and mixing the music file.

     Call this method when you are in a channel.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn stop_audio_mixing(&self) -> i32 {
        unsafe {
            return agorartcnative::stopAudioMixing(self.native_engine);
        }
    }

    /** Pauses playing and mixing the music file.

     Call this method when you are in a channel.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn pause_audio_mixing(&self) -> i32 {
        unsafe {
            return agorartcnative::pauseAudioMixing(self.native_engine);
        }
    }

    /** Resumes playing and mixing the music file.

     Call this method when you are in a channel.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn resume_audio_mixing(&self) -> i32 {
        unsafe {
            return agorartcnative::resumeAudioMixing(self.native_engine);
        }
    }

    /** **DEPRECATED** Agora does not recommend using this method.

     Sets the high-quality audio preferences. Call this method and set all parameters before joining a channel.

     Do not call this method again after joining a channel.

     @param fullband Sets whether to enable/disable full-band codec (48-kHz sample rate). Not compatible with SDK versions before v1.7.4:
     - true: Enable full-band codec.
     - false: Disable full-band codec.
     @param  stereo Sets whether to enable/disable stereo codec. Not compatible with SDK versions before v1.7.4:
     - true: Enable stereo codec.
     - false: Disable stereo codec.
     @param fullBitrate Sets whether to enable/disable high-bitrate mode. Recommended in voice-only mode:
     - true: Enable high-bitrate mode.
     - false: Disable high-bitrate mode.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_high_quality_audio_parameters(&self, fullband: bool, stereo: bool, full_bitrate: bool) -> i32 {
        unsafe {
            let is_full_band: i32 = if fullband { 1 } else { 0 };
            let is_stereo: i32 = if stereo { 1 } else { 0 };
            let is_full_bitrate: i32 = if full_bitrate { 1 } else { 0 };
            return agorartcnative::setHighQualityAudioParameters(self.native_engine, is_full_band, is_stereo, is_full_bitrate);
        }
    }

    /** Adjusts the volume during audio mixing.

     Call this method when you are in a channel.

     @note Calling this method does not affect the volume of audio effect file playback invoked by the \ref agora::rtc::IRtcEngine::playEffect "playEffect" method.

     @param volume Audio mixing volume. The value ranges between 0 and 100 (default).

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn adjust_audio_mixing_volume(&self, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::adjustAudioMixingVolume(self.native_engine, volume);
        }
    }

    /** Adjusts the audio mixing volume for local playback.

     @note Call this method when you are in a channel.

     @param volume Audio mixing volume for local playback. The value ranges between 0 and 100 (default).

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn adjust_audio_mixing_playout_volume(&self, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::adjustAudioMixingPlayoutVolume(self.native_engine, volume);
        }
    }

    /** Retrieves the audio mixing volume for local playback.

     This method helps troubleshoot audio volume related issues.

     @note Call this method when you are in a channel.

     @return
     - &ge; 0: The audio mixing volume, if this method call succeeds. The value range is [0,100].
     - < 0: Failure.
     */
    pub fn get_audio_mixing_playout_volume(&self) -> i32 {
        unsafe {
            return agorartcnative::getAudioMixingPlayoutVolume(self.native_engine);
        }
    }

    /** Adjusts the audio mixing volume for publishing (for remote users).

     @note Call this method when you are in a channel.

     @param volume Audio mixing volume for publishing. The value ranges between 0 and 100 (default).

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn adjust_audio_mixing_publish_volume(&self, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::adjustAudioMixingPublishVolume(self.native_engine, volume);
        }
    }

    /** Retrieves the audio mixing volume for publishing.

     This method helps troubleshoot audio volume related issues.

     @note Call this method when you are in a channel.

     @return
     - &ge; 0: The audio mixing volume for publishing, if this method call succeeds. The value range is [0,100].
     - < 0: Failure.
     */
    pub fn get_audio_mixing_publish_volume(&self) -> i32 {
        unsafe {
            return agorartcnative::getAudioMixingPublishVolume(self.native_engine);
        }
    }

    /** Retrieves the duration (ms) of the music file.

     Call this method when you are in a channel.

     @return
     - &ge; 0: The audio mixing duration, if this method call succeeds.
     - < 0: Failure.
     */
    pub fn get_audio_mixing_duration(&self) -> i32 {
        unsafe {
            return agorartcnative::getAudioMixingDuration(self.native_engine);
        }
    }

    /** Retrieves the playback position (ms) of the music file.

     Call this method when you are in a channel.

     @return
     - &ge; 0: The current playback position of the audio mixing, if this method call succeeds.
     - < 0: Failure.
     */
    pub fn get_audio_mixing_current_position(&self) -> i32 {
        unsafe {
            return agorartcnative::getAudioMixingCurrentPosition(self.native_engine);
        }
    }

    /** Sets the playback position of the music file to a different starting position (the default plays from the beginning).

     @param pos The playback starting position (ms) of the music file.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_audion_mixing_position(&self, pos: i32) -> i32 {
        unsafe {
            return agorartcnative::setAudioMixingPosition(self.native_engine, pos);
        }
    }

    /** Sets the pitch of the local music file.
     * @since v3.0.1
     *
     * When a local music file is mixed with a local human voice, call this method to set the pitch of the local music file only.
     *
     * @note
     * Call this method after calling `startAudioMixing`.
     *
     * @param pitch Sets the pitch of the local music file by chromatic scale. The default value is 0,
     * which means keeping the original pitch. The value ranges from -12 to 12, and the pitch value between
     * consecutive values is a chromatic value. The greater the absolute value of this parameter, the
     * higher or lower the pitch of the local music file.
     *
     * @return
     * - 0: Success.
     * - < 0: Failure.
     */
    pub fn set_audio_mixing_pitch(&self, pitch: i32) -> i32 {
        unsafe {
            return agorartcnative::setAudioMixingPitch(self.native_engine, pitch);
        }
    }

    /** Retrieves the volume of the audio effects.

     The value ranges between 0.0 and 100.0.

     @return
     - &ge; 0: Volume of the audio effects, if this method call succeeds.

     - < 0: Failure.
     */
    pub fn get_effects_volume(&self) -> i32 {
        unsafe {
            return agorartcnative::getEffectsVolume(self.native_engine);
        }
    }

    /** Sets the volume of the audio effects.

     @param volume Sets the volume of the audio effects. The value ranges between 0 and 100 (default).

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_effect_volume(&self, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::setEffectsVolume(self.native_engine, volume);
        }
    }

    /** Sets the volume of a specified audio effect.

     @param soundId ID of the audio effect. Each audio effect has a unique ID.
     @param volume Sets the volume of the specified audio effect. The value ranges between 0 and 100 (default).

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_volume_of_effect(&self, sound_id: i32, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::setVolumeOfEffect(self.native_engine, sound_id, volume);
        }
    }

    /** Plays a specified local or online audio effect file.

     This method allows you to set the loop count, pitch, pan, and gain of the audio effect file, as well as whether the remote user can hear the audio effect.

     To play multiple audio effect files simultaneously, call this method multiple times with different soundIds and filePaths. We recommend playing no more than three audio effect files at the same time.

     @param soundId ID of the specified audio effect. Each audio effect has a unique ID.

     @note
     - If the audio effect is preloaded into the memory through the \ref IRtcEngine::preloadEffect "preloadEffect" method, the value of @p soundID must be the same as that in the *preloadEffect* method.
     - Playing multiple online audio effect files simultaneously is not supported on macOS and Windows.

     @param filePath Specifies the absolute path (including the suffixes of the filename) to the local audio effect file or the URL of the online audio effect file, for example, c:/music/audio.mp4. Supported audio formats: mp3, mp4, m4a, aac, 3gp, mkv and wav.
     @param loopCount Sets the number of times the audio effect loops:
     - 0: Play the audio effect once.
     - 1: Play the audio effect twice.
     - -1: Play the audio effect in an indefinite loop until the \ref IRtcEngine::stopEffect "stopEffect" or \ref IRtcEngine::stopAllEffects "stopAllEffects" method is called.
     @param pitch Sets the pitch of the audio effect. The value ranges between 0.5 and 2. The default value is 1 (no change to the pitch). The lower the value, the lower the pitch.
     @param pan Sets the spatial position of the audio effect. The value ranges between -1.0 and 1.0:
     - 0.0: The audio effect displays ahead.
     - 1.0: The audio effect displays to the right.
     - -1.0: The audio effect displays to the left.
     @param gain  Sets the volume of the audio effect. The value ranges between 0 and 100 (default). The lower the value, the lower the volume of the audio effect.
     @param publish Sets whether or not to publish the specified audio effect to the remote stream:
     - true: The locally played audio effect is published to the Agora Cloud and the remote users can hear it.
     - false: The locally played audio effect is not published to the Agora Cloud and the remote users cannot hear it.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn play_effect(&self, sound_id: i32, file_path: &str, loop_count: i32, pitch: f64, pan: f64, gain: i32, publish: bool) -> i32 {
        unsafe {
            let the_file_path: &CStr = &CString::new(file_path).expect("file_path new failed");
            let is_publish: i32 = if publish { 1 } else { 0 };
            return agorartcnative::playEffect(self.native_engine, sound_id, the_file_path.as_ptr(), loop_count, pitch, pan, gain, is_publish);
        }
    }

    /** Stops playing a specified audio effect.

     @param soundId ID of the audio effect to stop playing. Each audio effect has a unique ID.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn stop_effect(&self, sound_id: i32) -> i32 {
        unsafe {
            return agorartcnative::stopEffect(self.native_engine, sound_id);
        }
    }

    /** Stops playing all audio effects.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn stop_all_effect(&self) -> i32 {
        unsafe {
            return agorartcnative::stopAllEffects(self.native_engine);
        }
    }

    /** Preloads a specified audio effect file into the memory.

     @note This method does not support online audio effect files.

     To ensure smooth communication, limit the size of the audio effect file. We recommend using this method to preload the audio effect before calling the \ref IRtcEngine::joinChannel "joinChannel" method.

     Supported audio formats: mp3, aac, m4a, 3gp, and wav.

     @param soundId ID of the audio effect. Each audio effect has a unique ID.
     @param filePath Pointer to the absolute path of the audio effect file.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn preload_effect(&self, sound_id: i32, file_path: &str) -> i32 {
        unsafe {
            let the_file_path: &CStr = &CString::new(file_path).expect("file_path new failed");
            return agorartcnative::preloadEffect(self.native_engine, sound_id, the_file_path.as_ptr());
        }
    }

    /** Releases a specified preloaded audio effect from the memory.

     @param soundId ID of the audio effect. Each audio effect has a unique ID.
     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn unload_effect(&self, sound_id: i32) -> i32 {
        unsafe {
            return agorartcnative::unloadEffect(self.native_engine, sound_id);
        }
    }

    /** Pauses a specified audio effect.

     @param soundId ID of the audio effect. Each audio effect has a unique ID.
     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn pause_effect(&self, sound_id: i32) -> i32 {
        unsafe {
            return agorartcnative::pauseEffect(self.native_engine, sound_id);
        }
    }

    /** Pauses all audio effects.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn pause_all_effect(&self) -> i32 {
        unsafe {
            return agorartcnative::pauseAllEffects(self.native_engine);
        }
    }

    /** Resumes playing a specified audio effect.

     @param soundId ID of the audio effect. Each audio effect has a unique ID.
     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn resume_effect(&self, sound_id: i32) -> i32 {
        unsafe {
            return agorartcnative::resumeEffect(self.native_engine, sound_id);
        }
    }

    /** Resumes playing all audio effects.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn resume_all_effects(&self) -> i32 {
        unsafe {
            return agorartcnative::resumeAllEffects(self.native_engine);
        }
    }

    /** Enables/Disables stereo panning for remote users.

     Ensure that you call this method before joinChannel to enable stereo panning for remote users so that the local user can track the position of a remote user by calling \ref agora::rtc::IRtcEngine::setRemoteVoicePosition "setRemoteVoicePosition".

     @param enabled Sets whether or not to enable stereo panning for remote users:
     - true: enables stereo panning.
     - false: disables stereo panning.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn enable_sound_position_indication(&self, enabled: bool) -> i32 {
        unsafe {
            let is_enabled: i32 = if enabled { 1 } else { 0 };
            return agorartcnative::enableSoundPositionIndication(self.native_engine, is_enabled);
        }
    }

    /** Changes the voice pitch of the local speaker.

     @param pitch Sets the voice pitch. The value ranges between 0.5 and 2.0. The lower the value, the lower the voice pitch. The default value is 1.0 (no change to the local voice pitch).
     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_local_voice_pitch(&self, pitch: f64) -> i32 {
        unsafe {
            return agorartcnative::setLocalVoicePitch(self.native_engine, pitch);
        }
    }

    /** Sets the local voice equalization effect.

     @param bandFrequency Sets the band frequency. The value ranges between 0 and 9, representing the respective 10-band center frequencies of the voice effects, including 31, 62, 125, 500, 1k, 2k, 4k, 8k, and 16k Hz. See #AUDIO_EQUALIZATION_BAND_FREQUENCY.
     @param bandGain  Sets the gain of each band in dB. The value ranges between -15 and 15.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_local_voice_equalization(&self, band_frequency: AUDIO_EQUALIZATION_BAND_FREQUENCY, band_gain: i32) -> i32 {
        unsafe {
            return agorartcnative::setLocalVoiceEqualization(self.native_engine, band_frequency as u32, band_gain);
        }
    }

    /**  Sets the local voice reverberation.

     v2.4.0 adds the \ref agora::rtc::IRtcEngine::setLocalVoiceReverbPreset "setLocalVoiceReverbPreset" method, a more user-friendly method for setting the local voice reverberation. You can use this method to set the local reverberation effect, such as pop music, R&B, rock music, and hip-hop.

     @param reverbKey Sets the reverberation key. See #AUDIO_REVERB_TYPE.
     @param value Sets the value of the reverberation key.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_local_voice_reverb(&self, reverb_key: AUDIO_REVERB_TYPE, value: i32) -> i32 {
        unsafe {
            return agorartcnative::setLocalVoiceReverb(self.native_engine, reverb_key as u32, value);
        }
    }

    /** Sets the local voice changer option.

     This method can be used to set the local voice effect for users in a `COMMUNICATION` channel or hosts in a `LIVE_BROADCASTING` channel.
     Voice changer options include the following voice effects:

     - `VOICE_CHANGER_XXX`: Changes the local voice to an old man, a little boy, or the Hulk. Applies to the voice talk scenario.
     - `VOICE_BEAUTY_XXX`: Beautifies the local voice by making it sound more vigorous, resounding, or adding spacial resonance. Applies to the voice talk and singing scenario.
     - `GENERAL_VOICE_BEAUTY_XXX`: Adds gender-based beautification effect to the local voice. Applies to the voice talk scenario.
       - For a male voice: Adds magnetism to the voice.
       - For a female voice: Adds freshness or vitality to the voice.

     @note
     - To achieve better voice effect quality, Agora recommends setting the profile parameter in `setAudioProfile` as `AUDIO_PROFILE_MUSIC_HIGH_QUALITY(4)` or `AUDIO_PROFILE_MUSIC_HIGH_QUALITY_STEREO(5)`.
     - This method works best with the human voice, and Agora does not recommend using it for audio containing music and a human voice.
     - Do not use this method with `setLocalVoiceReverbPreset`, because the method called later overrides the one called earlier. For detailed considerations, see the advanced guide *Voice Changer and Reverberation*.

     @param voiceChanger Sets the local voice changer option. The default value is `VOICE_CHANGER_OFF`, which means the original voice. See details in #VOICE_CHANGER_PRESET.
     Gender-based beatification effect works best only when assigned a proper gender:
     - For male: `GENERAL_BEAUTY_VOICE_MALE_MAGNETIC`.
     - For female: `GENERAL_BEAUTY_VOICE_FEMALE_FRESH` or `GENERAL_BEAUTY_VOICE_FEMALE_VITALITY`.
     Failure to do so can lead to voice distortion.

     @return
     - 0: Success.
     - < 0: Failure. Check if the enumeration is properly set.
     */
    pub fn set_local_voice_changer(&self, voice_changer: VOICE_CHANGER_PRESET) -> i32 {
        unsafe {
            return agorartcnative::setLocalVoiceChanger(self.native_engine, voice_changer as u32);
        }
    }

    /** Sets the local voice reverberation option, including the virtual stereo.
     *
     * This method sets the local voice reverberation for users in a `COMMUNICATION` channel or hosts in a `LIVE_BROADCASTING` channel.
     * After successfully calling this method, all users in the channel can hear the voice with reverberation.
     *
     * @note
     * - When calling this method with enumerations that begin with `AUDIO_REVERB_FX`, ensure that you set profile in `setAudioProfile` as
     * `AUDIO_PROFILE_MUSIC_HIGH_QUALITY(4)` or `AUDIO_PROFILE_MUSIC_HIGH_QUALITY_STEREO(5)`; otherwise, this methods cannot set the corresponding voice reverberation option.
     * - When calling this method with `AUDIO_VIRTUAL_STEREO`, Agora recommends setting the `profile` parameter in `setAudioProfile` as `AUDIO_PROFILE_MUSIC_HIGH_QUALITY_STEREO(5)`.
     * - This method works best with the human voice, and Agora does not recommend using it for audio containing music and a human voice.
     * - Do not use this method with `setLocalVoiceChanger`, because the method called later overrides the one called earlier.
     * For detailed considerations, see the advanced guide *Voice Changer and Reverberation*.

     @param reverbPreset The local voice reverberation option. The default value is `AUDIO_REVERB_OFF`,
     which means the original voice.  See #AUDIO_REVERB_PRESET.
     To achieve better voice effects, Agora recommends the enumeration whose name begins with `AUDIO_REVERB_FX`.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_local_voice_reverb_preset(&self, reverb_preset: AUDIO_REVERB_PRESET) -> i32 {
        unsafe {
            return agorartcnative::setLocalVoiceReverbPreset(self.native_engine, reverb_preset as u32);
        }
    }

    /** Sets the external audio source. Please call this method before \ref agora::rtc::IRtcEngine::joinChannel "joinChannel".

     @param enabled Sets whether to enable/disable the external audio source:
     - true: Enables the external audio source.
     - false: (Default) Disables the external audio source.
     @param sampleRate Sets the sample rate (Hz) of the external audio source, which can be set as 8000, 16000, 32000, 44100, or 48000 Hz.
     @param channels Sets the number of audio channels of the external audio source:
     - 1: Mono.
     - 2: Stereo.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_external_audio_source(&self, enabled: bool, sample_rate: i32, channels: i32) -> i32 {
        unsafe {
            let is_enabled: i32 = if enabled { 1 } else { 0 };
            return agorartcnative::setExternalAudioSource(self.native_engine, is_enabled, sample_rate, channels);
        }
    }

    /** Sets the external audio sink.
     * This method applies to scenarios where you want to use external audio
     * data for playback. After enabling the external audio sink, you can call
     * the \ref agora::media::IMediaEngine::pullAudioFrame "pullAudioFrame" method to pull the remote audio data, process
     * it, and play it with the audio effects that you want.
     *
     * @note
     * Once you enable the external audio sink, the app will not retrieve any
     * audio data from the
     * \ref agora::media::IAudioFrameObserver::onPlaybackAudioFrame "onPlaybackAudioFrame" callback.
     *
     * @param enabled
     * - true: Enables the external audio sink.
     * - false: (Default) Disables the external audio sink.
     * @param sampleRate Sets the sample rate (Hz) of the external audio sink, which can be set as 16000, 32000, 44100 or 48000.
     * @param channels Sets the number of audio channels of the external
     * audio sink:
     * - 1: Mono.
     * - 2: Stereo.
     *
     * @return
     * - 0: Success.
     * - < 0: Failure.
     */
    pub fn set_external_audio_sink(&self, enabled: bool, sample_rate: i32, channels: i32) -> i32 {
        unsafe {
            let is_enabled: i32 = if enabled { 1 } else { 0 };
            return agorartcnative::setExternalAudioSink(self.native_engine, is_enabled, sample_rate, channels);
        }
    }

    /** Sets the audio recording format for the \ref agora::media::IAudioFrameObserver::onRecordAudioFrame "onRecordAudioFrame" callback.


     @param sampleRate Sets the sample rate (@p samplesPerSec) returned in the *onRecordAudioFrame* callback, which can be set as 8000, 16000, 32000, 44100, or 48000 Hz.
     @param channel Sets the number of audio channels (@p channels) returned in the *onRecordAudioFrame* callback:
     - 1: Mono
     - 2: Stereo
     @param mode Sets the use mode (see #RAW_AUDIO_FRAME_OP_MODE_TYPE) of the *onRecordAudioFrame* callback.
     @param samplesPerCall Sets the number of samples returned in the *onRecordAudioFrame* callback. `samplesPerCall` is usually set as 1024 for RTMP streaming.


     @note The SDK triggers the `onRecordAudioFrame` callback according to the sample interval. Ensure that the sample interval ≥ 0.01 (s). And, Sample interval (sec) = `samplePerCall`/(`sampleRate` × `channel`).

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_recoding_audio_frame_parameters(&self, sample_rate: i32, channel: i32, mode: RAW_AUDIO_FRAME_OP_MODE_TYPE, samples_per_call: i32) -> i32 {
        unsafe {
            return agorartcnative::setRecordingAudioFrameParameters(self.native_engine, sample_rate, channel, mode as u32, samples_per_call);
        }
    }

    /** Sets the audio playback format for the \ref agora::media::IAudioFrameObserver::onPlaybackAudioFrame "onPlaybackAudioFrame" callback.


     @param sampleRate Sets the sample rate (@p samplesPerSec) returned in the *onPlaybackAudioFrame* callback, which can be set as 8000, 16000, 32000, 44100, or 48000 Hz.
     @param channel Sets the number of channels (@p channels) returned in the *onPlaybackAudioFrame* callback:
     - 1: Mono
     - 2: Stereo
     @param mode Sets the use mode (see #RAW_AUDIO_FRAME_OP_MODE_TYPE) of the *onPlaybackAudioFrame* callback.
     @param samplesPerCall Sets the number of samples returned in the *onPlaybackAudioFrame* callback. `samplesPerCall` is usually set as 1024 for RTMP streaming.

     @note The SDK triggers the `onPlaybackAudioFrame` callback according to the sample interval. Ensure that the sample interval ≥ 0.01 (s). And, Sample interval (sec) = `samplePerCall`/(`sampleRate` × `channel`).

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_playback_audio_frame_parameters(&self, sample_rate: i32, channel: i32, mode: RAW_AUDIO_FRAME_OP_MODE_TYPE, sample_per_call: i32) -> i32 {
        unsafe {
            return agorartcnative::setPlaybackAudioFrameParameters(self.native_engine, sample_rate, channel, mode as u32, sample_per_call);
        }
    }

    /** Sets the mixed audio format for the \ref agora::media::IAudioFrameObserver::onMixedAudioFrame "onMixedAudioFrame" callback.


     @param sampleRate Sets the sample rate (@p samplesPerSec) returned in the *onMixedAudioFrame* callback, which can be set as 8000, 16000, 32000, 44100, or 48000 Hz.
     @param samplesPerCall Sets the number of samples (`samples`) returned in the *onMixedAudioFrame* callback. `samplesPerCall` is usually set as 1024 for RTMP streaming.

     @note The SDK triggers the `onMixedAudioFrame` callback according to the sample interval. Ensure that the sample interval ≥ 0.01 (s). And, Sample interval (sec) = `samplePerCall`/(`sampleRate` × `channels`).

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_mixed_audio_frame_parameters(&self, sample_rate: i32, samples_per_call: i32) -> i32 {
        unsafe {
            return agorartcnative::setMixedAudioFrameParameters(self.native_engine, sample_rate, samples_per_call);
        }
    }

    /** Enables/Disables the built-in encryption.
     *
     * @since v3.1.0
     *
     * In scenarios requiring high security, Agora recommends calling this method to enable the built-in encryption before joining a channel.
     *
     * All users in the same channel must use the same encryption mode and encryption key. Once all users leave the channel, the encryption key of this channel is automatically cleared.
     *
     * @note
     * - If you enable the built-in encryption, you cannot use the RTMP streaming function.
     * - Agora supports four encryption modes. If you choose an encryption mode (excepting `SM4_128_ECB` mode), you need to add an external encryption library when integrating the SDK. See the advanced guide *Channel Encryption*.
     *
     * @param enabled Whether to enable the built-in encryption:
     * - true: Enable the built-in encryption.
     * - false: Disable the built-in encryption.
     * @param config Configurations of built-in encryption schemas. See EncryptionConfig.
     *
     * @return
     * - 0: Success.
     * - < 0: Failure.
     *  - -2(ERR_INVALID_ARGUMENT): An invalid parameter is used. Set the parameter with a valid value.
     *  - -4(ERR_NOT_SUPPORTED): The encryption mode is incorrect or the SDK fails to load the external encryption library. Check the enumeration or reload the external encryption library.
     *  - -7(ERR_NOT_INITIALIZED): The SDK is not initialized. Initialize the `IRtcEngine` instance before calling this method.
     */
    pub fn enable_encryption(&self, enabled: bool, config: agorartcnative::EncryptionConfig) -> i32 {
        unsafe {
            let is_enabled: i32 = if enabled { 1 } else { 0 };
            return agorartcnative::enableEncryption(self.native_engine, is_enabled, config);
        }
    }
}

impl AgoraRtcChannel {
    /** Sets the channel event handler.

     After setting the channel event handler, you can listen for channel events and receive the statistics of the corresponding `IChannel` object.

     @param channelEh The event handler of the `IChannel` object. For details, see IChannelEventHandler.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn add_channel_event_handler(&self, handler: *mut agorartcnative::ChannelEventHandler) {
        unsafe {
            return agorartcnative::channel_add_C_ChannelEventHandler(self.native_channel, handler);
        }
    }

    pub fn remove_channel_event_handler(&self) {
        unsafe {
            return agorartcnative::channel_remove_C_ChannelEventHandler(self.native_channel);
        }
    }

    /** Joins the channel with a user ID.

     This method differs from the `joinChannel` method in the `IRtcEngine` class in the following aspects:

     | IChannel::joinChannel                                                                                                                    | IRtcEngine::joinChannel                                                                                      |
     |------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------|
     | Does not contain the `channelId` parameter, because `channelId` is specified when creating the `IChannel` object.                              | Contains the `channelId` parameter, which specifies the channel to join.                                       |
     | Contains the `options` parameter, which decides whether to subscribe to all streams before joining the channel.                            | Does not contain the `options` parameter. By default, users subscribe to all streams when joining the channel. |
     | Users can join multiple channels simultaneously by creating multiple `IChannel` objects and calling the `joinChannel` method of each object. | Users can join only one channel.                                                                             |
     | By default, the SDK does not publish any stream after the user joins the channel. You need to call the publish method to do that.        | By default, the SDK publishes streams once the user joins the channel.                                       |

     @note
     - If you are already in a channel, you cannot rejoin it with the same `uid`.
     - We recommend using different UIDs for different channels.
     - If you want to join the same channel from different devices, ensure that the UIDs in all devices are different.
     - Ensure that the app ID you use to generate the token is the same with the app ID used when creating the `IRtcEngine` object.

     @param token The token for authentication:
     - In situations not requiring high security: You can use the temporary token generated at Console. For details, see [Get a temporary token](https://docs.agora.io/en/Agora%20Platform/token?platfor%20*%20m=All%20Platforms#get-a-temporary-token).
     - In situations requiring high security: Set it as the token generated at your server. For details, see [Generate a token](https://docs.agora.io/en/Agora%20Platform/token?platfor%20*%20m=All%20Platforms#get-a-token).
     @param info (Optional) Additional information about the channel. This parameter can be set as null. Other users in the channel do not receive this information.
     @param uid The user ID. A 32-bit unsigned integer with a value ranging from 1 to (232-1). This parameter must be unique. If `uid` is not assigned (or set as `0`), the SDK assigns a `uid` and reports it in the \ref agora::rtc::IChannelEventHandler::onJoinChannelSuccess "onJoinChannelSuccess" callback. The app must maintain this user ID.
     @param options The channel media options: \ref agora::rtc::ChannelMediaOptions::ChannelMediaOptions "ChannelMediaOptions"

     @return
     - 0(ERR_OK): Success.
     - < 0: Failure.
        - -2(ERR_INALID_ARGUMENT): The parameter is invalid.
        - -3(ERR_NOT_READY): The SDK fails to be initialized. You can try re-initializing the SDK.
        - -5(ERR_REFUSED): The request is rejected. This may be caused by the following:
           - You have created an IChannel object with the same channel name.
           - You have joined and published a stream in a channel created by the IChannel object.
     */
    pub fn join_channel(&self, token: &str, info: &str, uid: u32, options: agorartcnative::ChannelMediaOptions) -> i32 {
        unsafe {
            let the_token: &CStr = &CString::new(token).expect("token new failed");
            let the_info: &CStr = &CString::new(info).expect("info new failed");
            return agorartcnative::channel_joinChannel(self.native_channel, the_token.as_ptr(), the_info.as_ptr(), uid, options);
        }
    }

    /** Joins the channel with a user account.

     After the user successfully joins the channel, the SDK triggers the following callbacks:

     - The local client: \ref agora::rtc::IRtcEngineEventHandler::onLocalUserRegistered "onLocalUserRegistered" and \ref agora::rtc::IChannelEventHandler::onJoinChannelSuccess "onJoinChannelSuccess" .
     - The remote client: \ref agora::rtc::IChannelEventHandler::onUserJoined "onUserJoined" and \ref agora::rtc::IRtcEngineEventHandler::onUserInfoUpdated "onUserInfoUpdated" , if the user joining the channel is in the `COMMUNICATION` profile, or is a host in the `LIVE_BROADCASTING` profile.

     @note To ensure smooth communication, use the same parameter type to identify the user. For example, if a user joins the channel with a user ID, then ensure all the other users use the user ID too. The same applies to the user account.
     If a user joins the channel with the Agora Web SDK, ensure that the uid of the user is set to the same parameter type.

     @param token The token generated at your server:
     - For low-security requirements: You can use the temporary token generated at Console. For details, see [Get a temporary toke](https://docs.agora.io/en/Voice/token?platform=All%20Platforms#get-a-temporary-token).
     - For high-security requirements: Set it as the token generated at your server. For details, see [Get a token](https://docs.agora.io/en/Voice/token?platform=All%20Platforms#get-a-token).
     @param userAccount The user account. The maximum length of this parameter is 255 bytes. Ensure that you set this parameter and do not set it as null. Supported character scopes are:
     - All lowercase English letters: a to z.
     - All uppercase English letters: A to Z.
     - All numeric characters: 0 to 9.
     - The space character.
     - Punctuation characters and other symbols, including: "!", "#", "$", "%", "&", "(", ")", "+", "-", ":", ";", "<", "=", ".", ">", "?", "@", "[", "]", "^", "_", " {", "}", "|", "~", ",".
     @param options The channel media options: \ref agora::rtc::ChannelMediaOptions::ChannelMediaOptions “ChannelMediaOptions”.

     @return
     - 0: Success.
     - < 0: Failure.
        - #ERR_INVALID_ARGUMENT (-2)
        - #ERR_NOT_READY (-3)
        - #ERR_REFUSED (-5)
     */
    pub fn join_channel_with_user_account(&self, token: &str, user_account: &str, options: agorartcnative::ChannelMediaOptions) -> i32 {
        unsafe {
            let the_token: &CStr = &CString::new(token).expect("token new failed");
            let the_user_account = &CString::new(user_account).expect("user_account new failed");
            return agorartcnative::channel_joinChannelWithUserAccount(self.native_channel, the_token.as_ptr(), the_user_account.as_ptr(), options);
        }
    }

    /** Allows a user to leave a channel, such as hanging up or exiting a call.

     After joining a channel, the user must call the *leaveChannel* method to end the call before joining another channel.

     This method returns 0 if the user leaves the channel and releases all resources related to the call.

     This method call is asynchronous, and the user has not left the channel when the method call returns. Once the user leaves the channel, the SDK triggers the \ref IChannelEventHandler::onLeaveChannel "onLeaveChannel" callback.

     A successful \ref agora::rtc::IChannel::leaveChannel "leaveChannel" method call triggers the following callbacks:
     - The local client: \ref agora::rtc::IChannelEventHandler::onLeaveChannel "onLeaveChannel"
     - The remote client: \ref agora::rtc::IChannelEventHandler::onUserOffline "onUserOffline" , if the user leaving the channel is in the Communication channel, or is a host in the `LIVE_BROADCASTING` profile.

     @note
     - If you call the \ref IChannel::release "release" method immediately after the *leaveChannel* method, the *leaveChannel* process interrupts, and the \ref IChannelEventHandler::onLeaveChannel "onLeaveChannel" callback is not triggered.
     - If you call the *leaveChannel* method during a CDN live streaming, the SDK triggers the \ref IChannel::removePublishStreamUrl "removePublishStreamUrl" method.

     @return
     - 0(ERR_OK): Success.
     - < 0: Failure.
        - -1(ERR_FAILED): A general error occurs (no specified reason).
        - -2(ERR_INALID_ARGUMENT): The parameter is invalid.
        - -7(ERR_NOT_INITIALIZED): The SDK is not initialized.
     */
    pub fn leave_channel(&self) -> i32 {
        unsafe {
            return agorartcnative::channel_leaveChannel(self.native_channel);
        }
    }

    /** Publishes the local stream to the channel.

     You must keep the following restrictions in mind when calling this method. Otherwise, the SDK returns the #ERR_REFUSED (5):
     - This method publishes one stream only to the channel corresponding to the current `IChannel` object.
     - In the live interactive streaming channel, only a host can call this method. To switch the client role, call \ref agora::rtc::IChannel::setClientRole "setClientRole" of the current `IChannel` object.
     - You can publish a stream to only one channel at a time. For details on joining multiple channels, see the advanced guide *Join Multiple Channels*.

     @return
     - 0: Success.
     - < 0: Failure.
        - #ERR_REFUSED (5): The method call is refused.
     */
    pub fn publish(&self) -> i32 {
        unsafe {
            return agorartcnative::channel_publish(self.native_channel);
        }
    }

    /** Stops publishing a stream to the channel.

     If you call this method in a channel where you are not publishing streams, the SDK returns #ERR_REFUSED (5).

     @return
     - 0: Success.
     - < 0: Failure.
        - #ERR_REFUSED (5): The method call is refused.
     */
    pub fn unpublish(&self) -> i32 {
        unsafe {
            return agorartcnative::channel_unpublish(self.native_channel);
        }
    }

    /** Gets the channel ID of the current `IChannel` object.

     @return
     - The channel ID of the current `IChannel` object, if the method call succeeds.
     - The empty string "", if the method call fails.
     */
    pub fn get_channel_id(&self) -> String {
        unsafe {
            let s = agorartcnative::channel_channelId(self.native_channel);
            let r = CStr::from_ptr(s).to_string_lossy().into_owned();
            return r;
        }
    }

    /** Retrieves the current call ID.

     When a user joins a channel on a client, a `callId` is generated to identify the call from the client.
     Feedback methods, such as \ref IRtcEngine::rate "rate" and \ref IRtcEngine::complain "complain", must be called after the call ends to submit feedback to the SDK.

     The `rate` and `complain` methods require the `callId` parameter retrieved from the `getCallId` method during a call. `callId` is passed as an argument into the `rate` and `complain` methods after the call ends.

     @param callId The current call ID.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn get_call_id(&self) -> String {
        unsafe {
            let s = agorartcnative::channel_getCallId(self.native_channel);
            let r = CStr::from_ptr(s).to_string_lossy().into_owned();
            return r;
        }
    }

    /** Gets a new token when the current token expires after a period of time.

     The `token` expires after a period of time once the token schema is enabled when:

     - The SDK triggers the \ref IChannelEventHandler::onTokenPrivilegeWillExpire "onTokenPrivilegeWillExpire" callback, or
     - The \ref IChannelEventHandler::onConnectionStateChanged "onConnectionStateChanged" reports CONNECTION_CHANGED_TOKEN_EXPIRED(9).

     The application should call this method to get the new `token`. Failure to do so will result in the SDK disconnecting from the server.

     @param token Pointer to the new token.

     @return
     - 0(ERR_OK): Success.
     - < 0: Failure.
        - -1(ERR_FAILED): A general error occurs (no specified reason).
        - -2(ERR_INALID_ARGUMENT): The parameter is invalid.
        - -7(ERR_NOT_INITIALIZED): The SDK is not initialized.
     */
    pub fn renew_token(&self, token: &str) -> i32 {
        unsafe {
            let the_token: &CStr = &CString::new(token).expect("token new failed");
            return agorartcnative::channel_renewToken(self.native_channel, the_token.as_ptr());
        }
    }

    /** Enables built-in encryption with an encryption password before users join a channel.

     @deprecated Deprecated as of v3.1.0. Use the \ref agora::rtc::IChannel::enableEncryption "enableEncryption" instead.

     All users in a channel must use the same encryption password. The encryption password is automatically cleared once a user leaves the channel.

     If an encryption password is not specified, the encryption functionality will be disabled.

     @note
     - Do not use this method for CDN live streaming.
     - For optimal transmission, ensure that the encrypted data size does not exceed the original data size + 16 bytes. 16 bytes is the maximum padding size for AES encryption.

     @param secret Pointer to the encryption password.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_encryption_secret(&self, secret: &str) -> i32 {
        unsafe {
            let the_secret: &CStr = &CString::new(secret).expect("secret new failed");
            return agorartcnative::channel_setEncryptionSecret(self.native_channel, the_secret.as_ptr());
        }
    }

    /** Sets the built-in encryption mode.

     @deprecated Deprecated as of v3.1.0. Use the \ref agora::rtc::IChannel::enableEncryption "enableEncryption" instead.

     The Agora SDK supports built-in encryption, which is set to the `aes-128-xts` mode by default. Call this method to use other encryption modes.

     All users in the same channel must use the same encryption mode and password.

     Refer to the information related to the AES encryption algorithm on the differences between the encryption modes.

     @note Call the \ref IChannel::setEncryptionSecret "setEncryptionSecret" method to enable the built-in encryption function before calling this method.

     @param encryptionMode The set encryption mode:
     - "aes-128-xts": (Default) 128-bit AES encryption, XTS mode.
     - "aes-128-ecb": 128-bit AES encryption, ECB mode.
     - "aes-256-xts": 256-bit AES encryption, XTS mode.
     - "": When encryptionMode is set as NULL, the encryption mode is set as "aes-128-xts" by default.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_encryption_mode(&self, encryption_mode: &str) -> i32 {
        unsafe {
            let the_encryption_mode: &CStr = &CString::new(encryption_mode).expect("encryption_mode new failed");
            return agorartcnative::channel_setEncryptionMode(self.native_channel, the_encryption_mode.as_ptr());
        }
    }

    /** Sets the role of the user, such as a host or an audience (default), before joining a channel in the interactive live streaming.

     This method can be used to switch the user role in the interactive live streaming after the user joins a channel.

     In the `LIVE_BROADCASTING` profile, when a user switches user roles after joining a channel, a successful \ref agora::rtc::IChannel::setClientRole "setClientRole" method call triggers the following callbacks:
     - The local client: \ref agora::rtc::IChannelEventHandler::onClientRoleChanged "onClientRoleChanged"
     - The remote client: \ref agora::rtc::IChannelEventHandler::onUserJoined "onUserJoined" or \ref agora::rtc::IChannelEventHandler::onUserOffline "onUserOffline" (BECOME_AUDIENCE)

     @note
     This method applies only to the `LIVE_BROADCASTING` profile.

     @param role Sets the role of the user. See #CLIENT_ROLE_TYPE.
     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_client_role(&self, role: CLIENT_ROLE_TYPE) -> i32 {
        unsafe {
            return agorartcnative::channel_setClientRole(self.native_channel, role as u32);
        }
    }

    /** Prioritizes a remote user's stream.

     Use this method with the \ref IRtcEngine::setRemoteSubscribeFallbackOption "setRemoteSubscribeFallbackOption" method.
     If the fallback function is enabled for a subscribed stream, the SDK ensures the high-priority user gets the best possible stream quality.

     @note The Agora SDK supports setting `serPriority` as high for one user only.

     @param  uid  The ID of the remote user.
     @param  userPriority Sets the priority of the remote user. See #PRIORITY_TYPE.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_remote_user_priority(&self, uid: u32, user_priority: PRIORITY_TYPE) -> i32 {
        unsafe {
            return agorartcnative::channel_setRemoteUserPriority(self.native_channel, uid, user_priority as u32);
        }
    }

    /** Sets the sound position and gain of a remote user.

     When the local user calls this method to set the sound position of a remote user, the sound difference between the left and right channels allows the
     local user to track the real-time position of the remote user, creating a real sense of space. This method applies to massively multiplayer online games,
     such as Battle Royale games.

     @note
     - For this method to work, enable stereo panning for remote users by calling the \ref agora::rtc::IRtcEngine::enableSoundPositionIndication "enableSoundPositionIndication" method before joining a channel.
     - This method requires hardware support. For the best sound positioning, we recommend using a stereo speaker.

     @param uid The ID of the remote user.
     @param pan The sound position of the remote user. The value ranges from -1.0 to 1.0:
     - 0.0: the remote sound comes from the front.
     - -1.0: the remote sound comes from the left.
     - 1.0: the remote sound comes from the right.
     @param gain Gain of the remote user. The value ranges from 0.0 to 100.0. The default value is 100.0 (the original gain of the remote user).
     The smaller the value, the less the gain.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_remote_voice_position(&self, uid: u32, pan: f64, gain: f64) -> i32 {
        unsafe {
            return agorartcnative::channel_setRemoteVoicePosition(self.native_channel, uid, pan, gain);
        }
    }

    /** Updates the display mode of the video view of a remote user.

     After initializing the video view of a remote user, you can call this method to update its rendering and mirror modes.
     This method affects only the video view that the local user sees.

     @note
     - Call this method after calling the \ref agora::rtc::IRtcEngine::setupRemoteVideo "setupRemoteVideo" method to initialize the remote video view.
     - During a call, you can call this method as many times as necessary to update the display mode of the video view of a remote user.

     @param userId The ID of the remote user.
     @param renderMode The rendering mode of the remote video view. See #RENDER_MODE_TYPE.
     @param mirrorMode
     - The mirror mode of the remote video view. See #VIDEO_MIRROR_MODE_TYPE.
     - **Note**: The SDK disables the mirror mode by default.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_remote_render_mode(&self, uid: u32, render_mode: RENDER_MODE_TYPE, mirror_mode: VIDEO_MIRROR_MODE_TYPE) -> i32 {
        unsafe {
            return agorartcnative::channel_setRemoteRenderMode(self.native_channel, uid, render_mode as u32, mirror_mode as u32);
        }
    }

    /** Sets whether to receive all remote audio streams by default.

     You can call this method either before or after joining a channel. If you call `setDefaultMuteAllRemoteAudioStreams (true)` after joining a channel, the remote audio streams of all subsequent users are not received.

     @note If you want to resume receiving the audio stream, call \ref agora::rtc::IChannel::muteRemoteAudioStream "muteRemoteAudioStream (false)",
     and specify the ID of the remote user whose audio stream you want to receive.
     To receive the audio streams of multiple remote users, call `muteRemoteAudioStream (false)` as many times.
     Calling `setDefaultMuteAllRemoteAudioStreams (false)` resumes receiving the audio streams of subsequent users only.

     @param mute Sets whether to receive/stop receiving all remote users' audio streams by default:
     - true:  Stops receiving all remote users' audio streams by default.
     - false: (Default) Receives all remote users' audio streams by default.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_default_mute_all_remote_audio_streams(&self, mute: bool) -> i32 {
        unsafe {
            let is_mute: i32 = if mute { 1 } else { 0 };
            return agorartcnative::channel_setDefaultMuteAllRemoteAudioStreams(self.native_channel, is_mute);
        }
    }

    /** Sets whether to receive all remote video streams by default.

     You can call this method either before or after joining a channel. If you
     call `setDefaultMuteAllRemoteVideoStreams (true)` after joining a channel,
     the remote video streams of all subsequent users are not received.

     @note If you want to resume receiving the video stream, call
     \ref agora::rtc::IChannel::muteRemoteVideoStream "muteRemoteVideoStream (false)",
     and specify the ID of the remote user whose video stream you want to receive.
     To receive the video streams of multiple remote users, call `muteRemoteVideoStream (false)`
     as many times. Calling `setDefaultMuteAllRemoteVideoStreams (false)` resumes
     receiving the video streams of subsequent users only.

     @param mute Sets whether to receive/stop receiving all remote users' video streams by default:
     - true: Stop receiving all remote users' video streams by default.
     - false: (Default) Receive all remote users' video streams by default.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_default_mute_all_remote_video_streams(&self, mute: bool) -> i32 {
        unsafe {
            let is_mute: i32 = if mute { 1 } else { 0 };
            return agorartcnative::channel_setDefaultMuteAllRemoteVideoStreams(self.native_channel, is_mute);
        }
    }

    /** Stops/Resumes receiving all remote users' audio streams.

     @param mute Sets whether to receive/stop receiving all remote users' audio streams.
     - true: Stops receiving all remote users' audio streams.
     - false: (Default) Receives all remote users' audio streams.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn mute_all_remote_audio_streams(&self, mute: bool) -> i32 {
        unsafe {
            let is_mute: i32 = if mute { 1 } else { 0 };
            return agorartcnative::channel_muteAllRemoteAudioStreams(self.native_channel, is_mute);
        }
    }

    /** Adjust the playback volume of the specified remote user.

     After joining a channel, call \ref agora::rtc::IRtcEngine::adjustPlaybackSignalVolume "adjustPlaybackSignalVolume" to adjust the playback volume of different remote users,
     or adjust multiple times for one remote user.

     @note
     - Call this method after joining a channel.
     - This method adjusts the playback volume, which is the mixed volume for the specified remote user.
     - This method can only adjust the playback volume of one specified remote user at a time. If you want to adjust the playback volume of several remote users,
     call the method multiple times, once for each remote user.

     @param userId The user ID, which should be the same as the `uid` of \ref agora::rtc::IChannel::joinChannel "joinChannel"
     @param volume The playback volume of the voice. The value ranges from 0 to 100:
     - 0: Mute.
     - 100: Original volume.

     @return
     - 0: Success.
	 - < 0: Failure.
     */
    pub fn adjust_user_playback_signal_volume(&self, uid: u32, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::channel_adjustUserPlaybackSignalVolume(self.native_channel, uid, volume);
        }
    }

    /** Stops/Resumes receiving a specified remote user's audio stream.

	 @note If you called the \ref agora::rtc::IChannel::muteAllRemoteAudioStreams "muteAllRemoteAudioStreams" method and set `mute` as `true` to stop
     receiving all remote users' audio streams, call the `muteAllRemoteAudioStreams` method and set `mute` as `false` before calling this method.
     The `muteAllRemoteAudioStreams` method sets all remote audio streams, while the `muteRemoteAudioStream` method sets a specified remote audio stream.

	 @param userId The user ID of the specified remote user sending the audio.
	 @param mute Sets whether to receive/stop receiving a specified remote user's audio stream:
	 - true: Stops receiving the specified remote user's audio stream.
	 - false: (Default) Receives the specified remote user's audio stream.

	 @return
	 - 0: Success.
	 - < 0: Failure.

	 */
    pub fn mute_remote_audio_stream(&self, uid: u32, mute: bool) -> i32 {
        unsafe {
            let is_mute: i32 = if mute { 1 } else { 0 };
            return agorartcnative::channel_muteRemoteAudioStream(self.native_channel, uid, is_mute);
        }
    }

    /** Stops/Resumes receiving all video stream from a specified remote user.

     @param  mute Sets whether to receive/stop receiving all remote users' video streams:
     - true: Stop receiving all remote users' video streams.
     - false: (Default) Receive all remote users' video streams.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn mute_all_remote_video_streams(&self, mute: bool) -> i32 {
        unsafe {
            let is_mute: i32 = if mute { 1 } else { 0 };
            return agorartcnative::channel_muteAllRemoteVideoStreams(self.native_channel, is_mute);
        }
    }

    /** Stops/Resumes receiving the video stream from a specified remote user.

     @note If you called the \ref agora::rtc::IChannel::muteAllRemoteVideoStreams "muteAllRemoteVideoStreams" method and
     set `mute` as `true` to stop receiving all remote video streams, call the `muteAllRemoteVideoStreams` method and
     set `mute` as `false` before calling this method.

     @param userId The user ID of the specified remote user.
     @param mute Sets whether to stop/resume receiving the video stream from a specified remote user:
     - true: Stop receiving the specified remote user's video stream.
     - false: (Default) Receive the specified remote user's video stream.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn mute_remote_video_stream(&self, uid: u32, mute: bool) -> i32 {
        unsafe {
            let is_mute: i32 = if mute { 1 } else { 0 };
            return agorartcnative::channel_muteRemoteVideoStream(self.native_channel, uid, is_mute);
        }
    }

    /** Sets the stream type of the remote video.

     Under limited network conditions, if the publisher has not disabled the dual-stream mode using
     \ref agora::rtc::IRtcEngine::enableDualStreamMode "enableDualStreamMode" (false),
     the receiver can choose to receive either the high-quality video stream (the high resolution, and high bitrate video stream) or
     the low-video stream (the low resolution, and low bitrate video stream).

     By default, users receive the high-quality video stream. Call this method if you want to switch to the low-video stream.
     This method allows the app to adjust the corresponding video stream type based on the size of the video window to
     reduce the bandwidth and resources.

     The aspect ratio of the low-video stream is the same as the high-quality video stream. Once the resolution of the high-quality video
     stream is set, the system automatically sets the resolution, frame rate, and bitrate of the low-video stream.

     The method result returns in the \ref agora::rtc::IRtcEngineEventHandler::onApiCallExecuted "onApiCallExecuted" callback.

     @param userId The ID of the remote user sending the video stream.
     @param streamType  Sets the video-stream type. See #REMOTE_VIDEO_STREAM_TYPE.
     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_remote_video_stream_type(&self, uid: u32, stream_type: REMOTE_VIDEO_STREAM_TYPE) -> i32 {
        unsafe {
            return agorartcnative::channel_setRemoteVideoStreamType(self.native_channel, uid, stream_type as u32);
        }
    }

    /** Sets the default stream type of remote videos.

     Under limited network conditions, if the publisher has not disabled the dual-stream mode using
     \ref agora::rtc::IRtcEngine::enableDualStreamMode "enableDualStreamMode" (false),
     the receiver can choose to receive either the high-quality video stream (the high resolution, and high bitrate video stream) or
     the low-video stream (the low resolution, and low bitrate video stream).

     By default, users receive the high-quality video stream. Call this method if you want to switch to the low-video stream.
     This method allows the app to adjust the corresponding video stream type based on the size of the video window to
     reduce the bandwidth and resources. The aspect ratio of the low-video stream is the same as the high-quality video stream.
      Once the resolution of the high-quality video
     stream is set, the system automatically sets the resolution, frame rate, and bitrate of the low-video stream.

     The method result returns in the \ref agora::rtc::IRtcEngineEventHandler::onApiCallExecuted "onApiCallExecuted" callback.

     @param streamType Sets the default video-stream type. See #REMOTE_VIDEO_STREAM_TYPE.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_remote_default_video_stream_type(&self, stream_type: REMOTE_VIDEO_STREAM_TYPE) -> i32 {
        unsafe {
            return agorartcnative::channel_setRemoteDefaultVideoStreamType(self.native_channel, stream_type as u32);
        }
    }

    /** Publishes the local stream to a specified CDN live RTMP address.  (CDN live only.)

     The SDK returns the result of this method call in the \ref IRtcEngineEventHandler::onStreamPublished "onStreamPublished" callback.

     The \ref agora::rtc::IChannel::addPublishStreamUrl "addPublishStreamUrl" method call triggers
     the \ref agora::rtc::IChannelEventHandler::onRtmpStreamingStateChanged "onRtmpStreamingStateChanged" callback on the local client
     to report the state of adding a local stream to the CDN.

     @note
     - Ensure that the user joins the channel before calling this method.
     - Ensure that you enable the RTMP Converter service before using this function. See Prerequisites in the advanced guide *Push Streams to CDN*.
     - This method adds only one stream RTMP URL address each time it is called.

     @param url The CDN streaming URL in the RTMP format. The maximum length of this parameter is 1024 bytes. The RTMP URL address must not contain special characters, such as Chinese language characters.
     @param  transcodingEnabled Sets whether transcoding is enabled/disabled:
     - true: Enable transcoding. To [transcode](https://docs.agora.io/en/Agora%20Platform/terms?platform=All%20Platforms#transcoding) the audio or video streams when publishing them to CDN live, often used for combining the audio and video streams of multiple hosts in CDN live. If you set this parameter as `true`, ensure that you call the \ref IChannel::setLiveTranscoding "setLiveTranscoding" method before this method.
     - false: Disable transcoding.

     @return
     - 0: Success.
     - < 0: Failure.
          - #ERR_INVALID_ARGUMENT (2): The RTMP URL address is NULL or has a string length of 0.
          - #ERR_NOT_INITIALIZED (7): You have not initialized `IChannel` when publishing the stream.
     */
    pub fn add_publish_stream_url(&self, url: &str, transcoding_enabled: bool) -> i32 {
        unsafe {
            let the_url: &CStr = &CString::new(url).expect("url new failed");
            let is_transcoding_enabled: i32 = if transcoding_enabled { 1 } else { 0 };
            return agorartcnative::channel_addPublishStreamUrl(self.native_channel, the_url.as_ptr(), is_transcoding_enabled);
        }
    }

    /** Removes an RTMP stream from the CDN.

     This method removes the RTMP URL address (added by the \ref IChannel::addPublishStreamUrl "addPublishStreamUrl" method) from a CDN live stream.

     The SDK returns the result of this method call in the \ref IRtcEngineEventHandler::onStreamUnpublished "onStreamUnpublished" callback.

     The \ref agora::rtc::IChannel::removePublishStreamUrl "removePublishStreamUrl" method call triggers
     the \ref agora::rtc::IChannelEventHandler::onRtmpStreamingStateChanged "onRtmpStreamingStateChanged" callback on the local client to report the state of removing an RTMP stream from the CDN.

     @note
     - This method removes only one RTMP URL address each time it is called.
     - The RTMP URL address must not contain special characters, such as Chinese language characters.

     @param url The RTMP URL address to be removed. The maximum length of this parameter is 1024 bytes.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn remove_publish_stream_url(&self, url: &str) -> i32 {
        unsafe {
            let the_url: &CStr = &CString::new(url).expect("url new failed");
            return agorartcnative::channel_removePublishStreamUrl(self.native_channel, the_url.as_ptr());
        }
    }

    /** Sets the video layout and audio settings for CDN live. (CDN live only.)

     The SDK triggers the \ref agora::rtc::IChannelEventHandler::onTranscodingUpdated "onTranscodingUpdated" callback when you
     call the `setLiveTranscoding` method to update the transcoding setting.

     @note
     - Ensure that you enable the RTMP Converter service before using this function. See Prerequisites in the advanced guide *Push Streams to CDN*..
     - If you call the `setLiveTranscoding` method to set the transcoding setting for the first time, the SDK does not trigger the `onTranscodingUpdated` callback.

     @param transcoding Sets the CDN live audio/video transcoding settings. See LiveTranscoding.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn set_live_transcoding(&self, transcoding: *const agorartcnative::LiveTranscoding) -> i32 {
        unsafe {
            return agorartcnative::channel_setLiveTranscoding(self.native_channel, transcoding);
        }
    }

    /** Adds a voice or video stream URL address to the interactive live streaming.

    The \ref IRtcEngineEventHandler::onStreamPublished "onStreamPublished" callback returns the inject status.
    If this method call is successful, the server pulls the voice or video stream and injects it into a live channel.
    This is applicable to scenarios where all audience members in the channel can watch a live show and interact with each other.

     The \ref agora::rtc::IChannel::addInjectStreamUrl "addInjectStreamUrl" method call triggers the following callbacks:
    - The local client:
      - \ref agora::rtc::IChannelEventHandler::onStreamInjectedStatus "onStreamInjectedStatus" , with the state of the injecting the online stream.
      - \ref agora::rtc::IChannelEventHandler::onUserJoined "onUserJoined" (uid: 666), if the method call is successful and the online media stream is injected into the channel.
    - The remote client:
      - \ref agora::rtc::IChannelEventHandler::onUserJoined "onUserJoined" (uid: 666), if the method call is successful and the online media stream is injected into the channel.

     @note
     - Ensure that you enable the RTMP Converter service before using this function. See Prerequisites in the advanced guide *Push Streams to CDN*.
     - This method applies to the Native SDK v2.4.1 and later.
     - This method applies to the `LIVE_BROADCASTING` profile only.
     - You can inject only one media stream into the channel at the same time.

     @param url The URL address to be added to the ongoing live streaming. Valid protocols are RTMP, HLS, and HTTP-FLV.
     - Supported audio codec type: AAC.
     - Supported video codec type: H264 (AVC).
     @param config The InjectStreamConfig object that contains the configuration of the added voice or video stream.

     @return
     - 0: Success.
     - < 0: Failure.
        - #ERR_INVALID_ARGUMENT (2): The injected URL does not exist. Call this method again to inject the stream and ensure that the URL is valid.
        - #ERR_NOT_READY (3): The user is not in the channel.
        - #ERR_NOT_SUPPORTED (4): The channel profile is not `LIVE_BROADCASTING`. Call the \ref IRtcEngine::setChannelProfile "setChannelProfile" method and set the channel profile to `LIVE_BROADCASTING` before calling this method.
        - #ERR_NOT_INITIALIZED (7): The SDK is not initialized. Ensure that the IChannel object is initialized before calling this method.
     */
    pub fn add_inject_stream_url(&self, url: &str, config: agorartcnative::InjectStreamConfig) -> i32 {
        unsafe {
            let the_url: &CStr = &CString::new(url).expect("url new failed");
            return agorartcnative::channel_addInjectStreamUrl(self.native_channel, the_url.as_ptr(), config);
        }
    }

    /** Removes the voice or video stream URL address from a live streaming.

     This method removes the URL address (added by the \ref IChannel::addInjectStreamUrl "addInjectStreamUrl" method) from the live streaming.

     @note If this method is called successfully, the SDK triggers the \ref IChannelEventHandler::onUserOffline "onUserOffline" callback and returns a stream uid of 666.

     @param url Pointer to the URL address of the added stream to be removed.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn remove_inject_stream_url(&self, url: &str) -> i32 {
        unsafe {
            let the_url: &CStr = &CString::new(url).expect("url new failed");
            return agorartcnative::channel_removeInjectStreamUrl(self.native_channel, the_url.as_ptr());
        }
    }

    /** Starts to relay media streams across channels.
     *
     * After a successful method call, the SDK triggers the
     * \ref agora::rtc::IChannelEventHandler::onChannelMediaRelayStateChanged
     *  "onChannelMediaRelayStateChanged" and
     * \ref agora::rtc::IChannelEventHandler::onChannelMediaRelayEvent
     * "onChannelMediaRelayEvent" callbacks, and these callbacks return the
     * state and events of the media stream relay.
     * - If the
     * \ref agora::rtc::IChannelEventHandler::onChannelMediaRelayStateChanged
     *  "onChannelMediaRelayStateChanged" callback returns
     * #RELAY_STATE_RUNNING (2) and #RELAY_OK (0), and the
     * \ref agora::rtc::IChannelEventHandler::onChannelMediaRelayEvent
     * "onChannelMediaRelayEvent" callback returns
     * #RELAY_EVENT_PACKET_SENT_TO_DEST_CHANNEL (4), the host starts
     * sending data to the destination channel.
     * - If the
     * \ref agora::rtc::IChannelEventHandler::onChannelMediaRelayStateChanged
     *  "onChannelMediaRelayStateChanged" callback returns
     * #RELAY_STATE_FAILURE (3), an exception occurs during the media stream
     * relay.
     *
     * @note
     * - Call this method after the \ref joinChannel() "joinChannel" method.
     * - This method takes effect only when you are a host in a
     * `LIVE_BROADCASTING` channel.
     * - After a successful method call, if you want to call this method
     * again, ensure that you call the
     * \ref stopChannelMediaRelay() "stopChannelMediaRelay" method to quit the
     * current relay.
     * - Contact sales-us@agora.io before implementing this function.
     * - We do not support string user accounts in this API.
     *
     * @param configuration The configuration of the media stream relay:
     * ChannelMediaRelayConfiguration.
     *
     * @return
     * - 0: Success.
     * - < 0: Failure.
     */
    pub fn start_channel_media_relay(&self, config: agorartcnative::ChannelMediaRelayConfiguration) -> i32 {
        unsafe {
            return agorartcnative::channel_startChannelMediaRelay(self.native_channel, config);
        }
    }

    /** Updates the channels for media stream relay.
     *
     * After a successful
     * \ref startChannelMediaRelay() "startChannelMediaRelay" method call, if
     * you want to relay the media stream to more channels, or leave the
     * current relay channel, you can call the
     * \ref updateChannelMediaRelay() "updateChannelMediaRelay" method.
     *
     * After a successful method call, the SDK triggers the
     * \ref agora::rtc::IChannelEventHandler::onChannelMediaRelayEvent
     *  "onChannelMediaRelayEvent" callback with the
     * #RELAY_EVENT_PACKET_UPDATE_DEST_CHANNEL (7) state code.
     *
     * @note
     * Call this method after the
     * \ref startChannelMediaRelay() "startChannelMediaRelay" method to update
     * the destination channel.
     *
     * @param configuration The media stream relay configuration:
     * ChannelMediaRelayConfiguration.
     *
     * @return
     * - 0: Success.
     * - < 0: Failure.
     */
    pub fn update_channel_media_relay(&self, config: agorartcnative::ChannelMediaRelayConfiguration) -> i32 {
        unsafe {
            return agorartcnative::channel_updateChannelMediaRelay(self.native_channel, config);
        }
    }

    /** Stops the media stream relay.
     *
     * Once the relay stops, the host quits all the destination
     * channels.
     *
     * After a successful method call, the SDK triggers the
     * \ref agora::rtc::IChannelEventHandler::onChannelMediaRelayStateChanged
     *  "onChannelMediaRelayStateChanged" callback. If the callback returns
     * #RELAY_STATE_IDLE (0) and #RELAY_OK (0), the host successfully
     * stops the relay.
     *
     * @note
     * If the method call fails, the SDK triggers the
     * \ref agora::rtc::IChannelEventHandler::onChannelMediaRelayStateChanged
     *  "onChannelMediaRelayStateChanged" callback with the
     * #RELAY_ERROR_SERVER_NO_RESPONSE (2) or
     * #RELAY_ERROR_SERVER_CONNECTION_LOST (8) state code. You can leave the
     * channel by calling the \ref leaveChannel() "leaveChannel" method, and
     * the media stream relay automatically stops.
     *
     * @return
     * - 0: Success.
     * - < 0: Failure.
     */
    pub fn stop_channel_media_relay(&self) -> i32 {
        unsafe {
            return agorartcnative::channel_stopChannelMediaRelay(self.native_channel);
        }
    }

    /** Creates a data stream.

     Each user can create up to five data streams during the lifecycle of the IChannel.

     @note Set both the `reliable` and `ordered` parameters to `true` or `false`. Do not set one as `true` and the other as `false`.

     @param streamId The ID of the created data stream.
     @param reliable Sets whether or not the recipients are guaranteed to receive the data stream from the sender within five seconds:
     - true: The recipients receive the data stream from the sender within five seconds. If the recipient does not receive the data stream within five seconds,
     an error is reported to the application.
     - false: There is no guarantee that the recipients receive the data stream within five seconds and no error message is reported for
     any delay or missing data stream.
     @param ordered Sets whether or not the recipients receive the data stream in the sent order:
     - true: The recipients receive the data stream in the sent order.
     - false: The recipients do not receive the data stream in the sent order.

     @return
     - Returns 0: Success.
     - < 0: Failure.
     */
    pub fn create_data_stream(&self, stream_id: *mut i32, reliable: bool, ordered: bool) -> i32 {
        unsafe {
            let is_reliable: i32 = if reliable { 1 } else { 0 };
            let is_ordered: i32 = if ordered { 1 } else { 0 };
            return agorartcnative::channel_createDataStream(self.native_channel, stream_id, is_reliable, is_ordered);
        }
    }

    /** Sends data stream messages to all users in a channel.

     The SDK has the following restrictions on this method:
     - Up to 30 packets can be sent per second in a channel with each packet having a maximum size of 1 kB.
     - Each client can send up to 6 kB of data per second.
     - Each user can have up to five data streams simultaneously.

     A successful \ref agora::rtc::IChannel::sendStreamMessage "sendStreamMessage" method call triggers
     the \ref agora::rtc::IChannelEventHandler::onStreamMessage "onStreamMessage" callback on the remote client, from which the remote user gets the stream message.

     A failed \ref agora::rtc::IChannel::sendStreamMessage "sendStreamMessage" method call triggers
     the \ref agora::rtc::IChannelEventHandler::onStreamMessageError "onStreamMessage" callback on the remote client.

     @note
     - This method applies only to the `COMMUNICATION` profile or to the hosts in the `LIVE_BROADCASTING` profile. If an audience in the `LIVE_BROADCASTING` profile calls this method, the audience may be switched to a host.
     - Ensure that you have created the data stream using \ref agora::rtc::IChannel::createDataStream "createDataStream" before calling this method.

     @param  streamId  The ID of the sent data stream, returned in the \ref IChannel::createDataStream "createDataStream" method.
     @param data The sent data.
     @param length The length of the sent data.

     @return
     - 0: Success.
     - < 0: Failure.
     */
    pub fn send_stream_message(&self, stream_id: i32, data: &str, length: i64) -> i32 {
        unsafe {
            let the_data: &CStr = &CString::new(data).expect("data new failed");
            return agorartcnative::channel_sendStreamMessage(self.native_channel, stream_id, the_data.as_ptr(), length);
        }
    }

    /** Gets the current connection state of the SDK.

     @return #CONNECTION_STATE_TYPE.
     */
    pub fn get_connection_state(&self) -> u32 {
        unsafe {
            return agorartcnative::channel_getConnectionState(self.native_channel);
        }
    }
}

impl VideoDeviceManager {
    pub fn get_device_count(&self) -> i32 {
        unsafe {
            return agorartcnative::getDeviceCount(self.native_video_manager);
        }
    }

    pub fn start_deivce_test(&self, hwnd: u64) -> i32 {
        unsafe {
            let h = hwnd as *mut ::std::os::raw::c_void;
            return agorartcnative::startDeviceTest(self.native_video_manager, h);
        }
    }

    pub fn stop_device_test(&self) -> i32 {
        unsafe {
            return agorartcnative::stopDeviceTest(self.native_video_manager);
        }
    }

    pub fn set_device(&self, device_id: &str) -> i32 {
        unsafe {
            let the_device_id: &CStr = &CString::new(device_id).expect("device_id new failed");
            return agorartcnative::setDevice(self.native_video_manager, the_device_id.as_ptr());
        }
    }

    pub fn get_device(&self, index: i32) -> (String, String, i32) {
        unsafe {
            let mut device_name: [::std::os::raw::c_char; 512] = [0; 512];
            let mut device_id: [::std::os::raw::c_char; 512] = [0; 512];
            let ret = agorartcnative::getDevice(self.native_video_manager, index, device_name.as_mut_ptr(), device_id.as_mut_ptr());

            let device_name_str = CStr::from_ptr(device_name.as_ptr()).to_string_lossy().into_owned();
            let device_id_str = CStr::from_ptr(device_id.as_ptr()).to_string_lossy().into_owned();

            (device_name_str, device_id_str, ret)
        }
    }

    pub fn get_current_device(&self) -> (String, i32) {
        unsafe {
            let mut device_id: [::std::os::raw::c_char; 512] = [0; 512];
            let ret = agorartcnative::getCurrentDevice(self.native_video_manager, device_id.as_mut_ptr());

            let device_id_str = CStr::from_ptr(device_id.as_ptr()).to_string_lossy().into_owned();

            (device_id_str, ret)
        }
    }
}

impl AudioPlaybackDeviceManager {
    pub fn get_count(&self) -> i32 {
        unsafe {
            return agorartcnative::audio_device_getCount(self.native_playback_manager, agorartcnative::DEVICE_TYPE_PLAYBACK_DEVICE);
        }
    }

    pub fn get_device(&self, index: i32) -> (String, String, i32) {
        unsafe {
            let mut device_name: [::std::os::raw::c_char; 512] = [0; 512];
            let mut device_id: [::std::os::raw::c_char; 512] = [0; 512];
            let ret = agorartcnative::audio_device_getDevice(self.native_playback_manager, agorartcnative::DEVICE_TYPE_PLAYBACK_DEVICE, index, device_name.as_mut_ptr(), device_id.as_mut_ptr());

            let device_name_str = CStr::from_ptr(device_name.as_ptr()).to_string_lossy().into_owned();
            let device_id_str = CStr::from_ptr(device_id.as_ptr()).to_string_lossy().into_owned();

            (device_name_str, device_id_str, ret)
        }
    }

    pub fn get_current_device(&self) -> (String, i32) {
        unsafe {
            let mut device_id: [::std::os::raw::c_char; 512] = [0; 512];
            let ret = agorartcnative::audio_device_getCurrentDevice(self.native_playback_manager, agorartcnative::DEVICE_TYPE_PLAYBACK_DEVICE, device_id.as_mut_ptr());

            let device_id_str = CStr::from_ptr(device_id.as_ptr()).to_string_lossy().into_owned();

            (device_id_str, ret)
        }
    }

    pub fn get_current_deivce_info(&self) -> (String, String, i32) {
        unsafe {
            let mut device_name: [::std::os::raw::c_char; 512] = [0; 512];
            let mut device_id: [::std::os::raw::c_char; 512] = [0; 512];
            let ret = agorartcnative::audio_device_getCurrentDeviceInfo(self.native_playback_manager, agorartcnative::DEVICE_TYPE_PLAYBACK_DEVICE, device_id.as_mut_ptr(), device_name.as_mut_ptr());

            let device_name_str = CStr::from_ptr(device_name.as_ptr()).to_string_lossy().into_owned();
            let device_id_str = CStr::from_ptr(device_id.as_ptr()).to_string_lossy().into_owned();

            (device_name_str, device_id_str, ret)
        }
    }

    pub fn set_device(&self, device_id: &str) -> i32 {
        unsafe {
            let the_device_id: &CStr = &CString::new(device_id).expect("device_id new failed");
            return agorartcnative::audio_device_setDevice(self.native_playback_manager, agorartcnative::DEVICE_TYPE_PLAYBACK_DEVICE, the_device_id.as_ptr());
        }
    }

    pub fn set_device_volume(&self, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::audio_device_setDeviceVolume(self.native_playback_manager, agorartcnative::DEVICE_TYPE_PLAYBACK_DEVICE, volume);
        }
    }

    pub fn get_device_volume(&self) -> (i32, i32) {
        unsafe {
            let mut volume: i32 = 0;
            let ret = agorartcnative::audio_device_getDeviceVolume(self.native_playback_manager, agorartcnative::DEVICE_TYPE_PLAYBACK_DEVICE, &mut volume);
            (volume, ret)
        }
    }

    pub fn set_device_mute(&self, mute: bool) -> i32 {
        unsafe {
            let is_mute: i32 = if mute { 1 } else { 0 };
            return agorartcnative::audio_device_setDeviceMute(self.native_playback_manager, agorartcnative::DEVICE_TYPE_PLAYBACK_DEVICE, is_mute);
        }
    }

    pub fn get_deivce_mute(&self) -> (bool, i32) {
        unsafe {
            let mut is_mute: i32 = 0;
            let ret = agorartcnative::audio_device_getDeviceMute(self.native_playback_manager, agorartcnative::DEVICE_TYPE_PLAYBACK_DEVICE, &mut is_mute);
            let mute: bool = if is_mute == 0 { false } else { true };
            (mute, ret)
        }
    }

    pub fn start_device_test(&self, test_audio_file_path: &str) -> i32 {
        unsafe {
            let the_test_audio_file_path: &CStr = &CString::new(test_audio_file_path).expect("test_audio_file_path new failed");
            return agorartcnative::audio_device_startDeviceTest(self.native_playback_manager, agorartcnative::DEVICE_TYPE_PLAYBACK_DEVICE, the_test_audio_file_path.as_ptr(), 0);
        }
    }

    pub fn stop_device_test(&self) -> i32 {
        unsafe {
            return agorartcnative::audio_device_stopDeviceTest(self.native_playback_manager, agorartcnative::DEVICE_TYPE_PLAYBACK_DEVICE);
        }
    }

    pub fn start_audio_device_loopback_test(&self, indication_interval: i32) -> i32 {
        unsafe {
            return agorartcnative::audio_device_startAudioDeviceLoopbackTest(self.native_playback_manager, agorartcnative::DEVICE_TYPE_PLAYBACK_DEVICE, indication_interval);
        }
    }

    pub fn stop_audio_device_loopback_test(&self) -> i32 {
        unsafe {
            return agorartcnative::audio_device_stopAudioDeviceLoopbackTest(self.native_playback_manager, agorartcnative::DEVICE_TYPE_PLAYBACK_DEVICE);
        }
    }
}

impl AudioRecordingDeviceManager {
    pub fn get_count(&self) -> i32 {
        unsafe {
            return agorartcnative::audio_device_getCount(self.native_recording_manager, agorartcnative::DEVICE_TYPE_RECORDING_DEVICE);
        }
    }

    pub fn get_device(&self, index: i32) -> (String, String, i32) {
        unsafe {
            let mut device_name: [::std::os::raw::c_char; 512] = [0; 512];
            let mut device_id: [::std::os::raw::c_char; 512] = [0; 512];
            let ret = agorartcnative::audio_device_getDevice(self.native_recording_manager, agorartcnative::DEVICE_TYPE_RECORDING_DEVICE, index, device_name.as_mut_ptr(), device_id.as_mut_ptr());

            let device_name_str = CStr::from_ptr(device_name.as_ptr()).to_string_lossy().into_owned();
            let device_id_str = CStr::from_ptr(device_id.as_ptr()).to_string_lossy().into_owned();

            (device_name_str, device_id_str, ret)
        }
    }

    pub fn get_current_device(&self) -> (String, i32) {
        unsafe {
            let mut device_id: [::std::os::raw::c_char; 512] = [0; 512];
            let ret = agorartcnative::audio_device_getCurrentDevice(self.native_recording_manager, agorartcnative::DEVICE_TYPE_RECORDING_DEVICE, device_id.as_mut_ptr());

            let device_id_str = CStr::from_ptr(device_id.as_ptr()).to_string_lossy().into_owned();

            (device_id_str, ret)
        }
    }

    pub fn get_current_deivce_info(&self) -> (String, String, i32) {
        unsafe {
            let mut device_name: [::std::os::raw::c_char; 512] = [0; 512];
            let mut device_id: [::std::os::raw::c_char; 512] = [0; 512];
            let ret = agorartcnative::audio_device_getCurrentDeviceInfo(self.native_recording_manager, agorartcnative::DEVICE_TYPE_RECORDING_DEVICE, device_id.as_mut_ptr(), device_name.as_mut_ptr());

            let device_name_str = CStr::from_ptr(device_name.as_ptr()).to_string_lossy().into_owned();
            let device_id_str = CStr::from_ptr(device_id.as_ptr()).to_string_lossy().into_owned();

            (device_name_str, device_id_str, ret)
        }
    }

    pub fn set_device(&self, device_id: &str) -> i32 {
        unsafe {
            let the_device_id: &CStr = &CString::new(device_id).expect("device_id new failed");
            return agorartcnative::audio_device_setDevice(self.native_recording_manager, agorartcnative::DEVICE_TYPE_RECORDING_DEVICE, the_device_id.as_ptr());
        }
    }

    pub fn set_device_volume(&self, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::audio_device_setDeviceVolume(self.native_recording_manager, agorartcnative::DEVICE_TYPE_RECORDING_DEVICE, volume);
        }
    }

    pub fn get_device_volume(&self) -> (i32, i32) {
        unsafe {
            let mut volume: i32 = 0;
            let ret = agorartcnative::audio_device_getDeviceVolume(self.native_recording_manager, agorartcnative::DEVICE_TYPE_RECORDING_DEVICE, &mut volume);
            (volume, ret)
        }
    }

    pub fn set_device_mute(&self, mute: bool) -> i32 {
        unsafe {
            let is_mute: i32 = if mute { 1 } else { 0 };
            return agorartcnative::audio_device_setDeviceMute(self.native_recording_manager, agorartcnative::DEVICE_TYPE_RECORDING_DEVICE, is_mute);
        }
    }

    pub fn get_deivce_mute(&self) -> (bool, i32) {
        unsafe {
            let mut is_mute: i32 = 0;
            let ret = agorartcnative::audio_device_getDeviceMute(self.native_recording_manager, agorartcnative::DEVICE_TYPE_RECORDING_DEVICE, &mut is_mute);
            let mute: bool = if is_mute == 0 { false } else { true };
            (mute, ret)
        }
    }

    pub fn start_device_test(&self, indication_interval: i32) -> i32 {
        unsafe {
            let the_test_audio_file_path: &CStr = &CString::new("").expect("test_audio_file_path new failed");
            return agorartcnative::audio_device_startDeviceTest(self.native_recording_manager, agorartcnative::DEVICE_TYPE_RECORDING_DEVICE, the_test_audio_file_path.as_ptr(), indication_interval);
        }
    }

    pub fn stop_device_test(&self) -> i32 {
        unsafe {
            return agorartcnative::audio_device_stopDeviceTest(self.native_recording_manager, agorartcnative::DEVICE_TYPE_RECORDING_DEVICE);
        }
    }

    pub fn start_audio_device_loopback_test(&self, indication_interval: i32) -> i32 {
        unsafe {
            return agorartcnative::audio_device_startAudioDeviceLoopbackTest(self.native_recording_manager, agorartcnative::DEVICE_TYPE_RECORDING_DEVICE, indication_interval);
        }
    }

    pub fn stop_audio_device_loopback_test(&self) -> i32 {
        unsafe {
            return agorartcnative::audio_device_stopAudioDeviceLoopbackTest(self.native_recording_manager, agorartcnative::DEVICE_TYPE_RECORDING_DEVICE);
        }
    }
}