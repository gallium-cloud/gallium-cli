use std::path::Component;

fn expected_filename() -> &'static str {
    if cfg!(windows) { "ga.exe" } else { "ga" }
}

pub fn exe_display_name() -> String {
    if let Ok(exe) = std::env::current_exe()
        && let Some(Component::Normal(last)) = exe.components().next_back()
    {
        if Some(expected_filename()) == last.to_str() {
            expected_filename().to_string()
        } else {
            format!("{}", exe.display())
        }
    } else {
        expected_filename().to_string()
    }
}
