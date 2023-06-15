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

#[cfg(target_os = "windows")]
mod audclnt;
#[cfg(target_os = "windows")]
mod audio;
#[cfg(target_os = "windows")]
mod backup;
#[cfg(target_os = "windows")]
mod bcd;
#[cfg(target_os = "windows")]
mod bluetooth_att;
#[cfg(target_os = "windows")]
mod cert;
#[cfg(target_os = "windows")]
mod complus;
#[cfg(target_os = "windows")]
mod control;
#[cfg(target_os = "windows")]
mod debuggers;
#[cfg(target_os = "windows")]
mod defrag;
#[cfg(target_os = "windows")]
mod deployment_services_content_provider;
#[cfg(target_os = "windows")]
mod deployment_services_multicast_client;
#[cfg(target_os = "windows")]
mod deployment_services_multicast_server;
#[cfg(target_os = "windows")]
mod deployment_services_transport_management;
#[cfg(target_os = "windows")]
mod direct2d;
#[cfg(target_os = "windows")]
mod direct3d10;
#[cfg(target_os = "windows")]
mod direct3d11;
#[cfg(target_os = "windows")]
mod direct3d11_debug;
#[cfg(target_os = "windows")]
mod direct3d12;
#[cfg(target_os = "windows")]
mod direct_draw;
#[cfg(target_os = "windows")]
mod directmusic;
#[cfg(target_os = "windows")]
mod dispatch;
#[cfg(target_os = "windows")]
mod dlt;
#[cfg(target_os = "windows")]
mod drt;
#[cfg(target_os = "windows")]
mod dxcore;
#[cfg(target_os = "windows")]
mod dxgi;
#[cfg(target_os = "windows")]
mod dxgi_ddi;
#[cfg(target_os = "windows")]
mod eap;
#[cfg(target_os = "windows")]
mod eas;
#[cfg(target_os = "windows")]
mod fwp;
#[cfg(target_os = "windows")]
mod graphics;
#[cfg(target_os = "windows")]
mod hsp_services;
#[cfg(target_os = "windows")]
mod hsp_software;
#[cfg(target_os = "windows")]
mod http;
#[cfg(target_os = "windows")]
mod imapi2;
#[cfg(target_os = "windows")]
mod input;
#[cfg(target_os = "windows")]
mod internet;
#[cfg(target_os = "windows")]
mod ioring;
#[cfg(target_os = "windows")]
mod itf;
#[cfg(target_os = "windows")]
mod jscript;
#[cfg(target_os = "windows")]
mod jsdebug;
#[cfg(target_os = "windows")]
mod leap;
#[cfg(target_os = "windows")]
mod mbn;
#[cfg(target_os = "windows")]
mod mca_error_code;
#[cfg(target_os = "windows")]
mod mediaserver;
#[cfg(target_os = "windows")]
mod msmq;
#[cfg(target_os = "windows")]
mod ndis;
#[cfg(target_os = "windows")]
mod ntdsb;
#[cfg(target_os = "windows")]
mod null;
#[cfg(target_os = "windows")]
mod online_id;
#[cfg(target_os = "windows")]
mod opc;
#[cfg(target_os = "windows")]
mod p2p;
#[cfg(target_os = "windows")]
mod pint_status_code;
#[cfg(target_os = "windows")]
mod pla;
#[cfg(target_os = "windows")]
mod powershell;
#[cfg(target_os = "windows")]
mod presentation;
#[cfg(target_os = "windows")]
mod quic;
#[cfg(target_os = "windows")]
mod rddb;
#[cfg(target_os = "windows")]
mod rpc;
#[cfg(target_os = "windows")]
mod rtc_interface;
#[cfg(target_os = "windows")]
mod sb;
#[cfg(target_os = "windows")]
mod scard;
#[cfg(target_os = "windows")]
mod sdiag;
#[cfg(target_os = "windows")]
mod setupapi;
#[cfg(target_os = "windows")]
mod shell;
#[cfg(target_os = "windows")]
mod sip_status_code;
#[cfg(target_os = "windows")]
mod smb;
#[cfg(target_os = "windows")]
mod sqlite;
#[cfg(target_os = "windows")]
mod sspi;
#[cfg(target_os = "windows")]
mod staterepository;
#[cfg(target_os = "windows")]
mod storage;
#[cfg(target_os = "windows")]
mod syncengine;
#[cfg(target_os = "windows")]
mod tiering;
#[cfg(target_os = "windows")]
mod tpm_services;
#[cfg(target_os = "windows")]
mod tpm_software;
#[cfg(target_os = "windows")]
mod ui;
#[cfg(target_os = "windows")]
mod urt;
#[cfg(target_os = "windows")]
mod user_mode_security_core;
#[cfg(target_os = "windows")]
mod usermode_filter_manager;
#[cfg(target_os = "windows")]
mod usermode_hns;
#[cfg(target_os = "windows")]
mod usermode_hypervisor;
#[cfg(target_os = "windows")]
mod usermode_licensing;
#[cfg(target_os = "windows")]
mod usermode_sdbus;
#[cfg(target_os = "windows")]
mod usermode_spaces;
#[cfg(target_os = "windows")]
mod usermode_vhd;
#[cfg(target_os = "windows")]
mod usermode_virtualization;
#[cfg(target_os = "windows")]
mod usermode_volmgr;
#[cfg(target_os = "windows")]
mod usermode_volsnap;
#[cfg(target_os = "windows")]
mod utc;
#[cfg(target_os = "windows")]
mod svhdx;
#[cfg(target_os = "windows")]
mod web;
#[cfg(target_os = "windows")]
mod web_socket;
#[cfg(target_os = "windows")]
mod webservices;
#[cfg(target_os = "windows")]
mod wep;
#[cfg(target_os = "windows")]
mod wer;
#[cfg(target_os = "windows")]
mod wia;
#[cfg(target_os = "windows")]
mod win32;
#[cfg(target_os = "windows")]
mod wincodec_dwrite_dwm;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
mod windows_ce;
#[cfg(target_os = "windows")]
mod windows_defender;
#[cfg(target_os = "windows")]
mod windows_store;
#[cfg(target_os = "windows")]
mod windowsupdate;
#[cfg(target_os = "windows")]
mod winml;
#[cfg(target_os = "windows")]
mod winrm;
#[cfg(target_os = "windows")]
mod wpn;
#[cfg(target_os = "windows")]
mod xactengine;
#[cfg(target_os = "windows")]
mod xaml;
#[cfg(target_os = "windows")]
mod xapo;
#[cfg(target_os = "windows")]
mod xaudio2;
#[cfg(target_os = "windows")]
mod xps;

#[cfg(target_os = "windows")]
pub use audclnt::{*};
#[cfg(target_os = "windows")]
pub use audio::{*};
#[cfg(target_os = "windows")]
pub use backup::{*};
#[cfg(target_os = "windows")]
pub use bcd::{*};
#[cfg(target_os = "windows")]
pub use bluetooth_att::{*};
#[cfg(target_os = "windows")]
pub use cert::{*};
#[cfg(target_os = "windows")]
pub use complus::{*};
#[cfg(target_os = "windows")]
pub use control::{*};
#[cfg(target_os = "windows")]
pub use debuggers::{*};
#[cfg(target_os = "windows")]
pub use defrag::{*};
#[cfg(target_os = "windows")]
pub use deployment_services_content_provider::{*};
#[cfg(target_os = "windows")]
pub use deployment_services_multicast_client::{*};
#[cfg(target_os = "windows")]
pub use deployment_services_multicast_server::{*};
#[cfg(target_os = "windows")]
pub use deployment_services_transport_management::{*};
#[cfg(target_os = "windows")]
pub use direct2d::{*};
#[cfg(target_os = "windows")]
pub use direct3d10::{*};
#[cfg(target_os = "windows")]
pub use direct3d11::{*};
#[cfg(target_os = "windows")]
pub use direct3d11_debug::{*};
#[cfg(target_os = "windows")]
pub use direct3d12::{*};
#[cfg(target_os = "windows")]
pub use direct_draw::{*};
#[cfg(target_os = "windows")]
pub use directmusic::{*};
#[cfg(target_os = "windows")]
pub use dispatch::{*};
#[cfg(target_os = "windows")]
pub use dlt::{*};
#[cfg(target_os = "windows")]
pub use drt::{*};
#[cfg(target_os = "windows")]
pub use dxcore::{*};
#[cfg(target_os = "windows")]
pub use dxgi::{*};
#[cfg(target_os = "windows")]
pub use dxgi_ddi::{*};
#[cfg(target_os = "windows")]
pub use eap::{*};
#[cfg(target_os = "windows")]
pub use eas::{*};
#[cfg(target_os = "windows")]
pub use fve::{*};
#[cfg(target_os = "windows")]
pub use fwp::{*};
#[cfg(target_os = "windows")]
pub use graphics::{*};
#[cfg(target_os = "windows")]
pub use hsp_services::{*};
#[cfg(target_os = "windows")]
pub use hsp_software::{*};
#[cfg(target_os = "windows")]
pub use http::{*};
#[cfg(target_os = "windows")]
pub use imapi2::{*};
#[cfg(target_os = "windows")]
pub use input::{*};
#[cfg(target_os = "windows")]
pub use internet::{*};
#[cfg(target_os = "windows")]
pub use ioring::{*};
#[cfg(target_os = "windows")]
pub use itf::{*};
#[cfg(target_os = "windows")]
pub use jscript::{*};
#[cfg(target_os = "windows")]
pub use jsdebug::{*};
#[cfg(target_os = "windows")]
pub use leap::{*};
#[cfg(target_os = "windows")]
pub use mbn::{*};
#[cfg(target_os = "windows")]
pub use mca_error_code::{*};
#[cfg(target_os = "windows")]
pub use mediaserver::{*};
#[cfg(target_os = "windows")]
pub use msmq::{*};
#[cfg(target_os = "windows")]
pub use ndis::{*};
#[cfg(target_os = "windows")]
pub use ntdsb::{*};
#[cfg(target_os = "windows")]
pub use null::{*};
#[cfg(target_os = "windows")]
pub use online_id::{*};
#[cfg(target_os = "windows")]
pub use opc::{*};
#[cfg(target_os = "windows")]
pub use p2p::{*};
#[cfg(target_os = "windows")]
pub use pint_status_code::{*};
#[cfg(target_os = "windows")]
pub use pla::{*};
#[cfg(target_os = "windows")]
pub use powershell::{*};
#[cfg(target_os = "windows")]
pub use presentation::{*};
#[cfg(target_os = "windows")]
pub use quic::{*};
#[cfg(target_os = "windows")]
pub use rddb::{*};
#[cfg(target_os = "windows")]
pub use rpc::{*};
#[cfg(target_os = "windows")]
pub use rtc_interface::{*};
#[cfg(target_os = "windows")]
pub use sb::{*};
#[cfg(target_os = "windows")]
pub use scard::{*};
#[cfg(target_os = "windows")]
pub use sdiag::{*};
#[cfg(target_os = "windows")]
pub use setupapi::{*};
#[cfg(target_os = "windows")]
pub use shell::{*};
#[cfg(target_os = "windows")]
pub use sip_status_code::{*};
#[cfg(target_os = "windows")]
pub use smb::{*};
#[cfg(target_os = "windows")]
pub use sqlite::{*};
#[cfg(target_os = "windows")]
pub use sspi::{*};
#[cfg(target_os = "windows")]
pub use staterepository::{*};
#[cfg(target_os = "windows")]
pub use storage::{*};
#[cfg(target_os = "windows")]
pub use syncengine::{*};
#[cfg(target_os = "windows")]
pub use tiering::{*};
#[cfg(target_os = "windows")]
pub use tpm_services::{*};
#[cfg(target_os = "windows")]
pub use tpm_software::{*};
#[cfg(target_os = "windows")]
pub use ui::{*};
#[cfg(target_os = "windows")]
pub use urt::{*};
#[cfg(target_os = "windows")]
pub use user_mode_security_core::{*};
#[cfg(target_os = "windows")]
pub use usermode_filter_manager::{*};
#[cfg(target_os = "windows")]
pub use usermode_hns::{*};
#[cfg(target_os = "windows")]
pub use usermode_hypervisor::{*};
#[cfg(target_os = "windows")]
pub use usermode_licensing::{*};
#[cfg(target_os = "windows")]
pub use usermode_sdbus::{*};
#[cfg(target_os = "windows")]
pub use usermode_spaces::{*};
#[cfg(target_os = "windows")]
pub use usermode_vhd::{*};
#[cfg(target_os = "windows")]
pub use usermode_virtualization::{*};
#[cfg(target_os = "windows")]
pub use usermode_volmgr::{*};
#[cfg(target_os = "windows")]
pub use usermode_volsnap::{*};
#[cfg(target_os = "windows")]
pub use utc::{*};
#[cfg(target_os = "windows")]
pub use svhdx::{*};
#[cfg(target_os = "windows")]
pub use web::{*};
#[cfg(target_os = "windows")]
pub use web_socket::{*};
#[cfg(target_os = "windows")]
pub use webservices::{*};
#[cfg(target_os = "windows")]
pub use wep::{*};
#[cfg(target_os = "windows")]
pub use wer::{*};
#[cfg(target_os = "windows")]
pub use wia::{*};
#[cfg(target_os = "windows")]
pub use win32::{*};
#[cfg(target_os = "windows")]
pub use wincodec_dwrite_dwm::{*};
#[cfg(target_os = "windows")]
pub use windows::{*};
#[cfg(target_os = "windows")]
pub use windows_ce::{*};
#[cfg(target_os = "windows")]
pub use windows_defender::{*};
#[cfg(target_os = "windows")]
pub use windows_store::{*};
#[cfg(target_os = "windows")]
pub use windowsupdate::{*};
#[cfg(target_os = "windows")]
pub use winml::{*};
#[cfg(target_os = "windows")]
pub use winrm::{*};
#[cfg(target_os = "windows")]
pub use wpn::{*};
#[cfg(target_os = "windows")]
pub use xactengine::{*};
#[cfg(target_os = "windows")]
pub use xaml::{*};
#[cfg(target_os = "windows")]
pub use xapo::{*};
#[cfg(target_os = "windows")]
pub use xaudio2::{*};
#[cfg(target_os = "windows")]
pub use xps::{*};
