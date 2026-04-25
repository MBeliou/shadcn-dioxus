use crate::chart::ChartBar;

#[test]
fn test_chart_bar_struct() {
    let bar = ChartBar { label: "A".into(), value: 10.0, color: "red".into() };
    assert_eq!(bar.label, "A");
    assert_eq!(bar.value, 10.0);
    assert_eq!(bar.color, "red");
}
