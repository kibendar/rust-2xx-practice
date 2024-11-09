fn time_conversion(s: &str) -> String {
    let mut parts: Vec<&str> = s.split(':').collect();
    let am_pm = &parts[2][2..]; // Get AM/PM suffix from the last part

    let hour: u32 = parts[0].parse().unwrap();
    let minute = parts[1];
    let second = &parts[2][..2];

    let military_hour = match (hour, am_pm) {
        (12, "AM") => 0,
        (12, "PM") => 12,
        (h, "PM") => h + 12,
        (h, "AM") => h,
        _ => hour,
    };

    format!("{:02}:{:}:{:}", military_hour, minute, second)
}

#[test]
fn main() {
    let time = "07:05:45PM";
    println!("{}", time_conversion(time));
}