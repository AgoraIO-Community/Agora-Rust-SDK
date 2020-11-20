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
    AREA_CODE_GLOBAL = 0xFFFFFFFF
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
    AUDIO_PROFILE_DEFAULT = 0, // use default settings
    /**
     1: A sample rate of 32 KHz, audio encoding, mono, and a bitrate of up to 18 Kbps.
     */
    AUDIO_PROFILE_SPEECH_STANDARD = 1, // 32Khz, 18Kbps, mono, speech
    /**
     2: A sample rate of 48 KHz, music encoding, mono, and a bitrate of up to 64 Kbps.
     */
    AUDIO_PROFILE_MUSIC_STANDARD = 2, // 48Khz, 48Kbps, mono, music
    /**
     3: A sample rate of 48 KHz, music encoding, stereo, and a bitrate of up to 80 Kbps.
     */
    AUDIO_PROFILE_MUSIC_STANDARD_STEREO = 3, // 48Khz, 56Kbps, stereo, music
    /**
     4: A sample rate of 48 KHz, music encoding, mono, and a bitrate of up to 96 Kbps.
     */
    AUDIO_PROFILE_MUSIC_HIGH_QUALITY = 4, // 48Khz, 128Kbps, mono, music
    /**
     5: A sample rate of 48 KHz, music encoding, stereo, and a bitrate of up to 128 Kbps.
     */
    AUDIO_PROFILE_MUSIC_HIGH_QUALITY_STEREO = 5, // 48Khz, 192Kbps, stereo, music
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
    VIDEO_MIRROR_MODE_AUTO = 0, //determined by SDK
    /** 1: Enable mirror mode. */
    VIDEO_MIRROR_MODE_ENABLED = 1, //enabled mirror
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
    AUDIO_REVERB_DRY_LEVEL = 0, // (dB, [-20,10]), the level of the dry signal
    /** 1: The level of the early reflection signal (wet signal) (dB). The value is between -20 and 10. */
    AUDIO_REVERB_WET_LEVEL = 1, // (dB, [-20,10]), the level of the early reflection signal (wet signal)
    /** 2: The room size of the reflection. The value is between 0 and 100. */
    AUDIO_REVERB_ROOM_SIZE = 2, // ([0,100]), the room size of the reflection
    /** 3: The length of the initial delay of the wet signal (ms). The value is between 0 and 200. */
    AUDIO_REVERB_WET_DELAY = 3, // (ms, [0,200]), the length of the initial delay of the wet signal in ms
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
    VOICE_CHANGER_OFF = 0x00000000, //Turn off the voice changer
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
    VOICE_BEAUTY_VIGOROUS = 0x00100001, //7,
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
    GENERAL_BEAUTY_VOICE_FEMALE_VITALITY = 0x00200003

}

/** Local voice reverberation presets. */
pub enum AUDIO_REVERB_PRESET {
    /**
     * Turn off local voice reverberation, that is, to use the original voice.
     */
    AUDIO_REVERB_OFF = 0x00000000, // Turn off audio reverb
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
    AUDIO_VIRTUAL_STEREO = 0x00200001
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

unsafe impl std::marker::Sync for AgoraRtcChannel { }

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

// static mut agoraRtcEngine: AgoraRtcEngine = AgoraRtcEngine {
//     native_engine: std::ptr::null_mut(),
// };

lazy_static!  {
    #[derive(Debug)]
    pub static ref Agora_Rtc_Engine: AgoraRtcEngine = AgoraRtcEngine {
        native_engine: unsafe {agorartcnative::createRtcBridge()},
    };
}

unsafe impl std::marker::Sync for AgoraRtcEngine{

}

impl AgoraRtcEngine {
    // pub fn create_rtc_bridge() -> &'static AgoraRtcEngine {
    //     unsafe {
    //         if agoraRtcEngine.native_engine.is_null() {
    //             agoraRtcEngine.native_engine = agorartcnative::createRtcBridge();
    //         }
    //         return &agoraRtcEngine;
    //     }
    // }

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

    pub fn release(&self, sync: bool) {
        unsafe {
            let is_sync: i32 = if sync { 1 } else { 0 };
            agorartcnative::release(self.native_engine, is_sync);
        }
    }

    pub fn set_channel_profile(&self, profile: CHANNEL_PROFILE_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setChannelProfile(self.native_engine, profile as u32);
        }
    }

    pub fn set_client_role(&self, role: CLIENT_ROLE_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setClientRole(self.native_engine, role as u32);
        }
    }

    pub fn join_channel(&self, token: &str, channel_id: &str, info: &str, uid: u32) -> i32 {
        unsafe {
            let the_token: &CStr = &CString::new(token).expect("token new failed");
            let the_channel_id: &CStr = &CString::new(channel_id).expect("channel_id new failed");
            let the_info: &CStr = &CString::new(info).expect("info new failed");
            return agorartcnative::joinChannel(self.native_engine, the_token.as_ptr(), the_channel_id.as_ptr(), the_info.as_ptr(), uid);
        }
    }

    pub fn switch_channel(&self, token: &str, channel_id: &str) -> i32 {
        unsafe {
            let the_token: &CStr = &CString::new(token).expect("token new failed");
            let the_channel_id: &CStr = &CString::new(channel_id).expect("channel_id new failed");
            return agorartcnative::switchChannel(self.native_engine, the_token.as_ptr(), the_channel_id.as_ptr());
        }
    }

    pub fn leave_channel(&self) -> i32 {
        unsafe {
            return agorartcnative::leaveChannel(self.native_engine);
        }
    }

    pub fn renew_token(&self, token: &str) -> i32 {
        unsafe {
            let the_token: &CStr = &CString::new(token).expect("token new failed");
            return agorartcnative::renewToken(self.native_engine, the_token.as_ptr());
        }
    }

    pub fn register_local_user_account(&self, app_id: &str, user_account: &str) -> i32 {
        unsafe {
            let the_app_id = &CString::new(app_id).expect("app_id new failed");
            let the_user_account = &CString::new(user_account).expect("user_account new failed");
            return agorartcnative::registerLocalUserAccount(self.native_engine, the_app_id.as_ptr(), the_user_account.as_ptr());
        }
    }

    pub fn join_channel_with_user_account(&self, token: &str, channel_id: &str, user_account: &str) -> i32 {
        unsafe {
            let the_token: &CStr = &CString::new(token).expect("token new failed");
            let the_channel_id: &CStr = &CString::new(channel_id).expect("channel_id new failed");
            let the_user_account = &CString::new(user_account).expect("user_account new failed");
            return agorartcnative::joinChannelWithUserAccount(self.native_engine, the_token.as_ptr(), the_channel_id.as_ptr(), the_user_account.as_ptr());

        }
    }

    pub fn get_user_info_by_user_account(&self, user_account: &str, user_info: *mut agorartcnative::UserInfo) -> i32 {
        unsafe {
            let the_user_account = &CString::new(user_account).expect("user_account new failed");
            return agorartcnative::getUserInfoByUserAccount(self.native_engine, the_user_account.as_ptr(), user_info);
        }
    }

    pub fn get_user_info_by_uid(&self, uid: u32, user_info: *mut agorartcnative::UserInfo) -> i32 {
        unsafe {
            return agorartcnative::getUserInfoByUid(self.native_engine, uid, user_info);
        }
    }

    pub fn start_echo_test(&self) -> i32 {
        unsafe {
            return agorartcnative::startEchoTest(self.native_engine);
        }
    }

    pub fn start_echo_test2(&self, interval_in_seconds: i32) -> i32 {
        unsafe {
            return agorartcnative::startEchoTest2(self.native_engine, interval_in_seconds);
        }
    }

    pub fn stop_echo_test(&self) -> i32 {
        unsafe {
            return agorartcnative::stopEchoTest(self.native_engine);
        }
    }

    pub fn enable_video(&self) -> i32 {
        unsafe {
            return agorartcnative::enableVideo(self.native_engine);
        }
    }

    pub fn disable_video(&self) -> i32 {
        unsafe {
            return agorartcnative::disableVideo(self.native_engine);
        }
    }

    pub fn set_video_profile(&self, profile: VIDEO_PROFILE_TYPE, swap_width_and_height: bool) -> i32 {
        unsafe {
            let swap: i32 = if swap_width_and_height { 1 } else { 0 };
            return agorartcnative::setVideoProfile(self.native_engine, profile as u32, swap);
        }
    }

    pub fn set_video_encoder_configuation(&self, config: agorartcnative::VideoEncoderConfiguration) -> i32 {
        unsafe {
            return agorartcnative::setVideoEncoderConfiguration(self.native_engine, config);
        }
    }

    pub fn set_camera_capture_configuration(&self, config: agorartcnative::CameraCapturerConfiguration) -> i32 {
        unsafe {
            return agorartcnative::setCameraCapturerConfiguration(self.native_engine, config);
        }
    }

    pub fn setup_local_video(&self, canvas: agorartcnative::VideoCanvas) -> i32 {
        unsafe {
            return agorartcnative::setupLocalVideo(self.native_engine, canvas);
        }
    }

    pub fn setup_remote_video(&self, canvas: agorartcnative::VideoCanvas) -> i32 {
        unsafe {
            return agorartcnative::setupRemoteVideo(self.native_engine, canvas);
        }
    }

    pub fn start_preview(&self) -> i32 {
        unsafe {
            return agorartcnative::startPreview(self.native_engine);
        }
    }

    pub fn set_remote_user_priority(&self, uid: u32, user_priority: PRIORITY_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setRemoteUserPriority(self.native_engine, uid, user_priority as u32);
        }
    }

    pub fn stop_preview(&self) -> i32 {
        unsafe {
            return agorartcnative::stopPreview(self.native_engine);
        }
    }

    pub fn enable_audio(&self) -> i32 {
        unsafe {
            return agorartcnative::enableAudio(self.native_engine);
        }
    }

    pub fn enable_local_audio(&self, enabled: bool) -> i32 {
        unsafe {
            let e: i32 = if enabled { 1 } else { 0 };
            return agorartcnative::enableLocalAudio(self.native_engine, e);
        }
    }

    pub fn disable_audio(&self) -> i32 {
        unsafe {
            return agorartcnative::disableAudio(self.native_engine);
        }
    }

    pub fn set_audio_profile(&self, profile: AUDIO_PROFILE_TYPE, scenario: AUDIO_SCENARIO_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setAudioProfile(self.native_engine, profile as u32, scenario as u32);
        }
    }

    pub fn mute_local_audio_stream(&self, mute: bool) -> i32 {
        unsafe {
            let m: i32 = if mute { 1 } else { 0 };
            return agorartcnative::muteLocalAudioStream(self.native_engine, m);
        }
    }

    pub fn mute_all_remote_audio_streams(&self, mute: bool) -> i32 {
        unsafe {
            let m: i32 = if mute { 1 } else { 0 };
            return agorartcnative::muteAllRemoteAudioStreams(self.native_engine, m);
        }
    }

    pub fn set_default_mute_all_remote_video_streams(&self, mute: bool) -> i32 {
        unsafe {
            let m: i32 = if mute { 1 } else { 0 };
            return agorartcnative::setDefaultMuteAllRemoteVideoStreams(self.native_engine, m);
        }
    }

    pub fn adjust_user_playback_signal_volume(&self, uid: u32, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::adjustUserPlaybackSignalVolume(self.native_engine, uid, volume);
        }
    }

    pub fn mute_remote_audio_stream(&self, uid: u32, mute: bool) -> i32 {
        unsafe {
            let m: i32 = if mute { 1 } else { 0 };
            return agorartcnative::muteRemoteAudioStream(self.native_engine, uid, m);
        }
    }

    pub fn mute_local_video_stream(&self, mute: bool) -> i32 {
        unsafe {
            let m: i32 = if mute { 1 } else { 0 };
            return agorartcnative::muteLocalVideoStream(self.native_engine, m);
        }
    }

    pub fn enable_local_video(&self, enabled: bool) -> i32 {
        unsafe {
            let e: i32 = if enabled { 1 } else { 0 };
            return agorartcnative::enableLocalVideo(self.native_engine, e);
        }
    }

    pub fn mute_all_remote_video_streams(&self, mute: bool) -> i32 {
        unsafe {
            let m: i32 = if mute { 1 } else { 0 };
            return agorartcnative::muteAllRemoteVideoStreams(self.native_engine, m);
        }
    }

    pub fn set_default_mute_all_remote_audio_streams(&self, mute: bool) -> i32 {
        unsafe {
            let m: i32 = if mute { 1 } else { 0 };
            return agorartcnative::setDefaultMuteAllRemoteAudioStreams(self.native_engine, m);
        }
    }

    pub fn mute_remote_video_stream(&self, uid: u32, mute: bool) -> i32 {
        unsafe {
            let m: i32 = if mute { 1 } else { 0 };
            return agorartcnative::muteRemoteVideoStream(self.native_engine, uid, m);
        }
    }

    pub fn set_remote_video_stream_type(&self, uid: u32, stream_type: REMOTE_VIDEO_STREAM_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setRemoteVideoStreamType(self.native_engine, uid, stream_type as u32);
        }
    }

    pub fn set_remote_default_video_stream_type(&self, stream_type: REMOTE_VIDEO_STREAM_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setRemoteDefaultVideoStreamType(self.native_engine, stream_type as u32);
        }
    }

    pub fn enable_audio_volume_indication(&self, interval: i32, smooth: i32, report_vad: bool) -> i32 {
        unsafe {
            let r: i32 = if report_vad { 1 } else { 0 };
            return agorartcnative::enableAudioVolumeIndication(self.native_engine, interval, smooth, r);
        }
    }

    pub fn start_audio_recoding(&self, file_path: &str, quality: AUDIO_RECORDING_QUALITY_TYPE) -> i32 {
        unsafe {
            let the_file_path: &CStr = &CString::new(file_path).expect("file_path new failed");
            return agorartcnative::startAudioRecording(self.native_engine, the_file_path.as_ptr(), quality as u32);
        }
    }

    pub fn start_audio_recoding2(&self, file_path: &str, sample_rate: i32, quality: AUDIO_RECORDING_QUALITY_TYPE) -> i32 {
        unsafe {
            let the_file_path: &CStr = &CString::new(file_path).expect("file_path new failed");
            return agorartcnative::startAudioRecording2(self.native_engine, the_file_path.as_ptr(), sample_rate, quality as u32);
        }
    }
    pub fn stop_audio_recoding(&self) -> i32 {
        unsafe {
            return agorartcnative::stopAudioRecording(self.native_engine);
        }
    }

    pub fn set_remote_voice_position(&self, uid: u32, pan: f64, gain: f64) -> i32 {
        unsafe {
            return agorartcnative::setRemoteVoicePosition(self.native_engine, uid, pan, gain);
        }
    }

    pub fn set_log_file(&self, file: &str) -> i32 {
        unsafe {
            let the_file: &CStr = &CString::new(file).expect("file new failed");
            return agorartcnative::setLogFile(self.native_engine, the_file.as_ptr());
        }
    }

    pub fn set_log_filter(&self, filter: u32) -> i32 {
        unsafe {
            return agorartcnative::setLogFilter(self.native_engine, filter);
        }
    }

    pub fn set_log_file_size(&self, file_size_in_kbytes: u32) -> i32 {
        unsafe {
            return agorartcnative::setLogFileSize(self.native_engine, file_size_in_kbytes);
        }
    }

    pub fn set_local_render_mode(&self, render_mode: RENDER_MODE_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setLocalRenderMode(self.native_engine, render_mode as u32);
        }
    }

    pub fn set_local_render_mode2(&self, render_mode: RENDER_MODE_TYPE, mirror_mode: VIDEO_MIRROR_MODE_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setLocalRenderMode2(self.native_engine, render_mode as u32, mirror_mode as u32);
        }
    }

    pub fn set_remote_render_mode(&self, uid: u32, render_mode: RENDER_MODE_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setRemoteRenderMode(self.native_engine, uid, render_mode as u32);
        }
    }

    pub fn set_remote_render_mode2(&self, uid: u32, render_mode: RENDER_MODE_TYPE, mirror_mode: VIDEO_MIRROR_MODE_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setRemoteRenderMode2(self.native_engine, uid, render_mode as u32, mirror_mode as u32);
        }
    }

    pub fn set_local_video_mirror_mode(&self, mirror_mode: VIDEO_MIRROR_MODE_TYPE) -> i32 {
        unsafe {
            return agorartcnative::setLocalVideoMirrorMode(self.native_engine, mirror_mode as u32);
        }
    }

    pub fn enable_dual_stream_mode(&self, enabled: bool) -> i32 {
        unsafe {
            let e: i32 = if enabled { 1 } else { 0 };
            return agorartcnative::enableDualStreamMode(self.native_engine, e);
        }
    }

    pub fn adjust_recoding_signal_volume(&self, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::adjustRecordingSignalVolume(self.native_engine, volume);
        }
    }

    pub fn adjust_playback_signal_volume(&self, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::adjustPlaybackSignalVolume(self.native_engine, volume);
        }
    }

    pub fn enable_web_sdk_interoperability(&self, enabled: bool) -> i32 {
        unsafe {
            let e: i32 = if enabled { 1 } else { 0 };
            return agorartcnative::enableWebSdkInteroperability(self.native_engine, e);
        }
    }

    pub fn set_video_quality_parameters(&self, prefer_frame_rate_over_image_quality: bool) -> i32 {
        unsafe {
            let prefer: i32 = if prefer_frame_rate_over_image_quality { 1 } else { 0 };
            return agorartcnative::setVideoQualityParameters(self.native_engine, prefer);
        }
    }

    pub fn set_local_publish_fallback_option(&self, option: STREAM_FALLBACK_OPTIONS) -> i32 {
        unsafe {
            return agorartcnative::setLocalPublishFallbackOption(self.native_engine, option as u32);
        }
    }

    pub fn set_remote_subscribe_fallback_option(&self, option: STREAM_FALLBACK_OPTIONS) -> i32 {
        unsafe {
            return agorartcnative::setRemoteSubscribeFallbackOption(self.native_engine, option as u32);
        }
    }

    pub fn get_call_id(&self) -> String {
        unsafe {
            let s = agorartcnative::getCallId(self.native_engine);
            let r = CStr::from_ptr(s).to_string_lossy().into_owned();
            return r;
        }
    }

    pub fn rate(&self, call_id: &str, rating: i32, description: &str) -> i32 {
        unsafe {
            let the_call_id: &CStr =&CString::new(call_id).expect("call_id new failed");
            let the_description: &CStr =&CString::new(description).expect("description new failed");
            return agorartcnative::rate(self.native_engine, the_call_id.as_ptr(), rating, the_description.as_ptr());
        }
    }

    pub fn complain(&self, call_id: &str, description: &str) -> i32 {
        unsafe {
            let the_call_id: &CStr =&CString::new(call_id).expect("call_id new failed");
            let the_description: &CStr =&CString::new(description).expect("description new failed");
            return agorartcnative::complain(self.native_engine, the_call_id.as_ptr(), the_description.as_ptr());
        }
    }

    pub fn get_version(&self) -> String {
        unsafe {
            let s = agorartcnative::getVersion(self.native_engine);
            let r = CStr::from_ptr(s).to_string_lossy().into_owned();
            return r;
        }
    }

    pub fn enable_lastmile_test(&self) -> i32 {
        unsafe {
            return agorartcnative::enableLastmileTest(self.native_engine);
        }
    }

    pub fn disable_lastmile_test(&self) -> i32 {
        unsafe {
            return agorartcnative::disableLastmileTest(self.native_engine);
        }
    }

    pub fn start_lastmile_probe_test(&self, config: agorartcnative::LastmileProbeConfig) -> i32 {
        unsafe {
            return agorartcnative::startLastmileProbeTest(self.native_engine, config);
        }
    }

    pub fn stop_lastmile_probe_test(&self) -> i32 {
        unsafe {
            return agorartcnative::stopLastmileProbeTest(self.native_engine);
        }
    }

    pub fn get_error_description(&self, code: i32) -> String {
        unsafe {
            let s = agorartcnative::getErrorDescription(self.native_engine, code);
            let r = CStr::from_ptr(s).to_string_lossy().into_owned();
            return r;
        }
    }

    pub fn set_encryption_secret(&self, secret: &str) -> i32 {
        unsafe {
            let the_secret: &CStr = &CString::new(secret).expect("secret new failed");
            return agorartcnative::setEncryptionSecret(self.native_engine, the_secret.as_ptr());
        }
    }

    pub fn set_encryption_mode(&self, encryption_mode: &str) -> i32 {
        unsafe {
            let the_encryption_mode: &CStr = &CString::new(encryption_mode).expect("encryption_mode new failed");
            return agorartcnative::setEncryptionMode(self.native_engine, the_encryption_mode.as_ptr());
        }
    }

    pub fn create_data_stream(&self, stream_id: *mut i32, reliable: bool, ordered: bool) -> i32 {
        unsafe {
            let is_reliable: i32 = if reliable { 1 } else { 0 };
            let is_ordered: i32 = if ordered { 1 } else { 0 };
            return agorartcnative::createDataStream(self.native_engine, stream_id, is_reliable, is_ordered);
        }
    }

    pub fn send_stream_message(&self, stream_id: i32, data: &str, length: i64) -> i32 {
        unsafe {
            let the_data: &CStr = &CString::new(data).expect("data new failed");
            return agorartcnative::sendStreamMessage(self.native_engine, stream_id, the_data.as_ptr(), length);
        }
    }

    pub fn add_publish_stream_url(&self, url: &str, transcoding_enabled: bool) -> i32 {
        unsafe {
            let the_url: &CStr = &CString::new(url).expect("url new failed");
            let is_transcoding_enabled: i32 = if transcoding_enabled { 1 } else { 0 };
            return agorartcnative::addPublishStreamUrl(self.native_engine, the_url.as_ptr(), is_transcoding_enabled);
        }
    }

    pub fn remove_publish_stream_url(&self, url: &str) -> i32 {
        unsafe {
            let the_url: &CStr = &CString::new(url).expect("url new failed");
            return agorartcnative::removePublishStreamUrl(self.native_engine, the_url.as_ptr());
        }
    }

    pub fn set_live_transcoding(&self, transcoding: *const agorartcnative::LiveTranscoding) -> i32 {
        unsafe {
            return agorartcnative::setLiveTranscoding(self.native_engine, transcoding);
        }
    }

    pub fn add_video_watermark(&self, watermark: agorartcnative::RtcImage) -> i32 {
        unsafe {
            return agorartcnative::addVideoWatermark(self.native_engine, watermark);
        }
    }

    pub fn add_video_watermark2(&self, watermark_url: &str, options: agorartcnative::WatermarkOptions) -> i32 {
        unsafe {
            let the_url: &CStr = &CString::new(watermark_url).expect("watermark_url new failed");
            return agorartcnative::addVideoWatermark2(self.native_engine, the_url.as_ptr(), options);
        }
    }

    pub fn clear_video_watermarks(&self) -> i32 {
        unsafe {
            return agorartcnative::clearVideoWatermarks(self.native_engine);
        }
    }

    pub fn set_beauty_effect_options(&self, enabled: bool, options: agorartcnative::BeautyOptions) -> i32 {
        unsafe {
            let is_enabled: i32 = if enabled { 1 } else { 0 };
            return agorartcnative::setBeautyEffectOptions(self.native_engine, is_enabled, options);
        }
    }

    pub fn add_inject_stream_url(&self, url: &str, config: agorartcnative::InjectStreamConfig) -> i32 {
        unsafe {
            let the_url: &CStr = &CString::new(url).expect("url new failed");
            return agorartcnative::addInjectStreamUrl(self.native_engine, the_url.as_ptr(), config);
        }
    }

    pub fn start_channel_media_relay(&self, config: agorartcnative::ChannelMediaRelayConfiguration) -> i32 {
        unsafe {
            return agorartcnative::startChannelMediaRelay(self.native_engine, config);
        }
    }

    pub fn update_channel_media_relay(&self, config: agorartcnative::ChannelMediaRelayConfiguration) -> i32 {
        unsafe {
            return agorartcnative::updateChannelMediaRelay(self.native_engine, config);
        }
    }

    pub fn stop_channel_media_relay(&self) -> i32 {
        unsafe {
            return agorartcnative::stopChannelMediaRelay(self.native_engine);
        }
    }

    pub fn remove_inject_stream_url(&self, url: &str) -> i32 {
        unsafe {
            let the_url: &CStr = &CString::new(url).expect("url new failed");
            return agorartcnative::removeInjectStreamUrl(self.native_engine, the_url.as_ptr());
        }
    }

    pub fn get_connection_state(&self) -> u32 {
        unsafe {
            return agorartcnative::getConnectionState(self.native_engine);
        }
    }

    pub fn set_parameters(&self, parameters: &str) -> i32 {
        unsafe {
            let param: &CStr = &CString::new(parameters).expect("parameters new failed");
            return agorartcnative::setParameters(self.native_engine, param.as_ptr());
        }
    }

    pub fn set_playback_device_volume(&self, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::setPlaybackDeviceVolume(self.native_engine, volume);
        }
    }

    pub fn start_audio_mixing(&self, file_path: &str, loopback: bool, replace: bool, cycle: i32) -> i32 {
        unsafe {
            let the_file_path: &CStr = &CString::new(file_path).expect("file_path new failed");
            let is_loopback: i32 = if loopback { 1 } else { 0 };
            let is_replace: i32 = if replace { 1 } else { 0 };
            return agorartcnative::startAudioMixing(self.native_engine, the_file_path.as_ptr(), is_loopback, is_replace, cycle);
        }
    }

    pub fn stop_audio_mixing(&self) -> i32 {
        unsafe {
            return agorartcnative::stopAudioMixing(self.native_engine);
        }
    }

    pub fn pause_audio_mixing(&self) -> i32 {
        unsafe {
            return agorartcnative::pauseAudioMixing(self.native_engine);
        }
    }

    pub fn resume_audio_mixing(&self) -> i32 {
        unsafe {
            return agorartcnative::resumeAudioMixing(self.native_engine);
        }
    }

    pub fn set_high_quality_audio_parameters(&self, fullband: bool, stereo: bool, full_bitrate: bool) -> i32 {
        unsafe {
            let is_full_band: i32 = if fullband { 1 } else { 0 };
            let is_stereo: i32 = if stereo { 1 } else { 0 };
            let is_full_bitrate: i32 = if full_bitrate { 1 } else { 0 };
            return agorartcnative::setHighQualityAudioParameters(self.native_engine, is_full_band, is_stereo, is_full_bitrate);
        }
    }

    pub fn adjust_audio_mixing_volume(&self, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::adjustAudioMixingVolume(self.native_engine, volume);
        }
    }

    pub fn adjust_audio_mixing_playout_volume(&self, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::adjustAudioMixingPlayoutVolume(self.native_engine, volume);
        }
    }

    pub fn get_audio_mixing_playout_volume(&self) -> i32 {
        unsafe {
            return agorartcnative::getAudioMixingPlayoutVolume(self.native_engine);
        }
    }

    pub fn adjust_audio_mixing_publish_volume(&self, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::adjustAudioMixingPublishVolume(self.native_engine, volume);
        }
    }
    
    pub fn get_audio_mixing_publish_volume(&self) -> i32 {
        unsafe {
            return agorartcnative::getAudioMixingPublishVolume(self.native_engine);
        }
    }

    pub fn get_audio_mixing_duration(&self) -> i32 {
        unsafe {
            return agorartcnative::getAudioMixingDuration(self.native_engine);
        }
    }

    pub fn get_audio_mixing_current_position(&self) -> i32 {
        unsafe {
            return agorartcnative::getAudioMixingCurrentPosition(self.native_engine);
        }
    }

    pub fn set_audion_mixing_position(&self, pos: i32) -> i32 {
        unsafe {
            return agorartcnative::setAudioMixingPosition(self.native_engine, pos);
        }
    }

    pub fn set_audio_mixing_pitch(&self, pitch: i32) -> i32 {
        unsafe {
            return agorartcnative::setAudioMixingPitch(self.native_engine, pitch);
        }
    }

    pub fn get_effects_volume(&self) -> i32 {
        unsafe {
            return agorartcnative::getEffectsVolume(self.native_engine);
        }
    }

    pub fn set_effect_volume(&self, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::setEffectsVolume(self.native_engine, volume);
        }
    }

    pub fn set_volume_of_effect(&self, sound_id: i32, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::setVolumeOfEffect(self.native_engine, sound_id, volume);
        }
    }

    pub fn play_effect(&self, sound_id: i32, file_path: &str, loop_count: i32, pitch: f64, pan: f64, gain: i32, publish: bool) -> i32 {
        unsafe {
            let the_file_path: &CStr = &CString::new(file_path).expect("file_path new failed");
            let is_publish: i32 = if publish { 1 } else { 0 };
            return agorartcnative::playEffect(self.native_engine, sound_id, the_file_path.as_ptr(), loop_count, pitch, pan, gain, is_publish);
        }
    }

    pub fn stop_effect(&self, sound_id: i32) -> i32 {
        unsafe {
            return agorartcnative::stopEffect(self.native_engine, sound_id);
        }
    }

    pub fn stop_all_effect(&self) -> i32 {
        unsafe {
            return agorartcnative::stopAllEffects(self.native_engine);
        }
    }

    pub fn preload_effect(&self, sound_id: i32, file_path: &str) -> i32 {
        unsafe {
            let the_file_path: &CStr = &CString::new(file_path).expect("file_path new failed");
            return agorartcnative::preloadEffect(self.native_engine, sound_id, the_file_path.as_ptr());
        }
    }

    pub fn unload_effect(&self, sound_id: i32) -> i32 {
        unsafe {
            return agorartcnative::unloadEffect(self.native_engine, sound_id);
        }
    }

    pub fn pause_effect(&self, sound_id: i32) -> i32 {
        unsafe {
            return agorartcnative::pauseEffect(self.native_engine, sound_id);
        }
    }

    pub fn pause_all_effect(&self) -> i32 {
        unsafe {
            return agorartcnative::pauseAllEffects(self.native_engine);
        }
    }

    pub fn resume_effect(&self, sound_id: i32) -> i32 {
        unsafe {
            return agorartcnative::resumeEffect(self.native_engine, sound_id);
        }
    }

    pub fn resume_all_effects(&self) -> i32 {
        unsafe {
            return agorartcnative::resumeAllEffects(self.native_engine);
        }
    }

    pub fn enable_sound_position_indication(&self, enabled: bool) -> i32 {
        unsafe {
            let is_enabled: i32 = if enabled { 1 } else { 0 };
            return agorartcnative::enableSoundPositionIndication(self.native_engine, is_enabled);
        }
    }

    pub fn set_local_voice_pitch(&self, pitch: f64) -> i32 {
        unsafe {
            return agorartcnative::setLocalVoicePitch(self.native_engine, pitch);
        }
    }

    pub fn set_local_voice_equalization(&self, band_frequency: AUDIO_EQUALIZATION_BAND_FREQUENCY, band_gain: i32) -> i32 {
        unsafe {
            return agorartcnative::setLocalVoiceEqualization(self.native_engine, band_frequency as u32, band_gain);
        }
    }

    pub fn set_local_voice_reverb(&self, reverb_key: AUDIO_REVERB_TYPE, value: i32) -> i32 {
        unsafe {
            return agorartcnative::setLocalVoiceReverb(self.native_engine, reverb_key as u32, value);
        }
    }

    pub fn set_local_voice_changer(&self, voice_changer: VOICE_CHANGER_PRESET) -> i32 {
        unsafe {
            return agorartcnative::setLocalVoiceChanger(self.native_engine, voice_changer as u32);
        }
    }

    pub fn set_local_voice_reverb_preset(&self, reverb_preset: AUDIO_REVERB_PRESET) -> i32 {
        unsafe {
            return agorartcnative::setLocalVoiceReverbPreset(self.native_engine, reverb_preset as u32);
        }
    }

    pub fn set_external_audio_source(&self, enabled: bool, sample_rate: i32, channels: i32) -> i32 {
        unsafe {
            let is_enabled: i32 = if enabled { 1 } else { 0 };
            return agorartcnative::setExternalAudioSource(self.native_engine, is_enabled, sample_rate, channels);
        }
    }

    pub fn set_external_audio_sink(&self, enabled: bool, sample_rate: i32, channels: i32) -> i32 {
        unsafe {
            let is_enabled: i32 = if enabled { 1 } else { 0 };
            return agorartcnative::setExternalAudioSink(self.native_engine, is_enabled, sample_rate, channels);
        }
    }

    pub fn set_recoding_audio_frame_parameters(&self, sample_rate: i32, channel: i32, mode: RAW_AUDIO_FRAME_OP_MODE_TYPE, samples_per_call: i32) -> i32 {
        unsafe {
            return agorartcnative::setRecordingAudioFrameParameters(self.native_engine, sample_rate, channel, mode as u32, samples_per_call);
        }
    }

    pub fn set_playback_audio_frame_parameters(&self, sample_rate: i32, channel: i32, mode: RAW_AUDIO_FRAME_OP_MODE_TYPE, sample_per_call: i32) -> i32 {
        unsafe {
            return agorartcnative::setPlaybackAudioFrameParameters(self.native_engine, sample_rate, channel, mode as u32, sample_per_call);
        }
    }

    pub fn set_mixed_audio_frame_parameters(&self, sample_rate: i32, samples_per_call: i32) -> i32 {
        unsafe {
            return agorartcnative::setMixedAudioFrameParameters(self.native_engine, sample_rate, samples_per_call);
        }
    }

    pub fn enable_encryption(&self, enabled: bool, config: agorartcnative::EncryptionConfig) -> i32 {
        unsafe {
            let is_enabled: i32 = if enabled { 1 } else { 0 };
            return agorartcnative::enableEncryption(self.native_engine, is_enabled, config);
        }
    }

}

impl AgoraRtcChannel {
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

    pub fn join_channel(&self, token: &str, info: &str, uid: u32, options: agorartcnative::ChannelMediaOptions) -> i32 {
        unsafe {
            let the_token: &CStr = &CString::new(token).expect("token new failed");
            let the_info: &CStr = &CString::new(info).expect("info new failed");
            return agorartcnative::channel_joinChannel(self.native_channel, the_token.as_ptr(), the_info.as_ptr(), uid, options);
        }
    }

    pub fn join_channel_with_user_account(&self, token: &str, user_account: &str, options: agorartcnative::ChannelMediaOptions  ) -> i32 {
        unsafe {
            let the_token: &CStr = &CString::new(token).expect("token new failed");
            let the_user_account = &CString::new(user_account).expect("user_account new failed");
            return agorartcnative::channel_joinChannelWithUserAccount(self.native_channel, the_token.as_ptr(), the_user_account.as_ptr(), options);
        }
    }

    pub fn leave_channel(&self) -> i32 {
        unsafe {
            return agorartcnative::channel_leaveChannel(self.native_channel);
        }
    }

    pub fn publish(&self) -> i32 {
        unsafe {
            return agorartcnative::channel_publish(self.native_channel);
        }
    }

    pub fn unpublish(&self) -> i32 {
        unsafe {
            return agorartcnative::channel_unpublish(self.native_channel);
        }
    }

    pub fn get_channel_id(&self) -> String {
        unsafe {
            let s = agorartcnative::channel_channelId(self.native_channel);
            let r = CStr::from_ptr(s).to_string_lossy().into_owned();
            return r;
        }
    }

    pub fn get_call_id(&self) -> String {
        unsafe {
            let s = agorartcnative::channel_getCallId(self.native_channel);
            let r = CStr::from_ptr(s).to_string_lossy().into_owned();
            return r;
        }
    }

    pub fn renew_token(&self, token: &str) -> i32 {
        unsafe {
            let the_token: &CStr = &CString::new(token).expect("token new failed");
            return agorartcnative::channel_renewToken(self.native_channel, the_token.as_ptr());   
        }
    }

    pub fn set_encryption_secret(&self, secret: &str) -> i32 {
        unsafe {
            let the_secret: &CStr = &CString::new(secret).expect("secret new failed");
            return agorartcnative::channel_setEncryptionSecret(self.native_channel, the_secret.as_ptr());
        }
    }

    pub fn set_encryption_mode(&self, encryption_mode: &str) -> i32 {
        unsafe {
            let the_encryption_mode: &CStr = &CString::new(encryption_mode).expect("encryption_mode new failed");
            return agorartcnative::channel_setEncryptionMode(self.native_channel, the_encryption_mode.as_ptr());
        }
    }

    pub fn set_client_role(&self, role: CLIENT_ROLE_TYPE) -> i32 {
        unsafe {
            return agorartcnative::channel_setClientRole(self.native_channel, role as u32);
        }
    }

    pub fn set_remote_user_priority(&self, uid: u32, user_priority: PRIORITY_TYPE) -> i32 {
        unsafe {
            return agorartcnative::channel_setRemoteUserPriority(self.native_channel, uid, user_priority as u32);
        }
    }

    // Mark
    pub fn set_remote_voice_position(&self, uid: u32, pan: f64, gain: f64) -> i32 {
        unsafe {
            return agorartcnative::channel_setRemoteVoicePosition(self.native_channel, uid, pan, gain);
        }
    }

    pub fn set_remote_render_mode(&self, uid: u32, render_mode: RENDER_MODE_TYPE, mirror_mode: VIDEO_MIRROR_MODE_TYPE) -> i32 {
        unsafe {
            return agorartcnative::channel_setRemoteRenderMode(self.native_channel, uid, render_mode as u32, mirror_mode as u32);
        }
    }

    pub fn set_default_mute_all_remote_audio_streams(&self, mute: bool) -> i32 {
        unsafe {
            let is_mute: i32 = if mute { 1 } else { 0 };
            return agorartcnative::channel_setDefaultMuteAllRemoteAudioStreams(self.native_channel, is_mute);
        }
    }

    pub fn set_default_mute_all_remote_video_streams(&self, mute: bool) -> i32 {
        unsafe {
            let is_mute: i32 = if mute { 1 } else { 0 };
            return agorartcnative::channel_setDefaultMuteAllRemoteVideoStreams(self.native_channel, is_mute);
        }
    }

    pub fn mute_all_remote_audio_streams(&self, mute: bool) -> i32 {
        unsafe {
            let is_mute: i32 = if mute { 1 } else { 0 };
            return agorartcnative::channel_muteAllRemoteAudioStreams(self.native_channel, is_mute);
        }
    }

    pub fn adjust_user_playback_signal_volume(&self, uid: u32, volume: i32) -> i32 {
        unsafe {
            return agorartcnative::channel_adjustUserPlaybackSignalVolume(self.native_channel, uid, volume);
        }
    }
    
    pub fn mute_remote_audio_stream(&self, uid: u32, mute: bool) -> i32 {
        unsafe {
            let is_mute: i32 = if mute { 1 } else { 0 };
            return agorartcnative::channel_muteRemoteAudioStream(self.native_channel, uid, is_mute);
        }
    }

    pub fn mute_all_remote_video_streams(&self, mute: bool) -> i32 {
        unsafe {
            let is_mute: i32 = if mute { 1 } else { 0 };
            return agorartcnative::channel_muteAllRemoteVideoStreams(self.native_channel, is_mute);
        }
    }

    pub fn mute_remote_video_stream(&self, uid: u32, mute: bool) -> i32 {
        unsafe {
            let is_mute: i32 = if mute { 1 } else { 0 };
            return agorartcnative::channel_muteRemoteVideoStream(self.native_channel, uid, is_mute);
        }
    }

    pub fn set_remote_video_stream_type(&self, uid: u32, stream_type: REMOTE_VIDEO_STREAM_TYPE) -> i32 {
        unsafe {
            return agorartcnative::channel_setRemoteVideoStreamType(self.native_channel, uid, stream_type as u32);
        }
    }

    pub fn set_remote_default_video_stream_type(&self, stream_type: REMOTE_VIDEO_STREAM_TYPE) -> i32 {
        unsafe {
            return agorartcnative::channel_setRemoteDefaultVideoStreamType(self.native_channel, stream_type as u32);
        }
    }

    pub fn add_publish_stream_url(&self, url: &str, transcoding_enabled: bool) -> i32 {
        unsafe {
            let the_url: &CStr = &CString::new(url).expect("url new failed");
            let is_transcoding_enabled: i32 = if transcoding_enabled { 1 } else { 0 };
            return agorartcnative::channel_addPublishStreamUrl(self.native_channel, the_url.as_ptr(), is_transcoding_enabled);
        }
    }

    pub fn remove_publish_stream_url(&self, url: &str) -> i32 {
        unsafe {
            let the_url: &CStr = &CString::new(url).expect("url new failed");
            return agorartcnative::channel_removePublishStreamUrl(self.native_channel, the_url.as_ptr());
        }
    }

    pub fn set_live_transcoding(&self, transcoding: *const agorartcnative::LiveTranscoding) -> i32 {
        unsafe {
            return agorartcnative::channel_setLiveTranscoding(self.native_channel, transcoding);
        }
    }

    pub fn add_inject_stream_url(&self, url: &str, config: agorartcnative::InjectStreamConfig) -> i32 {
        unsafe {
            let the_url: &CStr = &CString::new(url).expect("url new failed");
            return agorartcnative::channel_addInjectStreamUrl(self.native_channel, the_url.as_ptr(), config);
        }
    }

    pub fn remove_inject_stream_url(&self, url: &str) -> i32 {
        unsafe {
            let the_url: &CStr = &CString::new(url).expect("url new failed");
            return agorartcnative::channel_removeInjectStreamUrl(self.native_channel, the_url.as_ptr());
        }
    }

    pub fn start_channel_media_relay(&self, config: agorartcnative::ChannelMediaRelayConfiguration) -> i32 {
        unsafe {
            return agorartcnative::channel_startChannelMediaRelay(self.native_channel, config);
        }
    }

    pub fn update_channel_media_relay(&self, config: agorartcnative::ChannelMediaRelayConfiguration) -> i32 {
        unsafe {
            return agorartcnative::channel_updateChannelMediaRelay(self.native_channel, config);
        }
    }

    pub fn stop_channel_media_relay(&self) -> i32 {
        unsafe {
            return agorartcnative::channel_stopChannelMediaRelay(self.native_channel);
        }
    }

    pub fn create_data_stream(&self, stream_id: *mut i32, reliable: bool, ordered: bool) -> i32 {
        unsafe {
            let is_reliable: i32 = if reliable { 1 } else { 0 };
            let is_ordered: i32 = if ordered { 1 } else { 0 };
            return agorartcnative::channel_createDataStream(self.native_channel, stream_id, is_reliable, is_ordered);
        }
    }

    pub fn send_stream_message(&self, stream_id: i32, data: &str, length: i64) -> i32 {
        unsafe {
            let the_data: &CStr = &CString::new(data).expect("data new failed");
            return agorartcnative::channel_sendStreamMessage(self.native_channel, stream_id, the_data.as_ptr(), length);
        }
    }

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