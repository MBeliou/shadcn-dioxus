pub fn get_component_doc(name: &str) -> Option<&'static str> {
    match name {
        "button" => Some(include_str!("../content/components/button.md")),
        "badge" => Some(include_str!("../content/components/badge.md")),
        "card" => Some(include_str!("../content/components/card.md")),
        "avatar" => Some(include_str!("../content/components/avatar.md")),
        "input" => Some(include_str!("../content/components/input.md")),
        "label" => Some(include_str!("../content/components/label.md")),
        "progress" => Some(include_str!("../content/components/progress.md")),
        "separator" => Some(include_str!("../content/components/separator.md")),
        "skeleton" => Some(include_str!("../content/components/skeleton.md")),
        "spinner" => Some(include_str!("../content/components/spinner.md")),
        "kbd" => Some(include_str!("../content/components/kbd.md")),
        "item" => Some(include_str!("../content/components/item.md")),
        "empty" => Some(include_str!("../content/components/empty.md")),
        "button-group" => Some(include_str!("../content/components/button-group.md")),
        "checkbox" => Some(include_str!("../content/components/checkbox.md")),
        "textarea" => Some(include_str!("../content/components/textarea.md")),
        "switch" => Some(include_str!("../content/components/switch.md")),
        "field" => Some(include_str!("../content/components/field.md")),
        "toggle" => Some(include_str!("../content/components/toggle.md")),
        "aspect-ratio" => Some(include_str!("../content/components/aspect-ratio.md")),
        "native-select" => Some(include_str!("../content/components/native-select.md")),
        "input-group" => Some(include_str!("../content/components/input-group.md")),
        "dialog" => Some(include_str!("../content/components/dialog.md")),
        "sheet" => Some(include_str!("../content/components/sheet.md")),
        "alert" => Some(include_str!("../content/components/alert.md")),

        "alert_dialog" => Some(include_str!("../content/components/alert_dialog.md")),

        "accordion" => Some(include_str!("../content/components/accordion.md")),

        "breadcrumb" => Some(include_str!("../content/components/breadcrumb.md")),

        "calendar" => Some(include_str!("../content/components/calendar.md")),

        "carousel" => Some(include_str!("../content/components/carousel.md")),

        "chart" => Some(include_str!("../content/components/chart.md")),

        "collapsible" => Some(include_str!("../content/components/collapsible.md")),

        "color_picker" => Some(include_str!("../content/components/color_picker.md")),

        "combobox" => Some(include_str!("../content/components/combobox.md")),

        "command" => Some(include_str!("../content/components/command.md")),

        "context_menu" => Some(include_str!("../content/components/context_menu.md")),

        "data_table" => Some(include_str!("../content/components/data_table.md")),

        "date_picker" => Some(include_str!("../content/components/date_picker.md")),

        "drawer" => Some(include_str!("../content/components/drawer.md")),

        "dropdown_menu" => Some(include_str!("../content/components/dropdown_menu.md")),

        "hover_card" => Some(include_str!("../content/components/hover_card.md")),

        "input_otp" => Some(include_str!("../content/components/input_otp.md")),

        "menubar" => Some(include_str!("../content/components/menubar.md")),

        "navigation_menu" => Some(include_str!("../content/components/navigation_menu.md")),

        "pagination" => Some(include_str!("../content/components/pagination.md")),

        "popover" => Some(include_str!("../content/components/popover.md")),

        "radio_group" => Some(include_str!("../content/components/radio_group.md")),

        "resizable" => Some(include_str!("../content/components/resizable.md")),

        "scroll_area" => Some(include_str!("../content/components/scroll_area.md")),

        "select" => Some(include_str!("../content/components/select.md")),

        "sidebar" => Some(include_str!("../content/components/sidebar.md")),

        "slider" => Some(include_str!("../content/components/slider.md")),

        "stepper" => Some(include_str!("../content/components/stepper.md")),

        "table" => Some(include_str!("../content/components/table.md")),

        "tabs" => Some(include_str!("../content/components/tabs.md")),

        "toggle_group" => Some(include_str!("../content/components/toggle_group.md")),

        "tooltip" => Some(include_str!("../content/components/tooltip.md")),

        _ => None,
    }
}
