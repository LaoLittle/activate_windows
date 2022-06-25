pub fn get_display_size() -> (u64, u64) {
    #[cfg(target_os = "macos")]
    {
        use core_graphics::display::CGDisplay;
        let cg = CGDisplay::main();

        return (cg.pixels_wide(), cg.pixels_high());
    }

    #[cfg(not(target_os = "macos"))]
    {
        let lp = winit::event_loop::EventLoop::new();
        let size = lp.primary_monitor().unwrap().size();

        return (size.width, size.height);
    }
}