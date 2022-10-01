#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod utils;

pub mod InputFlags {
    pub const Data: u8 = 0x00;
    pub const Const: u8 = 0x01;

    pub const Array: u8 = 0x00;
    pub const Var: u8 = 0x02;

    pub const Abs: u8 = 0x00;
    pub const Rel: u8 = 0x04;

    pub const NoWrap: u8 = 0x00;
    pub const Wrap: u8 = 0x08;

    pub const Linear: u8 = 0x00;
    pub const Nonlinear: u8 = 0x10;

    pub const PreferredState: u8 = 0x00;
    pub const NoPreferredState: u8 = 0x20;

    pub const NoNullPosition: u8 = 0x00;
    pub const NullState: u8 = 0x40;
}

#[derive(Copy, Clone)]
enum Usage {
    Pointer = 0x01,
    Mouse = 0x02,
    Value_0x03 = 0x03,
    Joystick = 0x04,
    Gamepad = 0x05,
    Keyboard = 0x06,
    Keypad = 0x07,
    MultiAxisController = 0x08,
    X = 0x30,
    Y = 0x31,
    Z = 0x32,
    Rx = 0x33,
    Ry = 0x34,
    Rz = 0x35,
    Slider = 0x36,
    Dial = 0x37,
    Wheel = 0x38,
    HatSwitch = 0x39,
    CountedBuffer = 0x3A,
    ByteCount = 0x3B,
    MotionWakeup = 0x3C,
    Start = 0x3D,
    Vx = 0x40,
    Vy = 0x41,
    Vz = 0x42,
    Vbrx = 0x43,
    Vbry = 0x44,
    Vbrz = 0x45,
    Vno = 0x046,
    FeatureNotification = 0x47,
    SysControl = 0x80,
    SysPowerDown = 0x81,
    SysSleep = 0x82,
    SysWakeUp = 0x83,
    SysContextMenu = 0x84,
    SysMainMenu = 0x85,
    SysAppMenu = 0x86,
    SysMenuHelp = 0x87,
    SysMenuExit = 0x88,
    SysMenuSelect = 0x89,
    SysMenuRight = 0x8A,
    SysMenuLeft = 0x8B,
    SysMenuUp = 0x8C,
    SysMenuDown = 0x8D,
    SysColdRestart = 0x8E,
    SysWarmRestart = 0x8F,
    DPadUp = 0x90,
    DPadDown = 0x91,
    DPadRight = 0x92,
    DPadLeft = 0x93,
    SysDock = 0xA0,
    SysUndock = 0xA1,
    SysSetup = 0xA2,
    SysBreak = 0xA3,
    SysDebuggerBreak = 0xA4,
    ApplicationBreak = 0xA5,
    ApplicationDebuggerBreak = 0xA6,
    SysSpeakerMute = 0xA7,
    SysHibernate = 0xA8,
    SysDisplayInvert = 0xB0,
    SysDisplayInternal = 0xB1,
    SysDisplayExternal = 0xB2,
    SysDisplayBoth = 0xB3,
    SysDisplayDual = 0xB4,
    SysDisplayToggleIntExt = 0xB5,
    SysDisplaySwap = 0xB6,
    SysDisplayLCDAutoscale = 0xB7
}

enum ReportID {
    InputReport = 0x01,
    OutputReport = 0x02,
    FeatureReport = 0x03
}

enum Unit {
    EnglishRotationDegrees = 0x14,
    CM = 0x11,
    SIRad = 0x21
}
enum UsagePage {
    GenericDesktopCtrls = 0x01,
    SimCtrl = 0x02,
    VRCtrls = 0x03,
    SportCtrls = 0x04,
    GameCtrls = 0x05,
    GenericDevCtrls = 0x06,
    KeyboardKeypad = 0x07,
    LEDS = 0x08,
    Button = 0x09,
    Ordinal = 0x0A,
    Telephony = 0x0B,
    Consumer = 0x0C,
    Digitizer = 0x0D,
    PIDPage = 0x0F,
    Unicode = 0x10,
    AlphanumericDisplay = 0x14
}

#[derive(Clone, Copy)]
enum ItemTag {
    UsagePage = 0x04,
    Usage = 0x08,
    Collection = 0xA0,
    UsageMinimum = 0x18,
    UsageMaximum = 0x28,
    LogicalMinimum = 0x14,
    LogicalMaximum = 0x24,
    PhysicalMinimum = 0x34,
    PhysicalMaximum = 0x44,
    Unit = 0x64,
    ReportSize = 0x74,
    Input = 0x80,
    ReportID = 0x84,
    ReportCount = 0x94
}

struct Item {
    _tag: ItemTag,
    data: Vec<u8>
}

// Implementing only Short Items https://www.usb.org/sites/default/files/hid1_11.pdf (section 6.2.2.1)
impl Item {
    fn get_bytes(&self) -> &Vec<u8> {
        &self.data
    }

    fn new(tag: ItemTag, data: Option<u32>) -> Self {
        let mut header: u8 = tag as u8;
        let mut data_vec: Vec<u8> = Vec::new();
        if data.is_some() {
            let data = data.unwrap();
            if data > 65535 {
                header += 3;
                data_vec.push(header);
                data_vec.push((data & 255) as u8);
                data_vec.push(((data >> 8) & 255) as u8);
                data_vec.push(((data >> 16) & 255) as u8);
                data_vec.push(((data >> 24) & 255) as u8);
            } else if data > 255 {
                header += 2;
                data_vec.push(header);
                data_vec.push((data & 255) as u8);
                data_vec.push((data >> 8) as u8);
            } else {
                header += 1;
                data_vec.push(header);
                data_vec.push((data) as u8);
            }  
        }
        Item { _tag: tag, data: data_vec }
    }
}

enum CollectionAttr {
    Physical = 0x00,
    Application = 0x01,
    Value_0x02 = 0x02,
    Report = 0x03
}

struct Collection {
   byte_data: Vec<u8>,
}

impl Collection {
    fn new(c_type: CollectionAttr) -> Self {
        let mut bytes = Vec::new();
        let collecion_item = Item::new(ItemTag::Collection, Some(c_type as u32));
        for b in collecion_item.get_bytes() {
            bytes.push(*b);
        }
        Collection { byte_data: bytes, }
    }

    fn add_item(&mut self, item: Item) {
        for b in item.get_bytes() {
            self.byte_data.push(*b);
        }
    }

    fn add_collection(&mut self, collection: Collection) {
        let bytes = collection.finish_and_get_bytes();
        for b in bytes {
            self.byte_data.push(b);
        }
    }

    fn get_bytes(&self) -> &Vec<u8> {
        &self.byte_data
    }

    fn finish_and_get_bytes(mut self) -> Vec<u8> {
        self.byte_data.push(0xC0);
        self.byte_data
    }
}


struct HIDReport {
    byte_data: Vec<u8>
}

impl HIDReport {
    fn new() -> Self {
        HIDReport { byte_data: Vec::new() }
    }

    fn add_collection(&mut self, collection: Collection) {
        let bytes = collection.finish_and_get_bytes();
        for b in bytes {
            self.byte_data.push(b);
        }
    }

    fn add_item(&mut self, item: Item) {
        for b in item.get_bytes() {
            self.byte_data.push(*b);
        }
    }

    fn get_bytes(&self) -> &Vec<u8> {
        &self.byte_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_usage_page_desktop() {
        let mut report = HIDReport::new();
        report.add_item(Item::new(ItemTag::UsagePage, Some(UsagePage::GenericDesktopCtrls as u32)));
        assert_eq!(report.byte_data, vec![0x05, 0x01]);
    }

    #[test]
    fn test_usage_page_button() {
        let mut report = HIDReport::new();
        report.add_item(Item::new(ItemTag::UsagePage, Some(UsagePage::Button as u32)));
        assert_eq!(report.byte_data, vec![0x05, 0x09]);
    }

    #[test]
    fn test_logical_maximum() {
        let mut report = HIDReport::new();
        report.add_item(Item::new(ItemTag::LogicalMaximum, Some(32767)));
        assert_eq!(report.byte_data, vec![0x26, 0xff, 0x7f]);
    }

    #[test]
    fn test_hid_report() {
        let button_number = 12;

        let mut hid_report = HIDReport::new();
        hid_report.add_item(Item::new(ItemTag::UsagePage, Some(UsagePage::GenericDesktopCtrls as u32)));
        hid_report.add_item(Item::new(ItemTag::Usage, Some(Usage::Gamepad as u32)));

        let mut collection_app = Collection::new(CollectionAttr::Application);

        let mut collection_report = Collection::new(CollectionAttr::Report);
        collection_report.add_item(Item::new(ItemTag::ReportID, Some(ReportID::InputReport as u32)));
        collection_report.add_item(Item::new(ItemTag::UsagePage, Some(UsagePage::Button as u32)));
        collection_report.add_item(Item::new(ItemTag::UsageMinimum, Some(1)));
        collection_report.add_item(Item::new(ItemTag::UsageMaximum, Some(button_number)));
        collection_report.add_item(Item::new(ItemTag::LogicalMinimum, Some(0)));
        collection_report.add_item(Item::new(ItemTag::LogicalMaximum, Some(1)));
        collection_report.add_item(Item::new(ItemTag::ReportCount, Some(button_number)));
        collection_report.add_item(Item::new(ItemTag::ReportSize, Some(1)));
        collection_report.add_item(Item::new(ItemTag::Input, Some((InputFlags::Data + InputFlags::Var + InputFlags::Abs + InputFlags::NoWrap + InputFlags::Linear + InputFlags::PreferredState + InputFlags::NoNullPosition) as u32)));
        if button_number % 8 != 0 {
            collection_report.add_item(Item::new(ItemTag::ReportCount, Some(1)));
            collection_report.add_item(Item::new(ItemTag::ReportSize, Some(8-button_number % 8)));
            collection_report.add_item(Item::new(ItemTag::Input, Some((InputFlags::Const + InputFlags::Var + InputFlags::Abs + InputFlags::NoWrap + InputFlags::Linear + InputFlags::PreferredState + InputFlags::NoNullPosition) as u32)));
        }

        let mut axes_collection = Collection::new(CollectionAttr::Physical);
        axes_collection.add_item(Item::new(ItemTag::UsagePage, Some(UsagePage::GenericDesktopCtrls as u32)));
        axes_collection.add_item(Item::new(ItemTag::Usage, Some(Usage::X as u32)));
        axes_collection.add_item(Item::new(ItemTag::Usage, Some(Usage::Y as u32)));
        axes_collection.add_item(Item::new(ItemTag::Usage, Some(Usage::Z as u32)));
        axes_collection.add_item(Item::new(ItemTag::Usage, Some(Usage::Rz as u32)));
        axes_collection.add_item(Item::new(ItemTag::LogicalMinimum, Some(0)));
        axes_collection.add_item(Item::new(ItemTag::LogicalMaximum, Some(0xffff)));
        axes_collection.add_item(Item::new(ItemTag::ReportSize, Some(16)));
        axes_collection.add_item(Item::new(ItemTag::ReportCount, Some(4)));
        axes_collection.add_item(Item::new(ItemTag::Input, Some((InputFlags::Data + InputFlags::Var + InputFlags::Abs + InputFlags::NoWrap + InputFlags::Linear + InputFlags::PreferredState + InputFlags::NoNullPosition) as u32)));
        
        collection_report.add_collection(axes_collection);

        let mut hat_collection = Collection::new(CollectionAttr::Physical);
        hat_collection.add_item(Item::new(ItemTag::UsagePage, Some(UsagePage::GenericDesktopCtrls as u32)));
        hat_collection.add_item(Item::new(ItemTag::Usage, Some(Usage::HatSwitch as u32)));
        hat_collection.add_item(Item::new(ItemTag::LogicalMinimum, Some(0)));
        hat_collection.add_item(Item::new(ItemTag::LogicalMaximum, Some(8)));
        hat_collection.add_item(Item::new(ItemTag::PhysicalMinimum, Some(0)));
        hat_collection.add_item(Item::new(ItemTag::PhysicalMaximum, Some(315)));
        hat_collection.add_item(Item::new(ItemTag::Unit, Some(Unit::EnglishRotationDegrees as u32)));
        hat_collection.add_item(Item::new(ItemTag::ReportSize, Some(4)));
        hat_collection.add_item(Item::new(ItemTag::ReportCount, Some(1)));
        hat_collection.add_item(Item::new(ItemTag::Input, Some((InputFlags::Var + InputFlags::Abs + InputFlags::NullState) as u32)));
        hat_collection.add_item(Item::new(ItemTag::ReportCount, Some(1)));
        hat_collection.add_item(Item::new(ItemTag::ReportSize, Some(4)));
        
        hat_collection.add_item(Item::new(ItemTag::Input, Some((InputFlags::Const + InputFlags::Var + InputFlags::Abs + InputFlags::NoWrap + InputFlags::Linear + InputFlags::PreferredState + InputFlags::NoNullPosition) as u32)));

        collection_report.add_collection(hat_collection);
        collection_app.add_collection(collection_report);
        hid_report.add_collection(collection_app);

        
        use itertools::Itertools;
        let hid = hid_report.get_bytes().iter().map(|x| format!("{:02X}", x).to_lowercase()).join("");
        assert_eq!(hid, "05010905a101a103850105091901290c15002501950c75018102950175048103a10005010930093109320935150026ffff751095048102c0a10005010939150025083500463b016514750495018142950175048103c0c0c0");
    }
}