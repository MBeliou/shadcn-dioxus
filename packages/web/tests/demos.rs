use web::demos::get_demo;

#[test]
fn test_all_demo_names_return_element() {
    // List all known demo names; every demo must return Some(element)
    let demos = vec![
        "button-demo", "button-primary", "button-secondary", "button-destructive",
        "button-outline", "button-ghost", "button-link",
        "badge-demo", "badge-secondary", "badge-destructive", "badge-outline",
        "card-demo", "avatar-demo", "avatar-fallback",
        "input-demo", "input-disabled", "label-demo",
        "progress-demo", "progress-indeterminate",
        "separator-demo", "separator-vertical",
        "skeleton-demo", "spinner-demo",
        "kbd-demo", "kbd-group", "kbd-button",
        "item-demo", "empty-demo", "button-group-demo",
        "checkbox-demo", "checkbox-with-label", "checkbox-disabled",
        "textarea-demo", "textarea-disabled", "textarea-with-label", "textarea-with-button",
        "switch-demo", "switch-disabled", "switch-with-label",
        "field-demo", "field-textarea", "field-set-demo", "field-checkbox", "field-switch",
        "toggle-demo", "toggle-outline", "toggle-with-text", "toggle-sm", "toggle-lg", "toggle-disabled",
        "aspect-ratio-demo",
        "native-select-demo", "native-select-optgroup", "native-select-disabled", "native-select-invalid",
        "input-group-demo", "input-group-icon", "input-group-text", "input-group-button",
        "input-group-textarea", "input-group-spinner", "input-group-label",
        "input-group-button-group", "input-group-custom-input",
        "dialog-demo", "dialog-form",
        "sheet-demo", "sheet-side",
        "alert-demo", "alert-destructive", "alert-with-icon",
        "alert-dialog-default", "alert-dialog-with-row",
        "accordion-default", "accordion-multiple",
        "breadcrumb-default", "breadcrumb-separator", "breadcrumb-collapsible",
        "calendar-default", "calendar-with-selected-date",
        "carousel-default", "carousel-autoplay", "carousel-orientation",
        "chart-bar-chart", "chart-custom-colors",
        "collapsible-default", "collapsible-animated",
        "color-picker-default",
        "combobox-default", "combobox-with-group",
        "command-dialog", "command-combobox",
        "context-menu-default", "context-menu-with-submenus",
        "data-table-basic", "data-table-sorting-&-filtering", "data-table-pagination",
        "date-picker-default", "date-picker-with-presets",
        "drawer-right", "drawer-left", "drawer-bottom",
        "dropdown-menu-default", "dropdown-menu-with-checkboxes",
        "hover-card-default",
        "input-otp-default", "input-otp-with-separator", "input-otp-controlled",
        "menubar-default",
        "navigation-menu-default", "navigation-menu-horizontal",
        "pagination-default",
        "popover-default",
        "radio-group-default",
        "resizable-horizontal", "resizable-vertical", "resizable-with-handle",
        "scroll-area-horizontal", "scroll-area-vertical",
        "select-default", "select-with-label", "select-disabled",
        "sidebar-default", "sidebar-collapsible",
        "slider-default", "slider-with-min-max-steps",
        "stepper-default", "stepper-vertical",
        "table-default", "table-with-sorting",
        "tabs-default",
        "toggle-group-single", "toggle-group-multiple", "toggle-group-outline",
        "tooltip-default", "tooltip-with-arrow",
    ];

    for name in demos {
        let element = get_demo(name);
        assert!(element.is_some(), "Demo {} should return Some(Element), got None", name);
    }
}
