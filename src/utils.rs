use crate::*;

pub struct HidAxis {
    kind: Usage,
    max_value: u32,
    min_value: u32,
    report_size: u32
}

impl HidAxis {
    pub fn new(kind: AbsAxis, max_value: u32, min_value: u32) -> HidAxis {
        assert!(max_value >= min_value, "Max value must be higher or equal to min value!");
        let kind = match kind {
            AbsAxis::X => Usage::X,
            AbsAxis::Y => Usage::Y,
            AbsAxis::Z => Usage::Z,
            AbsAxis::Rx => Usage::Rx,
            AbsAxis::Ry => Usage::Ry,
            AbsAxis::Rz => Usage::Rz
        };
        let report_size = if max_value > 0xffff {
            32
        } else if max_value > 0xff {
            16
        } else {
            8
        };
        HidAxis { kind, max_value, min_value, report_size }
    }
}

pub enum AbsAxis {
    X,
    Y,
    Z,
    Rx,
    Ry,
    Rz
}

pub struct HidGamepad {
    button_number: u32,
    axes: Vec<HidAxis>,
    hat: bool
}

impl HidGamepad {
    pub fn new(button_number: u32, axes: Vec<HidAxis>, hat: bool) -> HidGamepad {
        HidGamepad { button_number, axes, hat }
    }

    pub fn generate_report(&self) -> String {
        let button_number = self.button_number;
        let gamepad_axes = &self.axes;
    
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
        for axis in gamepad_axes {
            axes_collection.add_item(Item::new(ItemTag::Usage, Some(axis.kind as u32)));
            axes_collection.add_item(Item::new(ItemTag::LogicalMinimum, Some(axis.min_value)));
            axes_collection.add_item(Item::new(ItemTag::LogicalMaximum, Some(axis.max_value)));
            axes_collection.add_item(Item::new(ItemTag::ReportSize, Some(axis.report_size)));
            axes_collection.add_item(Item::new(ItemTag::ReportCount, Some(1)));
            axes_collection.add_item(Item::new(ItemTag::Input, Some((InputFlags::Data + InputFlags::Var + InputFlags::Abs + InputFlags::NoWrap + InputFlags::Linear + InputFlags::PreferredState + InputFlags::NoNullPosition) as u32)));
        }
        
        /*axes_collection.add_item(Item::new(ItemTag::Usage, Some(Usage::Y as u32)));
        axes_collection.add_item(Item::new(ItemTag::Usage, Some(Usage::Z as u32)));
        axes_collection.add_item(Item::new(ItemTag::Usage, Some(Usage::Rz as u32)));
        axes_collection.add_item(Item::new(ItemTag::LogicalMinimum, Some(0)));
        axes_collection.add_item(Item::new(ItemTag::LogicalMaximum, Some(0xffff)));
        axes_collection.add_item(Item::new(ItemTag::ReportSize, Some(16)));
        axes_collection.add_item(Item::new(ItemTag::ReportCount, Some(4)));
        axes_collection.add_item(Item::new(ItemTag::Input, Some((InputFlags::Data + InputFlags::Var + InputFlags::Abs + InputFlags::NoWrap + InputFlags::Linear + InputFlags::PreferredState + InputFlags::NoNullPosition) as u32)));*/
        
        collection_report.add_collection(axes_collection);
        if self.hat {
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
        }
        
        collection_app.add_collection(collection_report);
        hid_report.add_collection(collection_app);
    
        
        use itertools::Itertools;
        hid_report.get_bytes().iter().map(|x| format!("{:02X}", x).to_lowercase()).join("")
    }
}

