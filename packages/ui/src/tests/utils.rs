use crate::cn;
use tailwind_fuse::tw_merge;

#[test]
fn test_ring_arbitrary_value_merge() {
    let result = tw_merge!("ring-[3px]", "ring-0");
    assert_eq!(result, "ring-0", "Arbitrary ring value should be overridden");
}

#[test]
fn test_ring_arbitrary_value_with_variant() {
    let result = tw_merge!("focus-visible:ring-[3px]", "focus-visible:ring-0");
    assert_eq!(result, "focus-visible:ring-0");
}

#[test]
fn test_ring_width_and_inset_separate() {
    let result = tw_merge!("ring-1", "ring-inset");
    assert_eq!(result, "ring-1 ring-inset");
}

#[test]
fn test_cn_empty_additional() {
    assert_eq!(cn("bg-red text-white", ""), "bg-red text-white");
}

#[test]
fn test_cn_empty_base() {
    assert_eq!(cn("", "extra"), "extra");
}

#[test]
fn test_cn_both_empty() {
    assert_eq!(cn("", ""), "");
}

#[test]
fn test_cn_conflict_resolution() {
    let result = cn("p-2 text-sm", "p-4 text-lg");
    assert_eq!(result, "p-4 text-lg");
    assert!(!result.contains("text-sm"));
}

#[test]
fn test_cn_no_conflict() {
    let result = cn("p-2", "m-4");
    assert!(result.contains("p-2"));
    assert!(result.contains("m-4"));
}
