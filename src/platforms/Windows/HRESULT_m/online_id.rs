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

pub enum ONLINE_ID {
    ONL_E_INVALID_AUTHENTICATION_TARGET = 0x80860001,
    ONL_E_ACCESS_DENIED_BY_TOU = 0x80860002,
    ONL_E_INVALID_APPLICATION = 0x80860003,
    ONL_E_PASSWORD_UPDATE_REQUIRED = 0x80860004,
    ONL_E_ACCOUNT_UPDATE_REQUIRED = 0x80860005,
    ONL_E_FORCESIGNIN = 0x80860006,
    ONL_E_ACCOUNT_LOCKED = 0x80860007,
    ONL_E_PARENTAL_CONSENT_REQUIRED = 0x80860008,
    ONL_E_EMAIL_VERIFICATION_REQUIRED = 0x80860009,
    ONL_E_ACCOUNT_SUSPENDED_COMPROIMISE = 0x8086000A,
    ONL_E_ACCOUNT_SUSPENDED_ABUSE = 0x8086000B,
    ONL_E_ACTION_REQUIRED = 0x8086000C,
    ONL_CONNECTION_COUNT_LIMIT = 0x8086000D,
    ONL_E_CONNECTED_ACCOUNT_CAN_NOT_SIGNOUT = 0x8086000E,
    ONL_E_USER_AUTHENTICATION_REQUIRED = 0x8086000F,
    ONL_E_REQUEST_THROTTLED = 0x80860010,
}

impl ONLINE_ID {
    pub fn description(&self) -> &'static str {
        match self {
            ONLINE_ID::ONL_E_INVALID_AUTHENTICATION_TARGET => "Authentication target is invalid or not configured correctly.",
            ONLINE_ID::ONL_E_ACCESS_DENIED_BY_TOU => "Your application cannot get the Online Id properties due to the Terms of Use accepted by the user.",
            ONLINE_ID::ONL_E_INVALID_APPLICATION => "The application requesting authentication tokens is either disabled or incorrectly configured.",
            ONLINE_ID::ONL_E_PASSWORD_UPDATE_REQUIRED => "Online Id password must be updated before signin.",
            ONLINE_ID::ONL_E_ACCOUNT_UPDATE_REQUIRED => "Online Id account properties must be updated before signin.",
            ONLINE_ID::ONL_E_FORCESIGNIN => "To help protect your Online Id account you must signin again.",
            ONLINE_ID::ONL_E_ACCOUNT_LOCKED => "Online Id account was locked because there have been too many attempts to sign in.",
            ONLINE_ID::ONL_E_PARENTAL_CONSENT_REQUIRED => "Online Id account requires parental consent before proceeding.",
            ONLINE_ID::ONL_E_EMAIL_VERIFICATION_REQUIRED => "Online Id signin name is not yet verified. Email verification is required before signin.",
            ONLINE_ID::ONL_E_ACCOUNT_SUSPENDED_COMPROIMISE => "We have noticed some unusual activity in your Online Id account. Your action is needed to make sure no one else is using your account.",
            ONLINE_ID::ONL_E_ACCOUNT_SUSPENDED_ABUSE => "We detected some suspicious activity with your Online Id account. To help protect you, we've temporarily blocked your account.",
            ONLINE_ID::ONL_E_ACTION_REQUIRED => "User interaction is required for authentication.",
            ONLINE_ID::ONL_CONNECTION_COUNT_LIMIT => "User has reached the maximum device associations per user limit.",
            ONLINE_ID::ONL_E_CONNECTED_ACCOUNT_CAN_NOT_SIGNOUT => "Cannot sign out from the application since the user account is connected.",
            ONLINE_ID::ONL_E_USER_AUTHENTICATION_REQUIRED => "User authentication is required for this operation.",
            ONLINE_ID::ONL_E_REQUEST_THROTTLED => "We want to make sure this is you. User interaction is required for authentication.",
        }
    }
}
