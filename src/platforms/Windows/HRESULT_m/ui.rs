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

pub enum UI {
    UI_E_CREATE_FAILED = 0x802A0001,
    E_WPD_DEVICE_ALREADY_OPENED = 0x802A0001,
    UI_E_SHUTDOWN_CALLED = 0x802A0002,
    E_WPD_DEVICE_NOT_OPEN = 0x802A0002,
    UI_E_ILLEGAL_REENTRANCY = 0x802A0003,
    E_WPD_OBJECT_ALREADY_ATTACHED_TO_DEVICE = 0x802A0003,
    UI_E_OBJECT_SEALED = 0x802A0004,
    E_WPD_OBJECT_NOT_ATTACHED_TO_DEVICE = 0x802A0004,
    UI_E_VALUE_NOT_SET = 0x802A0005,
    E_WPD_OBJECT_NOT_COMMITED = 0x802A0005,
    UI_E_VALUE_NOT_DETERMINED = 0x802A0006,
    E_WPD_DEVICE_IS_HUNG = 0x802A0006,
    UI_E_INVALID_OUTPUT = 0x802A0007,
    UI_E_BOOLEAN_EXPECTED = 0x802A0008,
    UI_E_DIFFERENT_OWNER = 0x802A0009,
    UI_E_AMBIGUOUS_MATCH = 0x802A000A,
    UI_E_FP_OVERFLOW = 0x802A000B,
    UI_E_WRONG_THREAD = 0x802A000C,
    E_WPD_SMS_INVALID_RECIPIENT = 0x802A0064,
    E_WPD_SMS_INVALID_MESSAGE_BODY = 0x802A0065,
    E_WPD_SMS_SERVICE_UNAVAILABLE = 0x802A0066,
    E_WPD_SERVICE_ALREADY_OPENED = 0x802A00C8,
    E_WPD_SERVICE_NOT_OPEN = 0x802A00C9,
    E_WPD_OBJECT_ALREADY_ATTACHED_TO_SERVICE = 0x802A00CA,
    E_WPD_OBJECT_NOT_ATTACHED_TO_SERVICE = 0x802A00CB,
    E_WPD_SERVICE_BAD_PARAMETER_ORDER = 0x802A00CC,
    UI_E_STORYBOARD_ACTIVE = 0x802A0101,
    UI_E_STORYBOARD_NOT_PLAYING = 0x802A0102,
    UI_E_START_KEYFRAME_AFTER_END = 0x802A0103,
    UI_E_END_KEYFRAME_NOT_DETERMINED = 0x802A0104,
    UI_E_LOOPS_OVERLAP = 0x802A0105,
    UI_E_TRANSITION_ALREADY_USED = 0x802A0106,
    UI_E_TRANSITION_NOT_IN_STORYBOARD = 0x802A0107,
    UI_E_TRANSITION_ECLIPSED = 0x802A0108,
    UI_E_TIME_BEFORE_LAST_UPDATE = 0x802A0109,
    UI_E_TIMER_CLIENT_ALREADY_CONNECTED = 0x802A010A,
    UI_E_INVALID_DIMENSION = 0x802A010B,
    UI_E_PRIMITIVE_OUT_OF_BOUNDS = 0x802A010C,
    UI_E_WINDOW_CLOSED = 0x802A0201,
}

impl UI {
    pub fn description(&self) -> &'static str {
        match self {
            UI::UI_E_CREATE_FAILED => "The object could not be created.",
            UI::E_WPD_DEVICE_ALREADY_OPENED => "The device connection has already been opened by a prior call to IPortableDevice::Open. See the WPD SDK for a description of this method.",
            UI::UI_E_SHUTDOWN_CALLED => "Shutdown was already called on this object or the object that owns it.",
            UI::E_WPD_DEVICE_NOT_OPEN => "The device connection has not yet been opened by a call to IPortableDevice::Open.",
            UI::UI_E_ILLEGAL_REENTRANCY => "This method cannot be called during this type of callback.",
            UI::E_WPD_OBJECT_ALREADY_ATTACHED_TO_DEVICE => "The interface object has already been attached to the device interface.",
            UI::UI_E_OBJECT_SEALED => "This object has been sealed, so this change is no longer allowed.",
            UI::E_WPD_OBJECT_NOT_ATTACHED_TO_DEVICE => "The interface object has not been attached the device.",
            UI::UI_E_VALUE_NOT_SET => "The requested value was never set.",
            UI::E_WPD_OBJECT_NOT_COMMITED => "IStream::Commit was never called when creating an object with data on a device.",
            UI::UI_E_VALUE_NOT_DETERMINED => "The requested value cannot be determined.",
            UI::E_WPD_DEVICE_IS_HUNG => "The device will no longer respond to input.",
            UI::UI_E_INVALID_OUTPUT => "A callback returned an invalid output parameter.",
            UI::UI_E_BOOLEAN_EXPECTED => "A callback returned a success code other than S_OK or S_FALSE.",
            UI::UI_E_DIFFERENT_OWNER => "A parameter that should be owned by this object is owned by a different object.",
            UI::UI_E_AMBIGUOUS_MATCH => "More than one item matched the search criteria.",
            UI::UI_E_FP_OVERFLOW => "A floating-point overflow occurred.",
            UI::UI_E_WRONG_THREAD => "This method can only be called from the thread that created the object.",
            UI::E_WPD_SMS_INVALID_RECIPIENT => "The recipient specified for a short message service (SMS) message is invalid.",
            UI::E_WPD_SMS_INVALID_MESSAGE_BODY => "The body of a message specified for an SMS message is invalid.",
            UI::E_WPD_SMS_SERVICE_UNAVAILABLE => "The SMS service is not available.",
            UI::E_WPD_SERVICE_ALREADY_OPENED => "The service connection has already been opened by a prior call to IPortableDevice::Open.",
            UI::E_WPD_SERVICE_NOT_OPEN => "The service connection has not yet been opened by a call to IPortableDeviceService::Open.",
            UI::E_WPD_OBJECT_ALREADY_ATTACHED_TO_SERVICE => "The interface object has already been attached to the IPortableDeviceService interface.",
            UI::E_WPD_OBJECT_NOT_ATTACHED_TO_SERVICE => "The interface object has not been attached to the IPortableDeviceService interface. Typically, this is returned if the application tries to access methods of an attached interface, such as IPortableDeviceServiceCapabilities, after IPortableDevice::Close is called.",
            UI::E_WPD_SERVICE_BAD_PARAMETER_ORDER => "The method parameters for IPortableDeviceServiceMethods::Invoke or IPortableDeviceServiceMethods::InvokeAsync are not set in the correct order. The parameter must be set in the ordering specified by WPD_PARAMETER_ATTRIBUTE_ORDER.",
            UI::UI_E_STORYBOARD_ACTIVE => "The storyboard is currently in the schedule.",
            UI::UI_E_STORYBOARD_NOT_PLAYING => "The storyboard is not playing.",
            UI::UI_E_START_KEYFRAME_AFTER_END => "The start keyframe might occur after the end keyframe.",
            UI::UI_E_END_KEYFRAME_NOT_DETERMINED => "It might not be possible to determine the end keyframe time when the start keyframe is reached.",
            UI::UI_E_LOOPS_OVERLAP => "Two repeated portions of a storyboard might overlap.",
            UI::UI_E_TRANSITION_ALREADY_USED => "The transition has already been added to a storyboard.",
            UI::UI_E_TRANSITION_NOT_IN_STORYBOARD => "The transition has not been added to a storyboard.",
            UI::UI_E_TRANSITION_ECLIPSED => "The transition might eclipse the beginning of another transition in the storyboard.",
            UI::UI_E_TIME_BEFORE_LAST_UPDATE => "The given time is earlier than the time passed to the last update.",
            UI::UI_E_TIMER_CLIENT_ALREADY_CONNECTED => "This client is already connected to a timer.",
            UI::UI_E_INVALID_DIMENSION => "The passed dimension is invalid or does not match the object's dimension.",
            UI::UI_E_PRIMITIVE_OUT_OF_BOUNDS => "The added primitive begins at or beyond the duration of the interpolator.",
            UI::UI_E_WINDOW_CLOSED => "The operation cannot be completed because the window is being closed.",
        }
    }
}
