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

pub enum WIA {
    WIA_STATUS_END_OF_MEDIA = 0x00210001,
    WIA_STATUS_WARMING_UP = 0x00210002,
    WIA_STATUS_CALIBRATING = 0x00210003,
    WIA_STATUS_RESERVING_NETWORK_DEVICE = 0x00210006,
    WIA_STATUS_NETWORK_DEVICE_RESERVED = 0x00210007,
    WIA_STATUS_CLEAR = 0x00210008,
    WIA_STATUS_SKIP_ITEM = 0x00210009,
    WIA_STATUS_NOT_HANDLED = 0x0021000A,
    WIA_ERROR_GENERAL_ERROR = 0x80210001,
    WIA_ERROR_PAPER_JAM = 0x80210002,
    WIA_ERROR_PAPER_EMPTY = 0x80210003,
    WIA_ERROR_PAPER_PROBLEM = 0x80210004,
    WIA_ERROR_OFFLINE = 0x80210005,
    WIA_ERROR_BUSY = 0x80210006,
    WIA_ERROR_WARMING_UP = 0x80210007,
    WIA_ERROR_USER_INTERVENTION = 0x80210008,
    WIA_ERROR_ITEM_DELETED = 0x80210009,
    WIA_ERROR_DEVICE_COMMUNICATION = 0x8021000A,
    WIA_ERROR_INVALID_COMMAND = 0x8021000B,
    WIA_ERROR_INCORRECT_HARDWARE_SETTING = 0x8021000C,
    WIA_ERROR_DEVICE_LOCKED = 0x8021000D,
    WIA_ERROR_EXCEPTION_IN_DRIVER = 0x8021000E,
    WIA_ERROR_INVALID_DRIVER_RESPONSE = 0x8021000F,
    WIA_ERROR_COVER_OPEN = 0x80210010,
    WIA_ERROR_LAMP_OFF = 0x80210011,
    WIA_ERROR_DESTINATION = 0x80210012,
    WIA_ERROR_NETWORK_RESERVATION_FAILED = 0x80210013,
    WIA_ERROR_MULTI_FEED = 0x80210014,
    WIA_S_NO_DEVICE_AVAILABLE = 0x80210015,
    WIA_ERROR_MAXIMUM_PRINTER_ENDORSER_COUNTER = 0x80210015,
}

impl WIA {
    pub fn description(&self) -> &'static str {
        match self {
            WIA::WIA_STATUS_END_OF_MEDIA => "End of Media",
            WIA::WIA_STATUS_WARMING_UP => "Warming Up",
            WIA::WIA_STATUS_CALIBRATING => "Calibrating",
            WIA::WIA_STATUS_RESERVING_NETWORK_DEVICE => "Reserving Network Device",
            WIA::WIA_STATUS_NETWORK_DEVICE_RESERVED => "Network Device reserved",
            WIA::WIA_STATUS_CLEAR => "Clear",
            WIA::WIA_STATUS_SKIP_ITEM => "Skip Item",
            WIA::WIA_STATUS_NOT_HANDLED => "Not Handled",
            WIA::WIA_ERROR_GENERAL_ERROR => "An unknown error has occurred with the WIA device.",
            WIA::WIA_ERROR_PAPER_JAM => "Paper is jammed in the scanner's document feeder.",
            WIA::WIA_ERROR_PAPER_EMPTY => "There are no documents in the document feeder.",
            WIA::WIA_ERROR_PAPER_PROBLEM => "An unspecified problem occurred with the scanner's document feeder.",
            WIA::WIA_ERROR_OFFLINE => "The device is offline. Make sure the device is powered on and connected to the PC.",
            WIA::WIA_ERROR_BUSY => "The device is busy. Close any apps that are using this device or wait for it to finish and then try again.",
            WIA::WIA_ERROR_WARMING_UP => "The device is warming up.",
            WIA::WIA_ERROR_USER_INTERVENTION => "There is a problem with the WIA device. Make sure that the device is turned on, online, and any cables are properly connected.",
            WIA::WIA_ERROR_ITEM_DELETED => "The WIA device was deleted. It's no longer available.",
            WIA::WIA_ERROR_DEVICE_COMMUNICATION => "Communication with the WIA device failed. Make sure that the device is powered on and connected to the PC. If the problem persists, disconnect and reconnect the device.",
            WIA::WIA_ERROR_INVALID_COMMAND => "The device doesn't support this command.",
            WIA::WIA_ERROR_INCORRECT_HARDWARE_SETTING => "There is an incorrect setting on the WIA device.",
            WIA::WIA_ERROR_DEVICE_LOCKED => "The device is locked. Close any apps that are using this device or wait for it to finish and then try again.",
            WIA::WIA_ERROR_EXCEPTION_IN_DRIVER => "The device driver threw an exception.",
            WIA::WIA_ERROR_INVALID_DRIVER_RESPONSE => "The response from the driver is invalid.",
            WIA::WIA_ERROR_COVER_OPEN => "One or more of the device's cover is open.",
            WIA::WIA_ERROR_LAMP_OFF => "The scanner's lamp is off.",
            WIA::WIA_ERROR_DESTINATION => "Description is missing",
            WIA::WIA_ERROR_NETWORK_RESERVATION_FAILED => "Description is missing",
            WIA::WIA_ERROR_MULTI_FEED => "A scan error occurred because of a multiple page feed condition. This feature is available with Windows 8 and later versions of Windows.",
            WIA::WIA_S_NO_DEVICE_AVAILABLE => "No scanner device was found. Make sure the device is online, connected to the PC, and has the correct driver installed on the PC.",
            WIA::WIA_ERROR_MAXIMUM_PRINTER_ENDORSER_COUNTER => "A scan job was interrupted because an Imprinter/Endorser item reached the maximum valid value for WIA_IPS_PRINTER_ENDORSER_COUNTER, and was reset to 0. This feature is available with Windows 8 and later versions of Windows.",
        }
    }
}
