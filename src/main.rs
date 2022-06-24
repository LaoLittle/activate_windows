use std::env;
use std::error::Error;

use iced::{Application, Color, Column, Command, Element, Settings, Text, window};
use iced::window::Position;

static FONT: &[u8] = include_bytes!("../MiSans-Regular.ttf");

fn main() -> Result<(), Box<dyn Error>> {
    {
        let env: Vec<String> = env::args().collect();
        let get_size = String::from("get_size");

        if env.contains(&get_size) {
            let lp = winit::event_loop::EventLoop::new();
            let str: String;
            match lp.primary_monitor() {
                Some(m) => {
                    let s = m.size();
                    str = format!("{}x{}", s.width, s.height);
                }
                None => {
                    str = String::from("1920x1080");
                }
            }

            print!("{str}"); // can ignore trim()
            return Ok(());
        }
    }

    let out = std::process::Command::new(env::current_exe()?)
        .args(&["get_size"]).output()?;

    let string = String::from_utf8(out.stdout)?;

    let sli: Vec<&str> = string.split('x').collect();

    let width: u32 = sli[0].parse()?;
    let height: u32 = sli[1].parse()?;

    let font_size = width.min(height) >> 6;

    ActivateWindows::run(
        Settings {
            id: None,
            window: window::Settings {
                size: (width >> 3, font_size << 1),
                position: Position::Specific(width as i32 / 3, (height as f32 / 2.5) as _),
                min_size: None,
                max_size: None,
                resizable: false,
                decorations: false,
                transparent: true,
                always_on_top: true,
                icon: None,
            },
            flags: (),
            default_font: Some(FONT),
            default_text_size: font_size as u16,
            text_multithreading: false,
            antialiasing: true,
            exit_on_close_request: true,
            try_opengles_first: false,
        }
    )?;

    Ok(())
}

struct ActivateWindows;

impl Application for ActivateWindows {
    type Executor = iced::executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self,
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::new()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let color = Color::new(1.0, 1.0, 1.0, 0.4);


        let top = "激活 Windows";
        #[cfg(target_os = "linux")]
            let top = "激活 Linux";
        #[cfg(target_os = "macos")]
            let top = "激活 MacOS";


        let bottom = "转到“设置”以激活 Windows。";
        #[cfg(target_os = "linux")]
            let bottom = "转到“设置”以激活 Linux。";
        #[cfg(target_os = "macos")]
            let bottom = "转到“偏好设置”以激活 MacOS。";

        Column::new()
            .push(Text::new(top).color(color))
            .push(Text::new(bottom).color(color))
            .into()
    }

    fn background_color(&self) -> Color {
        Color::TRANSPARENT
    }
}