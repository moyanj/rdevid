use web_sys::window;

pub fn get_device_info() -> String {
    let window = window().expect("no global window exists");
    let navigator = window.navigator();
    let screen = window.screen().unwrap();

    let mut info = String::new();

    // 硬件信息
    info.push_str(&format!("HC:{};", navigator.hardware_concurrency()));
    info.push_str(&format!(
        "UA:{};",
        navigator.user_agent().unwrap_or("".to_string())
    ));
    info.push_str(&format!("AN:{};", navigator.app_name())); // 新增浏览器厂商信息
    info.push_str(&format!(
        "AV:{};",
        navigator.app_version().unwrap_or("".to_string())
    )); // 新增浏览器版本信息

    // 浏览器信息
    if let Ok(pl) = navigator.platform() {
        info.push_str(&format!("PL:{};", pl));
    }

    // 屏幕信息
    info.push_str(&format!(
        "SCR:{}x{};",
        screen.width().unwrap_or(0),
        screen.height().unwrap_or(0)
    ));
    info.push_str(&format!("CD:{};", screen.color_depth().unwrap_or(0))); // 新增颜色深度

    // 时区信息
    let date = js_sys::Date::new_0();
    info.push_str(&format!("TZ:{};", date.get_timezone_offset()));

    // 额外浏览器特性检测
    let langs = navigator.languages();
    info.push_str(&format!("LANG:{};", langs.join(",")));
    info.push_str(&format!(
        "PLUGINS:{};",
        navigator
            .plugins()
            .map(|plugins| plugins.length())
            .unwrap_or(0)
    )); // 新增插件数量
    info.push_str(&format!("TP:{};", navigator.max_touch_points())); // 新增触控点数

    info
}
