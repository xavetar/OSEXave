/*
 * Copyright 2023 Stanislav Mikhailov (xavetar)
 *
 * Licensed under the Creative Commons Zero v1.0 Universal (CC0) License.
 * You may obtain a copy of the License at
 *
 *     http://creativecommons.org/publicdomain/zero/1.0/
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the CC0 license is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[cfg(any(feature = "windows"))]
mod audclnt;
#[cfg(any(feature = "windows"))]
mod audio;
#[cfg(any(feature = "windows"))]
mod backup;
#[cfg(any(feature = "windows"))]
mod bcd;
#[cfg(any(feature = "windows"))]
mod bluetooth_att;
#[cfg(any(feature = "windows"))]
mod cert;
#[cfg(any(feature = "windows"))]
mod complus;
#[cfg(any(feature = "windows"))]
mod configuration;
#[cfg(any(feature = "windows"))]
mod control;
#[cfg(any(feature = "windows"))]
mod d2d;
#[cfg(any(feature = "windows"))]
mod debuggers;
#[cfg(any(feature = "windows"))]
mod defrag;
#[cfg(any(feature = "windows"))]
mod direct3d10;
#[cfg(any(feature = "windows"))]
mod direct3d11;
#[cfg(any(feature = "windows"))]
mod direct3d11_debug;
#[cfg(any(feature = "windows"))]
mod direct3d12;
#[cfg(any(feature = "windows"))]
mod direct_draw;
#[cfg(any(feature = "windows"))]
mod directmusic;
#[cfg(any(feature = "windows"))]
mod dispatch;
#[cfg(any(feature = "windows"))]
mod dlt;
#[cfg(any(feature = "windows"))]
mod dxcore;
#[cfg(any(feature = "windows"))]
mod dxgi;
#[cfg(any(feature = "windows"))]
mod dxgi_ddi;
#[cfg(any(feature = "windows"))]
mod eap;
#[cfg(any(feature = "windows"))]
mod eas;
#[cfg(any(feature = "windows"))]
mod fve;
#[cfg(any(feature = "windows"))]
mod fwp;
#[cfg(any(feature = "windows"))]
mod graphics;
#[cfg(any(feature = "windows"))]
mod hsp_services;
#[cfg(any(feature = "windows"))]
mod hsp_software;
#[cfg(any(feature = "windows"))]
mod http;
#[cfg(any(feature = "windows"))]
mod imapi2;
#[cfg(any(feature = "windows"))]
mod input;
#[cfg(any(feature = "windows"))]
mod internet;
#[cfg(any(feature = "windows"))]
mod ioring;
#[cfg(any(feature = "windows"))]
mod itf;
#[cfg(any(feature = "windows"))]
mod jscript;
#[cfg(any(feature = "windows"))]
mod jsdebug;
#[cfg(any(feature = "windows"))]
mod leap;
#[cfg(any(feature = "windows"))]
mod mbn_powershell;
#[cfg(any(feature = "windows"))]
mod mca;
#[cfg(any(feature = "windows"))]
mod mediaserver;
#[cfg(any(feature = "windows"))]
mod msmq;
#[cfg(any(feature = "windows"))]
mod ndis;
#[cfg(any(feature = "windows"))]
mod ntdsb;
#[cfg(any(feature = "windows"))]
mod null;
#[cfg(any(feature = "windows"))]
mod online_id;
#[cfg(any(feature = "windows"))]
mod opc;
#[cfg(any(feature = "windows"))]
mod p2p;
#[cfg(any(feature = "windows"))]
mod p2p_int;
#[cfg(any(feature = "windows"))]
mod pint_status_code;
#[cfg(any(feature = "windows"))]
mod presentation;
#[cfg(any(feature = "windows"))]
mod quic;
#[cfg(any(feature = "windows"))]
mod rddb;
#[cfg(any(feature = "windows"))]
mod rpc;
#[cfg(any(feature = "windows"))]
mod rtc_interface;
#[cfg(any(feature = "windows"))]
mod sb;
#[cfg(any(feature = "windows"))]
mod scard;
#[cfg(any(feature = "windows"))]
mod sdiag;
#[cfg(any(feature = "windows"))]
mod security_sspi;
#[cfg(any(feature = "windows"))]
mod setupapi;
#[cfg(any(feature = "windows"))]
mod shared_vhdx;
#[cfg(any(feature = "windows"))]
mod shell_nap;
#[cfg(any(feature = "windows"))]
mod sip_status_code;
#[cfg(any(feature = "windows"))]
mod smb;
#[cfg(any(feature = "windows"))]
mod sqlite;
#[cfg(any(feature = "windows"))]
mod staterepository;
#[cfg(any(feature = "windows"))]
mod storage;
#[cfg(any(feature = "windows"))]
mod syncengine;
#[cfg(any(feature = "windows"))]
mod tiering;
#[cfg(any(feature = "windows"))]
mod tpm_services;
#[cfg(any(feature = "windows"))]
mod tpm_software;
#[cfg(any(feature = "windows"))]
mod ui;
#[cfg(any(feature = "windows"))]
mod urt;
#[cfg(any(feature = "windows"))]
mod user_mode_security_core;
#[cfg(any(feature = "windows"))]
mod usermode_filter_manager;
#[cfg(any(feature = "windows"))]
mod usermode_hns;
#[cfg(any(feature = "windows"))]
mod usermode_hypervisor;
#[cfg(any(feature = "windows"))]
mod usermode_licensing;
#[cfg(any(feature = "windows"))]
mod usermode_sdbus;
#[cfg(any(feature = "windows"))]
mod usermode_spaces;
#[cfg(any(feature = "windows"))]
mod usermode_vhd;
#[cfg(any(feature = "windows"))]
mod usermode_virtualization;
#[cfg(any(feature = "windows"))]
mod usermode_volmgr;
#[cfg(any(feature = "windows"))]
mod usermode_volsnap;
#[cfg(any(feature = "windows"))]
mod utc;
#[cfg(any(feature = "windows"))]
mod wds_content_provider;
#[cfg(any(feature = "windows"))]
mod wds_mc_client;
#[cfg(any(feature = "windows"))]
mod wds_mc_server;
#[cfg(any(feature = "windows"))]
mod wds_tm;
#[cfg(any(feature = "windows"))]
mod web;
#[cfg(any(feature = "windows"))]
mod web_socket;
#[cfg(any(feature = "windows"))]
mod webservices_winpe;
#[cfg(any(feature = "windows"))]
mod wep;
#[cfg(any(feature = "windows"))]
mod wer;
#[cfg(any(feature = "windows"))]
mod win32;
#[cfg(any(feature = "windows"))]
mod wincodec_dwrite_dwm;
#[cfg(any(feature = "windows"))]
mod windows;
#[cfg(any(feature = "windows"))]
mod windows_ce;
#[cfg(any(feature = "windows"))]
mod windows_defender;
#[cfg(any(feature = "windows"))]
mod windows_setup_pla;
#[cfg(any(feature = "windows"))]
mod windows_store;
#[cfg(any(feature = "windows"))]
mod windowsupdate;
#[cfg(any(feature = "windows"))]
mod winml;
#[cfg(any(feature = "windows"))]
mod winrm;
#[cfg(any(feature = "windows"))]
mod wpn;
#[cfg(any(feature = "windows"))]
mod xactengine;
#[cfg(any(feature = "windows"))]
mod xaml;
#[cfg(any(feature = "windows"))]
mod xapo;
#[cfg(any(feature = "windows"))]
mod xaudio2;
#[cfg(any(feature = "windows"))]
mod xps;
#[cfg(any(feature = "windows"))]

#[cfg(any(feature = "windows"))]
pub use audclnt::{*};
#[cfg(any(feature = "windows"))]
pub use audio::{*};
#[cfg(any(feature = "windows"))]
pub use backup::{*};
#[cfg(any(feature = "windows"))]
pub use bcd::{*};
#[cfg(any(feature = "windows"))]
pub use bluetooth_att::{*};
#[cfg(any(feature = "windows"))]
pub use cert::{*};
#[cfg(any(feature = "windows"))]
pub use complus::{*};
#[cfg(any(feature = "windows"))]
pub use configuration::{*};
#[cfg(any(feature = "windows"))]
pub use control::{*};
#[cfg(any(feature = "windows"))]
pub use d2d::{*};
#[cfg(any(feature = "windows"))]
pub use debuggers::{*};
#[cfg(any(feature = "windows"))]
pub use defrag::{*};
#[cfg(any(feature = "windows"))]
pub use direct3d10::{*};
#[cfg(any(feature = "windows"))]
pub use direct3d11::{*};
#[cfg(any(feature = "windows"))]
pub use direct3d11_debug::{*};
#[cfg(any(feature = "windows"))]
pub use direct3d12::{*};
#[cfg(any(feature = "windows"))]
pub use direct_draw::{*};
#[cfg(any(feature = "windows"))]
pub use directmusic::{*};
#[cfg(any(feature = "windows"))]
pub use dispatch::{*};
#[cfg(any(feature = "windows"))]
pub use dlt::{*};
#[cfg(any(feature = "windows"))]
pub use dxcore::{*};
#[cfg(any(feature = "windows"))]
pub use dxgi::{*};
#[cfg(any(feature = "windows"))]
pub use dxgi_ddi::{*};
#[cfg(any(feature = "windows"))]
pub use eap::{*};
#[cfg(any(feature = "windows"))]
pub use eas::{*};
#[cfg(any(feature = "windows"))]
pub use fve::{*};
#[cfg(any(feature = "windows"))]
pub use fwp::{*};
#[cfg(any(feature = "windows"))]
pub use graphics::{*};
#[cfg(any(feature = "windows"))]
pub use hsp_services::{*};
#[cfg(any(feature = "windows"))]
pub use hsp_software::{*};
#[cfg(any(feature = "windows"))]
pub use http::{*};
#[cfg(any(feature = "windows"))]
pub use imapi2::{*};
#[cfg(any(feature = "windows"))]
pub use input::{*};
#[cfg(any(feature = "windows"))]
pub use internet::{*};
#[cfg(any(feature = "windows"))]
pub use ioring::{*};
#[cfg(any(feature = "windows"))]
pub use itf::{*};
#[cfg(any(feature = "windows"))]
pub use jscript::{*};
#[cfg(any(feature = "windows"))]
pub use jsdebug::{*};
#[cfg(any(feature = "windows"))]
pub use leap::{*};
#[cfg(any(feature = "windows"))]
pub use mbn_powershell::{*};
#[cfg(any(feature = "windows"))]
pub use mca::{*};
#[cfg(any(feature = "windows"))]
pub use mediaserver::{*};
#[cfg(any(feature = "windows"))]
pub use msmq::{*};
#[cfg(any(feature = "windows"))]
pub use ndis::{*};
#[cfg(any(feature = "windows"))]
pub use ntdsb::{*};
#[cfg(any(feature = "windows"))]
pub use null::{*};
#[cfg(any(feature = "windows"))]
pub use online_id::{*};
#[cfg(any(feature = "windows"))]
pub use opc::{*};
#[cfg(any(feature = "windows"))]
pub use p2p::{*};
#[cfg(any(feature = "windows"))]
pub use p2p_int::{*};
#[cfg(any(feature = "windows"))]
pub use pint_status_code::{*};
#[cfg(any(feature = "windows"))]
pub use presentation::{*};
#[cfg(any(feature = "windows"))]
pub use quic::{*};
#[cfg(any(feature = "windows"))]
pub use rddb::{*};
#[cfg(any(feature = "windows"))]
pub use rpc::{*};
#[cfg(any(feature = "windows"))]
pub use rtc_interface::{*};
#[cfg(any(feature = "windows"))]
pub use sb::{*};
#[cfg(any(feature = "windows"))]
pub use scard::{*};
#[cfg(any(feature = "windows"))]
pub use sdiag::{*};
#[cfg(any(feature = "windows"))]
pub use security_sspi::{*};
#[cfg(any(feature = "windows"))]
pub use setupapi::{*};
#[cfg(any(feature = "windows"))]
pub use shared_vhdx::{*};
#[cfg(any(feature = "windows"))]
pub use shell_nap::{*};
#[cfg(any(feature = "windows"))]
pub use sip_status_code::{*};
#[cfg(any(feature = "windows"))]
pub use smb::{*};
#[cfg(any(feature = "windows"))]
pub use sqlite::{*};
#[cfg(any(feature = "windows"))]
pub use staterepository::{*};
#[cfg(any(feature = "windows"))]
pub use storage::{*};
#[cfg(any(feature = "windows"))]
pub use syncengine::{*};
#[cfg(any(feature = "windows"))]
pub use tiering::{*};
#[cfg(any(feature = "windows"))]
pub use tpm_services::{*};
#[cfg(any(feature = "windows"))]
pub use tpm_software::{*};
#[cfg(any(feature = "windows"))]
pub use ui::{*};
#[cfg(any(feature = "windows"))]
pub use urt::{*};
#[cfg(any(feature = "windows"))]
pub use user_mode_security_core::{*};
#[cfg(any(feature = "windows"))]
pub use usermode_filter_manager::{*};
#[cfg(any(feature = "windows"))]
pub use usermode_hns::{*};
#[cfg(any(feature = "windows"))]
pub use usermode_hypervisor::{*};
#[cfg(any(feature = "windows"))]
pub use usermode_licensing::{*};
#[cfg(any(feature = "windows"))]
pub use usermode_sdbus::{*};
#[cfg(any(feature = "windows"))]
pub use usermode_spaces::{*};
#[cfg(any(feature = "windows"))]
pub use usermode_vhd::{*};
#[cfg(any(feature = "windows"))]
pub use usermode_virtualization::{*};
#[cfg(any(feature = "windows"))]
pub use usermode_volmgr::{*};
#[cfg(any(feature = "windows"))]
pub use usermode_volsnap::{*};
#[cfg(any(feature = "windows"))]
pub use utc::{*};
#[cfg(any(feature = "windows"))]
pub use wds_content_provider::{*};
#[cfg(any(feature = "windows"))]
pub use wds_mc_client::{*};
#[cfg(any(feature = "windows"))]
pub use wds_mc_server::{*};
#[cfg(any(feature = "windows"))]
pub use wds_tm::{*};
#[cfg(any(feature = "windows"))]
pub use web::{*};
#[cfg(any(feature = "windows"))]
pub use web_socket::{*};
#[cfg(any(feature = "windows"))]
pub use webservices_winpe::{*};
#[cfg(any(feature = "windows"))]
pub use wep::{*};
#[cfg(any(feature = "windows"))]
pub use wer::{*};
#[cfg(any(feature = "windows"))]
pub use win32::{*};
#[cfg(any(feature = "windows"))]
pub use wincodec_dwrite_dwm::{*};
#[cfg(any(feature = "windows"))]
pub use windows::{*};
#[cfg(any(feature = "windows"))]
pub use windows_ce::{*};
#[cfg(any(feature = "windows"))]
pub use windows_defender::{*};
#[cfg(any(feature = "windows"))]
pub use windows_setup_pla::{*};
#[cfg(any(feature = "windows"))]
pub use windows_store::{*};
#[cfg(any(feature = "windows"))]
pub use windowsupdate::{*};
#[cfg(any(feature = "windows"))]
pub use winml::{*};
#[cfg(any(feature = "windows"))]
pub use winrm::{*};
#[cfg(any(feature = "windows"))]
pub use wpn::{*};
#[cfg(any(feature = "windows"))]
pub use xactengine::{*};
#[cfg(any(feature = "windows"))]
pub use xaml::{*};
#[cfg(any(feature = "windows"))]
pub use xapo::{*};
#[cfg(any(feature = "windows"))]
pub use xaudio2::{*};
#[cfg(any(feature = "windows"))]
pub use xps::{*};
