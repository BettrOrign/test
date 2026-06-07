use evdev::uinput::VirtualDeviceBuilder;
use evdev::{EventType, InputEvent, Key, RelativeAxisType, Synchronization};

fn build_mouse() -> Result<evdev::uinput::VirtualDevice, Box<dyn std::error::Error>> {
    let device = VirtualDeviceBuilder::new()?
        .name("mouse_and_what")
        .with_relative_axes(&{
            let mut axes = evdev::AttributeSet::new();
            axes.insert(RelativeAxisType::REL_X);
            axes.insert(RelativeAxisType::REL_Y);
            axes.insert(RelativeAxisType::REL_WHEEL);
            axes.insert(RelativeAxisType::REL_HWHEEL);
            axes
        })?
        .with_keys(&{
            let mut keys = evdev::AttributeSet::new();
            keys.insert(Key::BTN_LEFT);
            keys
        })?
        .build()?;
    Ok(device)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Создаю виртуальную мышь...");
    let mut mouse = build_mouse()?;
    println!("Мышь создана!");

    // Выводим путь устройства
    for path in mouse.enumerate_dev_nodes_blocking()? {
        let path = path?;
        println!("Доступна по пути: {}", path.display());
    }

    // Несколько движений с задержками
    for i in 0..5 {
        println!("Движение {}...", i + 1);

        let x = InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_X.0, 50);
        let y = InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_Y.0, 50);
        let syn_rep = InputEvent::new(EventType::SYNCHRONIZATION, Synchronization::SYN_REPORT.0, 0);

        mouse.emit(&[x, y, syn_rep])?;

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    println!("Готово!");
    Ok(())
}
