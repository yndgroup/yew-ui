use base64::prelude::BASE64_STANDARD;
use base64::Engine;

#[derive(Clone, PartialEq, Debug)]
pub enum IconName {
    Home,
}

impl Default for IconName {
    fn default() -> Self {
        Self::Home
    }
}

pub fn get_svg(icon_name: IconName, width: i32, height: i32, color: String) -> String {
    let icon = match icon_name {
        IconName::Home => format!(
            r#"<svg t="1734363781823" width="{}" height="{}"  fill="{}" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="4249" xmlns:xlink="http://www.w3.org/1999/xlink"><path d="M96.61472 624v336a48 48 0 0 0 48 48h448V624h192v384h96a48 48 0 0 0 48-48V624L512.61472 208 96.61472 624z m336 192H240.61472V624h192v192z m579.024-304.64l0.096-0.112L832.61472 332.112V192a48 48 0 1 0-96 0v44.112L547.73472 47.248l-0.032 0.032a47.872 47.872 0 0 0-29.84-14.992l-0.144-0.016A49.088 49.088 0 0 0 512.63072 32H512.61472c-13.6 0-25.856 5.664-34.592 14.752L15.36672 509.408a48 48 0 1 0 66.944 68.768l0.016 0.016L512.61472 147.92v0.016l1.232 1.152v0.032l430.032 430.032 0.112-0.128A47.68 47.68 0 0 0 976.61472 592a48 48 0 0 0 48-48c0-12.64-4.992-24.064-12.976-32.64z" p-id="4250"></path></svg>"#,
            width, height, color
        ),
    };
    let encoded_svg = BASE64_STANDARD.encode(icon);
    let image_src = format!("data:image/svg+xml;base64,{}", encoded_svg);
    image_src
}