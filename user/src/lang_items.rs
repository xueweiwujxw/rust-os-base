#[panic_handler]

fn panic(_info: &core::panic::PanicInfo) -> ! {
    let err = _info.message().unwrap();
    if let Some(location) = _info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            err
        );
    } else {
        println!("Panicked: {}", err);
    }
    loop {}
}
