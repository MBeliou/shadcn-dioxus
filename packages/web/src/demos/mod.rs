use dioxus::prelude::*;
use lucide_dioxus::{
    ArrowUp, Bold, Check, Copy, CornerDownLeft, CreditCard, FileCode, Info, Italic, Link2, Loader,
    Mail, Plus, RefreshCw, Search, Star, Underline,
};
use ui::*;

// ---------------------------------------------------------------------------
// Wrapper functions (Replacing the broken demo_fn! macro)
// We directly call and return the Element instead of wrapping in rsx!
// ---------------------------------------------------------------------------

fn AccordionDemo() -> Element {
    AccordionDefault()
}
fn AlertDefaultDemo() -> Element {
    AlertDemo()
}
fn AlertDialogDemo() -> Element {
    AlertDialogDefault()
}
fn CalendarDemo() -> Element {
    CalendarDefault()
}
fn CarouselDemo() -> Element {
    CarouselDefault()
}
fn ChartDemo() -> Element {
    ChartBarChart()
}
fn CollapsibleDemo() -> Element {
    CollapsibleDefault()
}
fn ColorPickerDemo() -> Element {
    ColorPickerDefault()
}
fn ComboboxDemo() -> Element {
    ComboboxDefault()
}
fn CommandDemo() -> Element {
    CommandDialog()
}
fn ContextMenuDemo() -> Element {
    ContextMenuDefault()
}
fn DataTableDemo() -> Element {
    DataTableBasic()
}
fn DataTableSortingAndFilteringDemo() -> Element {
    DataTableSortingAndFiltering()
}
fn DatePickerDemo() -> Element {
    DatePickerDefault()
}
fn DrawerDemo() -> Element {
    DrawerRight()
}
fn DropdownMenuDemo() -> Element {
    DropdownMenuDefault()
}
fn HoverCardDemo() -> Element {
    HoverCardDefault()
}
fn InputOtpDemo() -> Element {
    InputOtpDefault()
}
fn MenubarDemo() -> Element {
    MenubarDefault()
}
fn NavigationMenuDemo() -> Element {
    NavigationMenuDefault()
}
fn PaginationDemo() -> Element {
    PaginationDefault()
}
fn PopoverDemo() -> Element {
    PopoverDefault()
}
fn RadioGroupDemo() -> Element {
    RadioGroupDefault()
}
fn ResizableDemo() -> Element {
    ResizableHorizontal()
}
fn ScrollAreaDemo() -> Element {
    ScrollAreaHorizontal()
}
fn SelectDemo() -> Element {
    SelectDefault()
}
fn SidebarDemo() -> Element {
    SidebarDefault()
}
fn SliderDemo() -> Element {
    SliderDefault()
}
fn SliderWithMinMaxStepsDemo() -> Element {
    SliderWithMinMaxSteps()
}
fn StepperDemo() -> Element {
    StepperDefault()
}
fn TableDemo() -> Element {
    TableDefault()
}
fn TabsDemo() -> Element {
    TabsDefault()
}
fn ToggleGroupDemo() -> Element {
    ToggleGroupSingle()
}
fn TooltipDemo() -> Element {
    TooltipDefault()
}

// Breadcrumb wrappers (renamed to avoid collision with components)
fn BreadcrumbDefaultDemo() -> Element {
    BreadcrumbDefault()
}
fn BreadcrumbSeparatorDemo() -> Element {
    BreadcrumbSeparatorContent()
}
fn BreadcrumbCollapsibleDemo() -> Element {
    BreadcrumbCollapsible()
}

fn CarouselOrientationDemo() -> Element {
    rsx! {
        Carousel { orientation: CarouselOrientation::Vertical, total: 3, height: "16rem".to_string(), class: "w-48",
            CarouselContent {
                CarouselItem { index: 0,
                    div { class: "flex h-full items-center justify-center rounded-md bg-muted",
                        span { class: "text-xl font-semibold", "1" }
                    }
                }
                CarouselItem { index: 1,
                    div { class: "flex h-full items-center justify-center rounded-md bg-muted",
                        span { class: "text-xl font-semibold", "2" }
                    }
                }
                CarouselItem { index: 2,
                    div { class: "flex h-full items-center justify-center rounded-md bg-muted",
                        span { class: "text-xl font-semibold", "3" }
                    }
                }
            }
            CarouselPrevious {}
            CarouselNext {}
        }
    }
}

// Sheet / Drawer / etc. already exist as separate functions; no further wrappers needed

// ---------------------------------------------------------------------------
// Original full demo implementations (with new names where needed)
// ---------------------------------------------------------------------------

fn AlertDemo() -> Element {
    rsx! {
        Alert {
            AlertTitle { "Heads up!" }
            AlertDescription { "You can add components to your app using the CLI." }
        }
    }
}

fn AlertDestructive() -> Element {
    rsx! {
        Alert { variant: AlertVariant::Destructive,
            AlertTitle { "Error" }
            AlertDescription { "Your session has expired. Please log in again." }
        }
    }
}

fn AlertWithIcon() -> Element {
    rsx! {
        Alert {
            Info { class: "size-4" }
            AlertTitle { "Information" }
            AlertDescription { "This is an informational alert with an icon." }
        }
    }
}

fn AlertDialogDefault() -> Element {
    rsx! {
        AlertDialog {
            AlertDialogTrigger {
                Button { variant: ButtonVariant::Outline, "Delete account" }
            }
            AlertDialogContent {
                AlertDialogHeader {
                    AlertDialogTitle { "Are you absolutely sure?" }
                    AlertDialogDescription {
                        "This action cannot be undone. This will permanently delete your account."
                    }
                }
                AlertDialogFooter {
                    AlertDialogCancel {
                        Button { variant: ButtonVariant::Outline, "Cancel" }
                    }
                    AlertDialogAction {
                        Button { variant: ButtonVariant::Destructive, "Continue" }
                    }
                }
            }
        }
    }
}

fn AlertDialogWithRow() -> Element {
    rsx! {
        AlertDialog {
            AlertDialogTrigger {
                Button { variant: ButtonVariant::Outline, "Show Dialog" }
            }
            AlertDialogContent {
                AlertDialogHeader {
                    AlertDialogTitle { "Use Google's location service?" }
                    AlertDialogDescription {
                        "Let Google help apps determine location. This means sending anonymous location data to Google, even when no apps are running."
                    }
                }
                AlertDialogFooter {
                    AlertDialogCancel {
                        Button { variant: ButtonVariant::Outline, "Cancel" }
                    }
                    AlertDialogAction {
                        Button { variant: ButtonVariant::Default, "Agree" }
                    }
                }
            }
        }
    }
}

fn AccordionDefault() -> Element {
    rsx! {
        Accordion { multiple: false,
            AccordionItem { value: "item-1".to_string(),
                AccordionTrigger { "Is it accessible?" }
                AccordionContent { "Yes. It adheres to the WAI-ARIA design pattern." }
            }
            AccordionItem { value: "item-2".to_string(),
                AccordionTrigger { "Is it styled?" }
                AccordionContent {
                    "Yes. It comes with default styles that match the other components' aesthetic."
                }
            }
            AccordionItem { value: "item-3".to_string(),
                AccordionTrigger { "Is it animated?" }
                AccordionContent {
                    "Yes. It's animated by default, but you can disable it if you prefer."
                }
            }
        }
    }
}

fn AccordionMultiple() -> Element {
    rsx! {
        Accordion { multiple: true,
            AccordionItem { value: "item-1".to_string(),
                AccordionTrigger { "What is shadcn/ui?" }
                AccordionContent { "A set of beautifully designed components." }
            }
            AccordionItem { value: "item-2".to_string(),
                AccordionTrigger { "Can I use it in my Dioxus app?" }
                AccordionContent { "Absolutely! This library is built for Dioxus." }
            }
        }
    }
}

fn BreadcrumbDefault() -> Element {
    rsx! {
        Breadcrumb {
            BreadcrumbList {
                BreadcrumbItem {
                    BreadcrumbLink { href: "#", "Home" }
                }
                BreadcrumbSeparator {}
                BreadcrumbItem {
                    BreadcrumbLink { href: "#", "Components" }
                }
                BreadcrumbSeparator {}
                BreadcrumbItem {
                    BreadcrumbPage { "Breadcrumb" }
                }
            }
        }
    }
}

// Renamed from BreadcrumbSeparator to BreadcrumbSeparatorContent to avoid shadowing the UI component
fn BreadcrumbSeparatorContent() -> Element {
    rsx! {
        Breadcrumb {
            BreadcrumbList {
                BreadcrumbItem {
                    BreadcrumbLink { href: "#", "Settings" }
                }
                BreadcrumbSeparator {}
                BreadcrumbItem {
                    BreadcrumbLink { href: "#", "Profile" }
                }
                BreadcrumbSeparator {}
                BreadcrumbItem {
                    BreadcrumbPage { "Edit" }
                }
            }
        }
    }
}

fn BreadcrumbCollapsible() -> Element {
    rsx! {
        Breadcrumb {
            BreadcrumbList {
                BreadcrumbItem {
                    BreadcrumbLink { href: "#", "Home" }
                }
                BreadcrumbSeparator {}
                BreadcrumbItem {
                    span { "…" }
                }
                BreadcrumbSeparator {}
                BreadcrumbItem {
                    BreadcrumbPage { "Current Page" }
                }
            }
        }
    }
}

fn CalendarDefault() -> Element {
    rsx! { ui::Calendar { default_year: 2025, default_month: 4 } }
}

fn CalendarWithSelectedDate() -> Element {
    rsx! { ui::Calendar { default_year: 2025, default_month: 4, default_date: Some((2025, 4, 15)) } }
}

fn CarouselDefault() -> Element {
    rsx! {
        Carousel { total: 3, auto_play: false, width: "100%".to_string(), class: "max-w-xs",
            CarouselContent {
                CarouselItem { index: 0,
                    div { class: "flex h-32 items-center justify-center rounded-md bg-muted",
                        span { class: "text-2xl font-semibold", "1" }
                    }
                }
                CarouselItem { index: 1,
                    div { class: "flex h-32 items-center justify-center rounded-md bg-muted",
                        span { class: "text-2xl font-semibold", "2" }
                    }
                }
                CarouselItem { index: 2,
                    div { class: "flex h-32 items-center justify-center rounded-md bg-muted",
                        span { class: "text-2xl font-semibold", "3" }
                    }
                }
            }
            CarouselPrevious {}
            CarouselNext {}
            CarouselIndicators {}
        }
    }
}

fn CarouselAutoplay() -> Element {
    rsx! {
        Carousel { total: 3, auto_play: true, auto_play_interval_ms: 1500, width: "100%".to_string(), class: "max-w-xs",
            CarouselContent {
                for i in 0..3 {
                    CarouselItem { index: i,
                        div { class: "flex h-32 items-center justify-center rounded-md bg-muted",
                            span { class: "text-2xl font-semibold", "{i + 1}" }
                        }
                    }
                }
            }
            CarouselIndicators {}
        }
    }
}

fn ChartBarChart() -> Element {
    let bars = vec![
        ui::ChartBar {
            label: "A".into(),
            value: 10.0,
            color: "var(--primary)".into(),
        },
        ui::ChartBar {
            label: "B".into(),
            value: 7.0,
            color: "var(--secondary)".into(),
        },
        ui::ChartBar {
            label: "C".into(),
            value: 12.0,
            color: "var(--destructive)".into(),
        },
    ];
    rsx! { ui::Chart { height: 100.0, width: 200.0, bars } }
}

fn ChartCustomColors() -> Element {
    let bars = vec![
        ui::ChartBar {
            label: "X".into(),
            value: 5.0,
            color: "#ff6b6b".into(),
        },
        ui::ChartBar {
            label: "Y".into(),
            value: 8.0,
            color: "#4ecdc4".into(),
        },
        ui::ChartBar {
            label: "Z".into(),
            value: 3.0,
            color: "#45b7d1".into(),
        },
    ];
    rsx! { ui::Chart { height: 120.0, width: 240.0, bars } }
}

fn CollapsibleDefault() -> Element {
    rsx! {
        Collapsible {
            CollapsibleTrigger { Button { variant: ButtonVariant::Outline, "Toggle" } }
            CollapsibleContent { class: "mt-2 rounded-md border p-4", "This content can be shown or hidden." }
        }
    }
}

fn CollapsibleAnimated() -> Element {
    rsx! {
        Collapsible { default_open: true,
            CollapsibleTrigger { Button { variant: ButtonVariant::Ghost, "Show details" } }
            CollapsibleContent { class: "mt-2 space-y-2", p { "Item 1" } p { "Item 2" } }
        }
    }
}

fn ColorPickerDefault() -> Element {
    rsx! { ui::ColorPicker {} }
}

fn ComboboxDefault() -> Element {
    let options = vec![
        ui::ComboboxOption {
            value: "nextjs".into(),
            label: "Next.js".into(),
        },
        ui::ComboboxOption {
            value: "sveltekit".into(),
            label: "SvelteKit".into(),
        },
        ui::ComboboxOption {
            value: "nuxt".into(),
            label: "Nuxt.js".into(),
        },
        ui::ComboboxOption {
            value: "remix".into(),
            label: "Remix".into(),
        },
    ];
    rsx! { ui::Combobox { options, placeholder: "Select framework..." } }
}

fn ComboboxWithGroup() -> Element {
    let options = vec![
        ui::ComboboxOption {
            value: "apple".into(),
            label: "Apple".into(),
        },
        ui::ComboboxOption {
            value: "banana".into(),
            label: "Banana".into(),
        },
        ui::ComboboxOption {
            value: "blueberry".into(),
            label: "Blueberry".into(),
        },
    ];
    rsx! { ui::Combobox { options, placeholder: "Pick a fruit..." } }
}

fn CommandDialog() -> Element {
    rsx! {
        Command { class: "w-[400px] rounded-lg border shadow-md",
            CommandInput { placeholder: "Type a command or search..." }
            CommandList { max_height: 300,
                CommandEmpty { "No results found." }
                CommandGroup { heading: "Suggestions",
                    CommandItem { value: "settings", "Settings" }
                    CommandItem { value: "profile", "Profile" }
                }
                CommandSeparator {}
                CommandGroup { heading: "Actions",
                    CommandItem { value: "invite", "Invite members" }
                    CommandItem { value: "export", "Export data" }
                }
            }
        }
    }
}

fn CommandCombobox() -> Element {
    rsx! {
        Command { class: "w-[400px] rounded-lg border shadow-md",
            CommandInput { placeholder: "Search..." }
            CommandList { max_height: 150,
                CommandItem { value: "calendar", "Calendar" }
                CommandItem { value: "search-files", "Search Files" }
            }
        }
    }
}

fn ContextMenuDefault() -> Element {
    rsx! {
        ContextMenu {
            ContextMenuTrigger {
                div { class: "flex h-40 w-60 items-center justify-center rounded-md border border-dashed text-sm",
                    "Right click here"
                }
            }
            ContextMenuContent {
                ContextMenuItem { "Back" }
                ContextMenuItem { disabled: true, "Forward (disabled)" }
                ContextMenuSeparator {}
                ContextMenuItem { "Inspect" }
            }
        }
    }
}

fn ContextMenuWithSubmenus() -> Element {
    rsx! {
        ContextMenu {
            ContextMenuTrigger {
                div { class: "flex h-32 w-48 items-center justify-center rounded-md border border-dashed text-sm",
                    "Right click"
                }
            }
            ContextMenuContent {
                ContextMenuItem { "New Tab" }
                ContextMenuItem { "New Window" }
                ContextMenuSeparator {}
                ContextMenuItem { "Settings" }
            }
        }
    }
}

fn DataTableBasic() -> Element {
    let columns = vec![
        ui::DataTableColumn::new("name", "Name"),
        ui::DataTableColumn::new("email", "Email"),
        ui::DataTableColumn::new("role", "Role"),
    ];
    let rows = vec![
        vec!["Alice".into(), "alice@example.com".into(), "Admin".into()],
        vec!["Bob".into(), "bob@example.com".into(), "User".into()],
    ];
    rsx! { ui::DataTable { columns, rows, show_toolbar: false, show_pagination: false } }
}

fn DataTableSortingAndFiltering() -> Element {
    let columns = vec![
        ui::DataTableColumn::new("name", "Name"),
        ui::DataTableColumn::new("email", "Email"),
    ];
    let rows = vec![
        vec!["Alice".into(), "alice@acme.com".into()],
        vec!["Bob".into(), "bob@acme.com".into()],
        vec!["Charlie".into(), "charlie@acme.com".into()],
    ];
    rsx! { ui::DataTable { columns, rows } }
}

fn DataTablePagination() -> Element {
    let columns = vec![
        ui::DataTableColumn::new("title", "Title"),
        ui::DataTableColumn::new("status", "Status"),
    ];
    let rows = vec![
        vec!["Task 1".into(), "Done".into()],
        vec!["Task 2".into(), "In progress".into()],
        vec!["Task 3".into(), "Todo".into()],
        vec!["Task 4".into(), "Done".into()],
        vec!["Task 5".into(), "Todo".into()],
    ];
    rsx! { ui::DataTable { columns, rows, page_size: 2 } }
}

fn DatePickerDefault() -> Element {
    rsx! { DatePicker {} }
}
fn DatePickerWithPresets() -> Element {
    rsx! { DatePicker {} }
}

fn DrawerRight() -> Element {
    rsx! {
        Drawer {
            DrawerTrigger { Button { variant: ButtonVariant::Outline, "Open Right Drawer" } }
            DrawerOverlay {}
            DrawerContent { side: DrawerSide::Right,
                DrawerHeader {
                    DrawerTitle { "Right Drawer" }
                    DrawerDescription { "This drawer appears from the right." }
                }
                DrawerFooter {
                    DrawerClose { Button { variant: ButtonVariant::Outline, "Cancel" } }
                    Button { "Save" }
                }
            }
        }
    }
}

fn DrawerLeft() -> Element {
    rsx! {
        Drawer {
            DrawerTrigger { Button { variant: ButtonVariant::Outline, "Open Left Drawer" } }
            DrawerOverlay {}
            DrawerContent { side: DrawerSide::Left,
                DrawerHeader {
                    DrawerTitle { "Left Drawer" }
                    DrawerDescription { "This drawer appears from the left." }
                }
            }
        }
    }
}

fn DrawerBottom() -> Element {
    rsx! {
        Drawer {
            DrawerTrigger { Button { variant: ButtonVariant::Outline, "Open Bottom Drawer" } }
            DrawerOverlay {}
            DrawerContent { side: DrawerSide::Bottom,
                DrawerHeader { DrawerTitle { "Bottom Drawer" } }
            }
        }
    }
}

fn DropdownMenuDefault() -> Element {
    rsx! {
        DropdownMenu {
            DropdownMenuTrigger { Button { variant: ButtonVariant::Outline, "Open" } }
            DropdownMenuContent {
                DropdownMenuLabel { "My Account" }
                DropdownMenuSeparator {}
                DropdownMenuItem { "Profile" }
                DropdownMenuItem { "Billing" }
                DropdownMenuSeparator {}
                DropdownMenuItem { "Log out" }
            }
        }
    }
}

fn DropdownMenuWithCheckboxes() -> Element {
    rsx! {
        DropdownMenu {
            DropdownMenuTrigger { Button { variant: ButtonVariant::Outline, "Options" } }
            DropdownMenuContent {
                DropdownMenuItem { Check { class: "mr-2 h-4 w-4" } "Show status bar" }
                DropdownMenuItem { Check { class: "mr-2 h-4 w-4 opacity-0" } "Show activity bar" }
            }
        }
    }
}

fn HoverCardDefault() -> Element {
    rsx! {
        HoverCard {
            HoverCardTrigger { Button { variant: ButtonVariant::Link, "@hover" } }
            HoverCardContent {
                div { class: "flex justify-between space-x-4",
                    Avatar {
                        AvatarImage { src: "https://github.com/shadcn.png" }
                        AvatarFallback { "VC" }
                    }
                    div { class: "space-y-1",
                        h4 { class: "text-sm font-semibold", "@shadcn" }
                        p { class: "text-sm", "A design engineer." }
                    }
                }
            }
        }
    }
}

fn InputOtpDefault() -> Element {
    rsx! {
        InputOtp { max_length: 4,
            InputOtpGroup {
                InputOtpSlot { index: 0 }
                InputOtpSlot { index: 1 }
                InputOtpSlot { index: 2 }
                InputOtpSlot { index: 3 }
            }
        }
    }
}

fn InputOtpWithSeparator() -> Element {
    rsx! {
        InputOtp { max_length: 6,
            InputOtpGroup {
                InputOtpSlot { index: 0 }
                InputOtpSlot { index: 1 }
                InputOtpSlot { index: 2 }
            }
            InputOtpSeparator {}
            InputOtpGroup {
                InputOtpSlot { index: 3 }
                InputOtpSlot { index: 4 }
                InputOtpSlot { index: 5 }
            }
        }
    }
}

fn InputOtpControlled() -> Element {
    rsx! {
        InputOtp { max_length: 4, value: "1234".to_string(),
            InputOtpGroup {
                InputOtpSlot { index: 0 }
                InputOtpSlot { index: 1 }
                InputOtpSlot { index: 2 }
                InputOtpSlot { index: 3 }
            }
        }
    }
}

fn MenubarDefault() -> Element {
    rsx! {
        Menubar {
            MenubarMenu {
                MenubarTrigger { "File" }
                MenubarContent {
                    MenubarItem { "New Tab" }
                    MenubarItem { "New Window" }
                    MenubarSeparator {}
                    MenubarItem { "Exit" }
                }
            }
            MenubarMenu {
                MenubarTrigger { "Edit" }
                MenubarContent {
                    MenubarItem { "Undo" }
                    MenubarItem { disabled: true, "Redo" }
                }
            }
        }
    }
}

fn NavigationMenuDefault() -> Element {
    rsx! {
        NavigationMenu {
            NavigationMenuList {
                NavigationMenuItem {
                    NavigationMenuTrigger { "Getting started" }
                    NavigationMenuContent {
                        ul { class: "grid gap-3 p-4 md:w-[400px] lg:w-[500px]",
                            li { "Introduction" }
                            li { "Installation" }
                        }
                    }
                }
                NavigationMenuItem { NavigationMenuLink { href: "#", "Components" } }
                NavigationMenuItem { NavigationMenuLink { href: "#", "Documentation" } }
            }
        }
    }
}

fn NavigationMenuHorizontal() -> Element {
    rsx! {
        NavigationMenu {
            NavigationMenuList {
                NavigationMenuItem {
                    NavigationMenuTrigger { "Product" }
                    NavigationMenuContent { div { class: "p-4 w-48", "Overview" } }
                }
                NavigationMenuItem { NavigationMenuLink { href: "#", "Pricing" } }
            }
        }
    }
}

fn PaginationDefault() -> Element {
    rsx! {
        Pagination { page: 1, total_pages: 5,
            PaginationContent {
                PaginationPrevious {}
                PaginationItem { "1" }
                PaginationItem { active: true, "2" }
                PaginationItem { "3" }
                PaginationEllipsis {}
                PaginationNext {}
            }
        }
    }
}

fn PopoverDefault() -> Element {
    rsx! {
        Popover {
            PopoverTrigger { Button { variant: ButtonVariant::Outline, "Open popover" } }
            PopoverContent { class: "w-80",
                div { class: "flex justify-between space-x-4",
                    div { class: "space-y-1",
                        h4 { class: "text-sm font-semibold", "Edit Profile" }
                        p { class: "text-sm text-muted-foreground", "Make changes to your profile." }
                    }
                }
            }
        }
    }
}

fn RadioGroupDefault() -> Element {
    rsx! {
        RadioGroup { default_value: Some("option-one".into()),
            div { class: "flex items-center space-x-2",
                RadioGroupItem { value: "option-one" }
                RadioGroupLabel { "Option One" }
            }
            div { class: "flex items-center space-x-2",
                RadioGroupItem { value: "option-two" }
                RadioGroupLabel { "Option Two" }
            }
        }
    }
}

fn ScrollAreaHorizontal() -> Element {
    rsx! {
        ScrollArea { class: "w-96 whitespace-nowrap rounded-md border",
            div { class: "flex w-max space-x-4 p-4",
                for i in 1..=8 {
                    div { class: "w-40 shrink-0", div { class: "rounded-md border bg-card p-4 text-sm", "Item {i}" } }
                }
            }
        }
    }
}

fn ScrollAreaVertical() -> Element {
    rsx! {
        ScrollArea { class: "h-72 w-48 rounded-md border",
            div { class: "p-4",
                for i in 1..=20 { div { class: "mt-2 text-sm", "Line {i}" } }
            }
        }
    }
}

fn SelectDefault() -> Element {
    rsx! {
        Select {
            SelectTrigger { class: "w-[180px]", SelectValue { placeholder: "Select a fruit" } }
            SelectContent {
                SelectItem { value: "apple", "Apple" }
                SelectItem { value: "banana", "Banana" }
                SelectItem { value: "blueberry", "Blueberry" }
            }
        }
    }
}

fn SelectWithLabel() -> Element {
    rsx! {
        Select {
            SelectTrigger { class: "w-[180px]", SelectValue { placeholder: "Choose fruit" } }
            SelectContent {
                SelectItem { value: "apple", "Apple" }
                SelectItem { value: "banana", "Banana" }
                SelectItem { value: "orange", "Orange" }
            }
        }
    }
}

fn SelectDisabled() -> Element {
    rsx! {
        Select {
            SelectTrigger { class: "w-[180px]", SelectValue { placeholder: "Disabled" } }
            SelectContent { force_mount: false, SelectItem { value: "apple", "Apple" } }
        }
    }
}

fn SidebarDefault() -> Element {
    rsx! {
        SidebarProvider { default_open: true,
            Sidebar {
                SidebarHeader {
                    SidebarMenu {
                        SidebarMenuItem { "Dashboard" }
                        SidebarMenuItem { active: true, "Orders" }
                        SidebarMenuItem { "Products" }
                    }
                }
                SidebarFooter { SidebarInput { placeholder: "Search..." } }
            }
            SidebarInset { div { class: "flex h-64 items-center justify-center text-muted-foreground", "Main content area" } }
        }
    }
}

fn SidebarCollapsible() -> Element {
    rsx! {
        SidebarProvider { default_open: true,
            Sidebar { collapsible: true,
                SidebarHeader { SidebarMenu { SidebarMenuItem { "Home" } } }
            }
            SidebarInset { div { class: "flex h-48 items-center justify-center text-sm", "Content" } }
        }
    }
}

fn SliderDefault() -> Element {
    rsx! { Slider { value: 50.0, class: "w-[60%]" } }
}

fn SliderWithMinMaxSteps() -> Element {
    rsx! { Slider { min: 0.0, max: 100.0, step: 10.0, value: 30.0 } }
}

fn StepperDefault() -> Element {
    rsx! {
        Stepper { default_step: 0, total_steps: 3,
            StepperItem { step: 0, StepperTitle { "Account" } StepperDescription { "Create your account." } }
            StepperSeparator {}
            StepperItem { step: 1, StepperTitle { "Profile" } StepperDescription { "Set up your profile." } }
            StepperSeparator {}
            StepperItem { step: 2, StepperTitle { "Review" } StepperDescription { "Review and submit." } }
            StepperFooter { StepperPrevious {} StepperIndicator {} StepperNext {} }
        }
    }
}

fn StepperVertical() -> Element {
    rsx! {
        Stepper { default_step: 1, total_steps: 2,
            StepperItem { step: 0, StepperTitle { "Step 1" } }
            StepperSeparator {}
            StepperItem { step: 1, StepperTitle { "Step 2" } }
            StepperFooter { StepperPrevious {} StepperNext {} }
        }
    }
}

fn TableDefault() -> Element {
    rsx! {
        Table {
            TableCaption { "A list of recent invoices." }
            TableHeader {
                TableRow {
                    TableHead { "Invoice" }
                    TableHead { "Status" }
                    TableHead { "Method" }
                    TableHead { class: "text-right", "Amount" }
                }
            }
            TableBody {
                TableRow {
                    TableCell { "INV001" }
                    TableCell { "Paid" }
                    TableCell { "Credit Card" }
                    TableCell { class: "text-right", "$250.00" }
                }
                TableRow {
                    TableCell { "INV002" }
                    TableCell { "Pending" }
                    TableCell { "PayPal" }
                    TableCell { class: "text-right", "$150.00" }
                }
            }
            TableFooter {
                TableRow {
                    TableCell { column_span: 3, "Total" }
                    TableCell { class: "text-right", "$400.00" }
                }
            }
        }
    }
}

fn TableWithSorting() -> Element {
    rsx! {
        Table {
            TableHeader {
                TableRow {
                    TableHead { "Name" }
                    TableHead { "Email" }
                }
            }
            TableBody {
                TableRow {
                    TableCell { "Alice" }
                    TableCell { "alice@example.com" }
                }
                TableRow {
                    TableCell { "Bob" }
                    TableCell { "bob@example.com" }
                }
            }
        }
    }
}

fn TabsDefault() -> Element {
    rsx! {
        Tabs { default_value: "account".to_string(),
            TabsList {
                TabsTrigger { value: "account", "Account" }
                TabsTrigger { value: "password", "Password" }
            }
            TabsContent { value: "account", p { class: "text-sm text-muted-foreground", "Account settings." } }
            TabsContent { value: "password", p { class: "text-sm text-muted-foreground", "Change your password here." } }
        }
    }
}

fn ToggleGroupSingle() -> Element {
    rsx! {
        ToggleGroup { group_type: ToggleGroupType::Single,
            ToggleGroupItem { value: "bold", Bold {} }
            ToggleGroupItem { value: "italic", Italic {} }
            ToggleGroupItem { value: "underline", Underline {} }
        }
    }
}

fn ToggleGroupMultiple() -> Element {
    rsx! {
        ToggleGroup { group_type: ToggleGroupType::Multiple,
            ToggleGroupItem { value: "bold", Bold {} }
            ToggleGroupItem { value: "italic", Italic {} }
            ToggleGroupItem { value: "underline", Underline {} }
        }
    }
}

fn ToggleGroupOutline() -> Element {
    rsx! {
        ToggleGroup { group_type: ToggleGroupType::Single,
            ToggleGroupItem { value: "bold", Bold {} }
            ToggleGroupItem { value: "italic", Italic {} }
        }
    }
}

fn TooltipDefault() -> Element {
    rsx! {
        Tooltip {
            TooltipTrigger { Button { variant: ButtonVariant::Outline, "Hover me" } }
            TooltipContent { p { "Add to library" } }
        }
    }
}

fn TooltipWithArrow() -> Element {
    rsx! {
        Tooltip {
            TooltipTrigger { Button { variant: ButtonVariant::Outline, "Hover with arrow" } }
            TooltipContent { side: "top", p { "Tooltip with arrow" } }
        }
    }
}

fn ResizableHorizontal() -> Element {
    rsx! {
        ResizablePanelGroup { direction: Direction::Horizontal, class: "h-32 w-full rounded-md border",
            ResizablePanel { index: 0, div { class: "flex h-full items-center justify-center", "Left" } }
            ResizableHandle {}
            ResizablePanel { index: 1, div { class: "flex h-full items-center justify-center", "Right" } }
        }
    }
}

fn ResizableVertical() -> Element {
    rsx! {
        ResizablePanelGroup { direction: Direction::Vertical, class: "h-64 w-full rounded-md border",
            ResizablePanel { index: 0, div { class: "flex h-full items-center justify-center", "Top" } }
            ResizableHandle {}
            ResizablePanel { index: 1, div { class: "flex h-full items-center justify-center", "Middle" } }
            ResizableHandle {}
            ResizablePanel { index: 2, div { class: "flex h-full items-center justify-center", "Bottom" } }
        }
    }
}

fn ResizableWithHandle() -> Element {
    rsx! {
        ResizablePanelGroup { direction: Direction::Horizontal, class: "h-32 w-full rounded-md border",
            ResizablePanel { index: 0, div { class: "flex h-full items-center justify-center", "Panel 1" } }
            ResizableHandle {}
            ResizablePanel { index: 1, div { class: "flex h-full items-center justify-center", "Panel 2" } }
        }
    }
}

// ---------------------------------------------------------------------------
// Dispatcher
// ---------------------------------------------------------------------------
pub fn get_demo(name: &str) -> Option<Element> {
    match name {
        "button-demo" => Some(rsx! { Button { "Button" } }),
        "button-primary" => Some(rsx! { Button { variant: ButtonVariant::Default, "Primary" } }),
        "button-secondary" => {
            Some(rsx! { Button { variant: ButtonVariant::Secondary, "Secondary" } })
        }
        "button-destructive" => {
            Some(rsx! { Button { variant: ButtonVariant::Destructive, "Destructive" } })
        }
        "button-outline" => Some(rsx! { Button { variant: ButtonVariant::Outline, "Outline" } }),
        "button-ghost" => Some(rsx! { Button { variant: ButtonVariant::Ghost, "Ghost" } }),
        "button-link" => Some(rsx! { Button { variant: ButtonVariant::Link, "Link" } }),
        "badge-demo" => Some(rsx! { Badge { "Badge" } }),
        "badge-secondary" => Some(rsx! { Badge { variant: BadgeVariant::Secondary, "Secondary" } }),
        "badge-destructive" => {
            Some(rsx! { Badge { variant: BadgeVariant::Destructive, "Destructive" } })
        }
        "badge-outline" => Some(rsx! { Badge { variant: BadgeVariant::Outline, "Outline" } }),
        "card-demo" => Some(rsx! {
            Card { class: "w-[350px]",
                CardHeader { CardTitle { "Card Title" } CardDescription { "Card description goes here." } }
                CardContent { p { "Card content goes here." } }
                CardFooter { Button { "Action" } }
            }
        }),
        "avatar-demo" => Some(rsx! {
            Avatar { AvatarImage { src: "https://github.com/shadcn.png", alt: "shadcn" } AvatarFallback { "CN" } }
        }),
        "avatar-fallback" => Some(rsx! { Avatar { AvatarFallback { "JD" } } }),
        "input-demo" => Some(rsx! { Input { placeholder: "Email" } }),
        "input-disabled" => Some(rsx! { Input { placeholder: "Disabled", disabled: true } }),
        "label-demo" => Some(rsx! { Label { "Label" } }),
        "progress-demo" => Some(rsx! { Progress { value: 60.0, class: "w-[60%]" } }),
        "progress-indeterminate" => Some(rsx! { Progress { class: "w-[60%]" } }),
        "separator-demo" => Some(rsx! { Separator {} }),
        "separator-vertical" => {
            Some(rsx! { Separator { orientation:  ui::separator::SeparatorOrientation::Vertical } })
        }
        "skeleton-demo" => Some(rsx! { Skeleton { class: "w-[100px] h-[20px] rounded-full" } }),
        "spinner-demo" => Some(rsx! { Spinner {} }),
        "kbd-demo" => Some(rsx! { Kbd { "⌘" } }),
        "kbd-group" => {
            Some(rsx! { div { class: "flex items-center gap-2", Kbd { "⌘" } Kbd { "C" } } })
        }
        "kbd-button" => Some(rsx! { Button { "⌘K" } }),
        "item-demo" => Some(rsx! { Item { "Item" } }),
        "empty-demo" => Some(
            rsx! { Empty { EmptyHeader { EmptyTitle { "No results found" } EmptyDescription { "Try adjusting your search or filters." } } } },
        ),
        "button-group-demo" => Some(
            rsx! { ButtonGroup::Root { Button { variant: ButtonVariant::Outline, "Left" } Button { variant: ButtonVariant::Outline, "Center" } Button { variant: ButtonVariant::Outline, "Right" } } },
        ),
        "checkbox-demo" => Some(rsx! { Checkbox { default_checked: CheckboxState::Checked } }),
        "checkbox-with-label" => Some(
            rsx! { div { class: "flex items-center space-x-2", Checkbox { id: "terms" } Label { "for": "terms", "Accept terms and conditions" } } },
        ),
        "checkbox-disabled" => Some(
            rsx! { div { class: "flex items-center space-x-2", Checkbox { id: "disabled", disabled: true, default_checked: CheckboxState::Checked } Label { "for": "disabled", "Disabled" } } },
        ),
        "textarea-demo" => Some(rsx! { Textarea { placeholder: "Type your message here." } }),
        "textarea-disabled" => {
            Some(rsx! { Textarea { placeholder: "Type your message here.", disabled: true } })
        }
        "textarea-with-label" => Some(
            rsx! { div { class: "grid w-full gap-1.5", Label { "for": "message", "Your message" } Textarea { placeholder: "Type your message here.", id: "message" } } },
        ),
        "textarea-with-button" => Some(
            rsx! { div { class: "grid w-full gap-2", Textarea { placeholder: "Type your message here." } Button { "Send message" } } },
        ),
        "switch-demo" => Some(rsx! { Switch {} }),
        "switch-disabled" => Some(
            rsx! { div { class: "flex items-center space-x-2", Switch { id: "disabled", disabled: true } Label { "for": "disabled", "Airplane Mode" } } },
        ),
        "switch-with-label" => Some(
            rsx! { div { class: "flex items-center space-x-2", Switch { id: "airplane-mode" } Label { "for": "airplane-mode", "Airplane Mode" } } },
        ),
        "field-demo" => Some(
            rsx! { Field { class: "max-w-sm", FieldLabel { "for": "email", "Email" } Input { id: "email", placeholder: "Enter your email" } FieldDescription { "We'll never share your email with anyone." } } },
        ),
        "field-textarea" => Some(
            rsx! { Field { class: "max-w-sm", FieldLabel { "for": "bio", "Bio" } Textarea { id: "bio", placeholder: "Tell us about yourself" } FieldDescription { "You can use markdown for formatting." } } },
        ),
        "field-set-demo" => Some(
            rsx! { FieldSet { class: "max-w-md", FieldLegend { "Address" } Field { FieldLabel { "for": "street", "Street" } Input { id: "street", placeholder: "123 Main St" } } Field { FieldLabel { "for": "city", "City" } Input { id: "city", placeholder: "New York" } } } },
        ),
        "field-checkbox" => Some(
            rsx! { Field { orientation: FieldOrientation::Horizontal, Checkbox { id: "terms" } FieldContent { FieldLabel { "for": "terms", "Accept terms and conditions" } FieldDescription { "You agree to our Terms of Service and Privacy Policy." } } } },
        ),
        "field-switch" => Some(
            rsx! { Field { orientation: FieldOrientation::Horizontal, class: "max-w-sm", Switch { id: "mfa" } FieldContent { FieldLabel { "for": "mfa", "Multi-factor Authentication" } FieldDescription { "Add an extra layer of security to your account." } } } },
        ),
        "toggle-demo" => Some(rsx! { Toggle { aria_label: "Toggle bold", Bold {} } }),
        "toggle-outline" => Some(
            rsx! { Toggle { variant: ToggleVariant::Outline, aria_label: "Toggle italic", Italic {} } },
        ),
        "toggle-with-text" => {
            Some(rsx! { Toggle { aria_label: "Toggle italic", Italic { class: "me-2" } "Italic" } })
        }
        "toggle-sm" => {
            Some(rsx! { Toggle { size: ToggleSize::Sm, aria_label: "Toggle bold", Bold {} } })
        }
        "toggle-lg" => {
            Some(rsx! { Toggle { size: ToggleSize::Lg, aria_label: "Toggle bold", Bold {} } })
        }
        "toggle-disabled" => {
            Some(rsx! { Toggle { disabled: true, aria_label: "Toggle underline", Underline {} } })
        }
        "aspect-ratio-demo" => Some(
            rsx! { div { class: "w-[450px]", AspectRatio { ratio: 16.0 / 9.0, class: "bg-muted", img { src: "https://images.unsplash.com/photo-1588345921523-c2dcdb7f1dcd?w=800&dpr=2&q=80", alt: "Photo by Drew Beamer", class: "h-full w-full rounded-md object-cover" } } } },
        ),
        "native-select-demo" => Some(
            rsx! { NativeSelect { NativeSelectOption { value: "", "Select a fruit" } NativeSelectOption { value: "apple", "Apple" } NativeSelectOption { value: "banana", "Banana" } NativeSelectOption { value: "blueberry", "Blueberry" } NativeSelectOption { value: "grapes", disabled: true, "Grapes" } NativeSelectOption { value: "pineapple", "Pineapple" } } },
        ),
        "native-select-optgroup" => Some(
            rsx! { NativeSelect { NativeSelectOption { value: "", "Select a food" } NativeSelectOptGroup { label: "Fruits", NativeSelectOption { value: "apple", "Apple" } NativeSelectOption { value: "banana", "Banana" } NativeSelectOption { value: "blueberry", "Blueberry" } } NativeSelectOptGroup { label: "Vegetables", NativeSelectOption { value: "carrot", "Carrot" } NativeSelectOption { value: "broccoli", "Broccoli" } NativeSelectOption { value: "spinach", "Spinach" } } } },
        ),
        "native-select-disabled" => Some(
            rsx! { NativeSelect { disabled: true, NativeSelectOption { value: "", "Select a fruit" } NativeSelectOption { value: "apple", "Apple" } NativeSelectOption { value: "banana", "Banana" } } },
        ),
        "native-select-invalid" => Some(
            rsx! { NativeSelect { aria_invalid: "true", NativeSelectOption { value: "", "Select a fruit" } NativeSelectOption { value: "apple", "Apple" } NativeSelectOption { value: "banana", "Banana" } } },
        ),
        "input-group-demo" => Some(rsx! { InputGroup { Input { placeholder: "Search..." } } }),
        "input-group-icon" => Some(
            rsx! { InputGroup { InputGroupAddon { Search {} } Input { placeholder: "Search..." } } },
        ),
        "input-group-text" => Some(
            rsx! { InputGroup { InputGroupAddon { "https://" } Input { placeholder: "website.com" } } },
        ),
        "input-group-button" => Some(
            rsx! { InputGroup { Input { placeholder: "Search..." } InputGroupButton { Button { variant: ButtonVariant::Outline, "Search" } } } },
        ),
        "input-group-textarea" => Some(
            rsx! { InputGroup { Textarea { placeholder: "Type your message here." } InputGroupButton { Button { "Send" } } } },
        ),
        "input-group-spinner" => Some(
            rsx! { InputGroup { InputGroupAddon { Spinner {} } Input { placeholder: "Loading...", disabled: true } } },
        ),
        "input-group-label" => Some(
            rsx! { InputGroup { InputGroupAddon { "Email" } Input { placeholder: "name@example.com" } } },
        ),
        "input-group-button-group" => Some(
            rsx! { InputGroup { Input { placeholder: "Search..." } InputGroupButton { Button { variant: ButtonVariant::Outline, "A" } Button { variant: ButtonVariant::Outline, "B" } } } },
        ),
        "input-group-custom-input" => Some(
            rsx! { InputGroup { InputGroupAddon { CreditCard {} } Input { placeholder: "Card number" } InputGroupAddon { "USD" } } },
        ),
        "dialog-demo" => Some(rsx! {
            Dialog {
                DialogTrigger { Button { variant: ButtonVariant::Outline, "Open Dialog" } }
                DialogContent {
                    DialogHeader { DialogTitle { "Dialog Title" } DialogDescription { "This is a description of the dialog." } }
                    DialogFooter { Button { "Save" } }
                }
            }
        }),
        "dialog-form" => Some(rsx! {
            Dialog {
                DialogTrigger { Button { variant: ButtonVariant::Outline, "Edit Profile" } }
                DialogContent {
                    DialogHeader { DialogTitle { "Edit profile" } DialogDescription { "Make changes to your profile here. Click save when you're done." } }
                    div { class: "grid gap-4 py-4",
                        div { class: "grid grid-cols-4 items-center gap-4", Label { "for": "name", class: "text-right", "Name" } Input { id: "name", value: "Pedro Duarte", class: "col-span-3" } }
                        div { class: "grid grid-cols-4 items-center gap-4", Label { "for": "username", class: "text-right", "Username" } Input { id: "username", value: "@peduarte", class: "col-span-3" } }
                    }
                    DialogFooter { Button { "Save changes" } }
                }
            }
        }),
        "sheet-demo" => Some(rsx! {
            Sheet {
                SheetTrigger { Button { variant: ButtonVariant::Outline, "Open" } }
                SheetContent {
                    SheetHeader { SheetTitle { "Edit Profile" } SheetDescription { "Make changes to your profile here." } }
                }
            }
        }),
        "sheet-side" => Some(rsx! {
            Sheet {
                SheetTrigger { Button { variant: ButtonVariant::Outline, "Open Left" } }
                SheetContent { side: Side::Left,
                    SheetHeader { SheetTitle { "Left Sheet" } SheetDescription { "Description goes here." } }
                }
            }
        }),

        // New Demo mappings resolved directly to avoid infinite component loops
        "alert-demo" | "alert-default" => Some(AlertDemo()),
        "alert-destructive" => Some(AlertDestructive()),
        "alert-with-icon" => Some(AlertWithIcon()),
        "alert-dialog-demo" | "alert-dialog-default" => Some(AlertDialogDefault()),
        "alert-dialog-with-row" => Some(AlertDialogWithRow()),
        "accordion-demo" | "accordion-default" => Some(AccordionDefault()),
        "accordion-multiple" => Some(AccordionMultiple()),
        "breadcrumb-demo" | "breadcrumb-default" => Some(BreadcrumbDefault()),
        "breadcrumb-separator" => Some(BreadcrumbSeparatorContent()),
        "breadcrumb-collapsible" => Some(BreadcrumbCollapsible()),
        "calendar-demo" | "calendar-default" => Some(CalendarDefault()),
        "calendar-with-selected-date" => Some(CalendarWithSelectedDate()),
        "carousel-demo" | "carousel-default" => Some(CarouselDefault()),
        "carousel-autoplay" => Some(CarouselAutoplay()),
        "carousel-orientation" => Some(CarouselOrientationDemo()),
        "chart-demo" | "chart-bar-chart" => Some(ChartBarChart()),
        "chart-custom-colors" => Some(ChartCustomColors()),
        "collapsible-demo" | "collapsible-default" => Some(CollapsibleDefault()),
        "collapsible-animated" => Some(CollapsibleAnimated()),
        "color-picker-demo" | "color-picker-default" => Some(ColorPickerDefault()),
        "combobox-demo" | "combobox-default" => Some(ComboboxDefault()),
        "combobox-with-group" => Some(ComboboxWithGroup()),
        "command-demo" | "command-dialog" => Some(CommandDialog()),
        "command-combobox" => Some(CommandCombobox()),
        "context-menu-demo" | "context-menu-default" => Some(ContextMenuDefault()),
        "context-menu-with-submenus" => Some(ContextMenuWithSubmenus()),
        "data-table-demo" | "data-table-basic" => Some(DataTableBasic()),
        "data-table-sorting-&-filtering" => Some(DataTableSortingAndFiltering()),
        "data-table-pagination" => Some(DataTablePagination()),
        "date-picker-demo" | "date-picker-default" => Some(DatePickerDefault()),
        "date-picker-with-presets" => Some(DatePickerWithPresets()),
        "drawer-demo" | "drawer-right" => Some(DrawerRight()),
        "drawer-left" => Some(DrawerLeft()),
        "drawer-bottom" => Some(DrawerBottom()),
        "dropdown-menu-demo" | "dropdown-menu-default" => Some(DropdownMenuDefault()),
        "dropdown-menu-with-checkboxes" => Some(DropdownMenuWithCheckboxes()),
        "hover-card-demo" | "hover-card-default" => Some(HoverCardDefault()),
        "input-otp-demo" | "input-otp-default" => Some(InputOtpDefault()),
        "input-otp-with-separator" => Some(InputOtpWithSeparator()),
        "input-otp-controlled" => Some(InputOtpControlled()),
        "menubar-demo" | "menubar-default" => Some(MenubarDefault()),
        "navigation-menu-demo" | "navigation-menu-default" => Some(NavigationMenuDefault()),
        "navigation-menu-horizontal" => Some(NavigationMenuHorizontal()),
        "pagination-demo" | "pagination-default" => Some(PaginationDefault()),
        "popover-demo" | "popover-default" => Some(PopoverDefault()),
        "radio-group-demo" | "radio-group-default" => Some(RadioGroupDefault()),
        "resizable-demo" | "resizable-horizontal" => Some(ResizableHorizontal()),
        "resizable-vertical" => Some(ResizableVertical()),
        "resizable-with-handle" => Some(ResizableWithHandle()),
        "scroll-area-demo" | "scroll-area-horizontal" => Some(ScrollAreaHorizontal()),
        "scroll-area-vertical" => Some(ScrollAreaVertical()),
        "select-demo" | "select-default" => Some(SelectDefault()),
        "select-with-label" => Some(SelectWithLabel()),
        "select-disabled" => Some(SelectDisabled()),
        "sidebar-demo" | "sidebar-default" => Some(SidebarDefault()),
        "sidebar-collapsible" => Some(SidebarCollapsible()),
        "slider-demo" | "slider-default" => Some(SliderDefault()),
        "slider-with-min-max-steps" | "slider-with-min/max-steps" => Some(SliderWithMinMaxSteps()),
        "stepper-demo" | "stepper-default" => Some(StepperDefault()),
        "stepper-vertical" => Some(StepperVertical()),
        "table-demo" | "table-default" => Some(TableDefault()),
        "table-with-sorting" => Some(TableWithSorting()),
        "tabs-demo" | "tabs-default" => Some(TabsDefault()),
        "toggle-group-demo" | "toggle-group-single" => Some(ToggleGroupSingle()),
        "toggle-group-multiple" => Some(ToggleGroupMultiple()),
        "toggle-group-outline" => Some(ToggleGroupOutline()),
        "tooltip-demo" | "tooltip-default" => Some(TooltipDefault()),
        "tooltip-with-arrow" => Some(TooltipWithArrow()),

        _ => None,
    }
}
