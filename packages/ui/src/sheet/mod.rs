// Re-export Dialog components as Sheet aliases
mod sheet;
pub use sheet::Sheet;
pub use crate::dialog::DialogClose as SheetClose;
pub use crate::dialog::DialogDescription as SheetDescription;
pub use crate::dialog::DialogFooter as SheetFooter;
pub use crate::dialog::DialogHeader as SheetHeader;
pub use crate::dialog::DialogPortal as SheetPortal;
pub use crate::dialog::DialogTitle as SheetTitle;
pub use crate::dialog::DialogTrigger as SheetTrigger;

// Custom Sheet components
mod sheet_content;
mod sheet_overlay;

pub use sheet_content::*;
pub use sheet_overlay::*;
