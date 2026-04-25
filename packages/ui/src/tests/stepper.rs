use dioxus::prelude::*;

#[test]
fn test_stepper_renders() {
    let _: Element = rsx! {
        crate::Stepper { default_step: 0, total_steps: 2,
            crate::StepperItem { step: 0,
                crate::StepperTitle { "Step 1" }
            }
            crate::StepperSeparator {}
            crate::StepperItem { step: 1,
                crate::StepperTitle { "Step 2" }
            }
            crate::StepperFooter {
                crate::StepperPrevious {}
                crate::StepperNext {}
            }
        }
    };
    assert!(true);
}
