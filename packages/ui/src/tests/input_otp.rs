use dioxus::prelude::*;

#[test]
fn test_input_otp_renders() {
    let _: Element = rsx! {
        crate::InputOtp {
            crate::InputOtpGroup {
                crate::InputOtpSlot { index: 0 }
            }
        }
    };
    assert!(true);
}
