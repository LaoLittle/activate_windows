pub fn get_display_size() -> (u32, u32) {
    #[cfg(target_os = "macos")]
    {
        use core_graphics::display::CGDisplay;
        let cg = CGDisplay::main();

        return (cg.pixels_wide() as u32, cg.pixels_high() as u32);
    }

    #[cfg(not(target_os = "macos"))]
    {
        let lp = winit::event_loop::EventLoop::new();
        let size = lp.primary_monitor().unwrap().size();

        return (size.width, size.height);
    }
}

#[cfg(test)]
mod test {
    use font_kit::source::SystemSource;

    #[test]
    fn font() {
        let mut s = SystemSource::new()
            .select_family_by_name("Microsoft Yahei").unwrap()
            .fonts()
            .first()
            .unwrap()
            .load().unwrap();


    }
}