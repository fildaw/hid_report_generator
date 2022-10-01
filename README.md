# hid_report_generator

Utility crate to generate USB HID descriptor (https://www.usb.org/sites/default/files/hid1_11.pdf). 
Such descriptor is used also in Bluetooth BR/EDR HID SDP record (my private use case).

# Documentation
At the moment it doesn't exist. To see how to use this crate, look in the code in unit tests in `lib.rs`<br>
File `utils.rs` has helper functions to generate HID report of a gamepad.

# Copyright
&copy; 2022 fildaw<br><br>
Based on: https://github.com/GamesCreatorsClub/GCC-Joystick/blob/master/src/python/bt_joystick/hid_report_descriptor.py<br>
Written in Rust!<br>
My first published Rust crate, any advice on improving the code is highly appreciated, thank you!
