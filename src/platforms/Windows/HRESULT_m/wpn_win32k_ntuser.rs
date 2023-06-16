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

pub enum WPN_WIN32K_NTUSER {
    WPN_E_CHANNEL_CLOSED = 0x803E0100,
    WPN_E_CHANNEL_REQUEST_NOT_COMPLETE = 0x803E0101,
    WPN_E_INVALID_APP = 0x803E0102,
    WPN_E_OUTSTANDING_CHANNEL_REQUEST = 0x803E0103,
    WPN_E_DUPLICATE_CHANNEL = 0x803E0104,
    WPN_E_PLATFORM_UNAVAILABLE = 0x803E0105,
    WPN_E_NOTIFICATION_POSTED = 0x803E0106,
    WPN_E_NOTIFICATION_HIDDEN = 0x803E0107,
    WPN_E_NOTIFICATION_NOT_POSTED = 0x803E0108,
    WPN_E_CLOUD_DISABLED = 0x803E0109,
    WPN_E_CLOUD_INCAPABLE = 0x803E0110,
    WPN_E_NOTIFICATION_DISABLED = 0x803E0111,
    WPN_E_NOTIFICATION_INCAPABLE = 0x803E0112,
    WPN_E_INTERNET_INCAPABLE = 0x803E0113,
    WPN_E_NOTIFICATION_TYPE_DISABLED = 0x803E0114,
    WPN_E_NOTIFICATION_SIZE = 0x803E0115,
    WPN_E_TAG_SIZE = 0x803E0116,
    WPN_E_ACCESS_DENIED = 0x803E0117,
    WPN_E_DUPLICATE_REGISTRATION = 0x803E0118,
    WPN_E_PUSH_NOTIFICATION_INCAPABLE = 0x803E0119,
    WPN_E_CLOUD_AUTH_UNAVAILABLE = 0x803E011A,
    WPN_E_CLOUD_SERVICE_UNAVAILABLE = 0x803E011B,
    WPN_E_FAILED_LOCK_SCREEN_UPDATE_INTIALIZATION = 0x803E011C,
    WPN_E_DEV_ID_SIZE = 0x803E0120,
    WPN_E_TAG_ALPHANUMERIC = 0x803E012A,
    WPN_E_INVALID_HTTP_STATUS_CODE = 0x803E012B,
    WPN_E_OUT_OF_SESSION = 0x803E0200,
    WPN_E_POWER_SAVE = 0x803E0201,
    WPN_E_IMAGE_NOT_FOUND_IN_CACHE = 0x803E0202,
    WPN_E_ALL_URL_NOT_COMPLETED = 0x803E0203,
    WPN_E_INVALID_CLOUD_IMAGE = 0x803E0204,
    WPN_E_NOTIFICATION_ID_MATCHED = 0x803E0205,
    WPN_E_CALLBACK_ALREADY_REGISTERED = 0x803E0206,
    WPN_E_TOAST_NOTIFICATION_DROPPED = 0x803E0207,
    WPN_E_STORAGE_LOCKED = 0x803E0208,
    WPN_E_GROUP_SIZE = 0x803E0209,
    WPN_E_GROUP_ALPHANUMERIC = 0x803E020A,
    WPN_E_CLOUD_DISABLED_FOR_APP = 0x803E020B,
}

impl WPN_WIN32K_NTUSER {
    pub fn description(&self) -> &'static str {
        match self {
            WPN_WIN32K_NTUSER::WPN_E_CHANNEL_CLOSED => "The notification channel has already been closed.",
            WPN_WIN32K_NTUSER::WPN_E_CHANNEL_REQUEST_NOT_COMPLETE => "The notification channel request did not complete successfully.",
            WPN_WIN32K_NTUSER::WPN_E_INVALID_APP => "The application identifier provided is invalid.",
            WPN_WIN32K_NTUSER::WPN_E_OUTSTANDING_CHANNEL_REQUEST => "A notification channel request for the provided application identifier is in progress.",
            WPN_WIN32K_NTUSER::WPN_E_DUPLICATE_CHANNEL => "The channel identifier is already tied to another application endpoint.",
            WPN_WIN32K_NTUSER::WPN_E_PLATFORM_UNAVAILABLE => "The notification platform is unavailable.",
            WPN_WIN32K_NTUSER::WPN_E_NOTIFICATION_POSTED => "The notification has already been posted.",
            WPN_WIN32K_NTUSER::WPN_E_NOTIFICATION_HIDDEN => "The notification has already been hidden.",
            WPN_WIN32K_NTUSER::WPN_E_NOTIFICATION_NOT_POSTED => "The notification cannot be hidden until it has been shown.",
            WPN_WIN32K_NTUSER::WPN_E_CLOUD_DISABLED => "Cloud notifications have been turned off.",
            WPN_WIN32K_NTUSER::WPN_E_CLOUD_INCAPABLE => "The application does not have the cloud notification capability.",
            WPN_WIN32K_NTUSER::WPN_E_NOTIFICATION_DISABLED => "Settings prevent the notification from being delivered.",
            WPN_WIN32K_NTUSER::WPN_E_NOTIFICATION_INCAPABLE => "Application capabilities prevent the notification from being delivered.",
            WPN_WIN32K_NTUSER::WPN_E_INTERNET_INCAPABLE => "The application does not have the internet access capability.",
            WPN_WIN32K_NTUSER::WPN_E_NOTIFICATION_TYPE_DISABLED => "Settings prevent the notification type from being delivered.",
            WPN_WIN32K_NTUSER::WPN_E_NOTIFICATION_SIZE => "The size of the notification content is too large.",
            WPN_WIN32K_NTUSER::WPN_E_TAG_SIZE => "The size of the notification tag is too large.",
            WPN_WIN32K_NTUSER::WPN_E_ACCESS_DENIED => "The notification platform doesn't have appropriate privilege on resources.",
            WPN_WIN32K_NTUSER::WPN_E_DUPLICATE_REGISTRATION => "The notification platform found application is already registered.",
            WPN_WIN32K_NTUSER::WPN_E_PUSH_NOTIFICATION_INCAPABLE => "The application background task does not have the push notification capability.",
            WPN_WIN32K_NTUSER::WPN_E_CLOUD_AUTH_UNAVAILABLE => "The notification platform is unable to retrieve the authentication credentials required to connect to the cloud notification service.",
            WPN_WIN32K_NTUSER::WPN_E_CLOUD_SERVICE_UNAVAILABLE => "The notification platform is unable to connect to the cloud notification service.",
            WPN_WIN32K_NTUSER::WPN_E_FAILED_LOCK_SCREEN_UPDATE_INTIALIZATION => "The notification platform is unable to initialize a callback for lock screen updates.",
            WPN_WIN32K_NTUSER::WPN_E_DEV_ID_SIZE => "The size of the developer id for scheduled notification is too large.",
            WPN_WIN32K_NTUSER::WPN_E_TAG_ALPHANUMERIC => "The notification tag is not alphanumeric.",
            WPN_WIN32K_NTUSER::WPN_E_INVALID_HTTP_STATUS_CODE => "The notification platform has received invalid HTTP status code other than 2xx for polling.",
            WPN_WIN32K_NTUSER::WPN_E_OUT_OF_SESSION => "The notification platform has run out of presentation layer sessions.",
            WPN_WIN32K_NTUSER::WPN_E_POWER_SAVE => "The notification platform rejects image download request due to system in power save mode.",
            WPN_WIN32K_NTUSER::WPN_E_IMAGE_NOT_FOUND_IN_CACHE => "The notification platform doesn't have the requested image in its cache.",
            WPN_WIN32K_NTUSER::WPN_E_ALL_URL_NOT_COMPLETED => "The notification platform cannot complete all of requested image.",
            WPN_WIN32K_NTUSER::WPN_E_INVALID_CLOUD_IMAGE => "A cloud image downloaded from the notification platform is invalid.",
            WPN_WIN32K_NTUSER::WPN_E_NOTIFICATION_ID_MATCHED => "Notification Id provided as filter is matched with what the notification platform maintains.",
            WPN_WIN32K_NTUSER::WPN_E_CALLBACK_ALREADY_REGISTERED => "Notification callback interface is already registered.",
            WPN_WIN32K_NTUSER::WPN_E_TOAST_NOTIFICATION_DROPPED => "Toast Notification was dropped without being displayed to the user.",
            WPN_WIN32K_NTUSER::WPN_E_STORAGE_LOCKED => "The notification platform does not have the proper privileges to complete the request.",
            WPN_WIN32K_NTUSER::WPN_E_GROUP_SIZE => "The size of the notification group is too large.",
            WPN_WIN32K_NTUSER::WPN_E_GROUP_ALPHANUMERIC => "The notification group is not alphanumeric.",
            WPN_WIN32K_NTUSER::WPN_E_CLOUD_DISABLED_FOR_APP => "Cloud notifications have been disabled for the application due to a policy setting.",
        }
    }
}
