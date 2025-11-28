use std::env;
use winvd::{switch_desktop, get_desktop_count, move_window_to_desktop, create_desktop, Desktop};
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};

fn main() {
    // 1. Initialize COM
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
    }

    let args: Vec<String> = env::args().collect();

    // Basic usage check
    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  Switch (Implicit): desktop_switcher.exe <number>");
        eprintln!("  Switch (Explicit): desktop_switcher.exe switch <number>");
        eprintln!("  Move:              desktop_switcher.exe move <number>");
        return;
    }

    // 2. PARSE ARGUMENTS ROBUSTLY
    // We check if the first argument is a command keyword ("move" or "switch").
    let (mode, target_str) = if args[1] == "move" {
        if args.len() < 3 { eprintln!("Error: Missing desktop number for move."); return; }
        ("move", &args[2])
    } else if args[1] == "switch" {
        if args.len() < 3 { eprintln!("Error: Missing desktop number for switch."); return; }
        ("switch", &args[2])
    } else {
        // Assume the user just typed a number (e.g., "desktop_switcher.exe 2")
        ("switch", &args[1])
    };

    // Parse target number
    let target_visual_number: u32 = match target_str.parse() {
        Ok(n) => n,
        Err(_) => { 
            eprintln!("Error: '{}' is not a valid number.", target_str); 
            return; 
        }
    };

    if target_visual_number == 0 {
        eprintln!("Error: Desktops start at 1.");
        return;
    }

    // 3. CHECK AND CREATE MISSING DESKTOPS
    let current_count = get_desktop_count().unwrap_or(1);

    if target_visual_number > current_count {
        let needed = target_visual_number - current_count;
        println!("Target is Desktop {}, but only {} exist. Creating {} new desktop(s)...", 
            target_visual_number, current_count, needed);

        for _ in 0..needed {
            match create_desktop() {
                Ok(_) => {},
                Err(_) => eprintln!("Warning: Failed to create a new desktop."),
            }
        }
    }

    // Convert to 0-based index
    let target_index = target_visual_number - 1;

    // 4. EXECUTE ACTION
    if mode == "move" {
        unsafe {
            let window_handle = GetForegroundWindow();
            let desktop_obj = Desktop::from(target_index);
            
            match move_window_to_desktop(desktop_obj, &window_handle) {
                Ok(_) => {
                    // Follow the window to the new desktop
                    let _ = switch_desktop(target_index);
                },
                Err(_) => eprintln!("Failed to move window."),
            }
        }
    } else {
        // Just switch
        match switch_desktop(target_index) {
            Ok(_) => println!("Switched to Desktop {}", target_visual_number),
            Err(_) => eprintln!("Failed to switch desktops."),
        }
    }
}
