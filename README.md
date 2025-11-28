# desktop-switcher

# **Desktop Switcher for Windows (Rust \+ PowerToys)**

This guide explains how to build a lightweight, custom command-line tool to switch Virtual Desktops or move windows to specific desktops by number (e.g., Desktop 1, Desktop 2). We then use **Microsoft PowerToys** to trigger this tool with global hotkeys like Win+1.

## **Prerequisites**

1. **Rust**: Install from [rustup.rs](https://rustup.rs/) (default installation is fine).  
2. **PowerToys**: Install from the [Microsoft Store](https://apps.microsoft.com/store/detail/microsoft-powertoys/XP89DCGQ3K6VLD) or GitHub.

## **Part 1: Dowload or Build**

### **Download**
You can download the latest binary from [here](https://github.com/drlipton/desktop-switcher/releases/download/0.1.0/desktop_switcher.exe)
If using downloaded binary, please jump ahead to part 2.

### **Compile**

Run this command to build an optimized executable:  
cargo build \--release

### **Install**

1. Go to the target/release/ folder inside your project.  
2. Copy desktop\_switcher.exe to a permanent location (e.g., C:\\Tools\\desktop\_switcher.exe).

## **Part 2: Configure PowerToys**

We will now map hotkeys (like Win+1) to run your new tool.

1. Open **PowerToys** and select **Keyboard Manager** from the sidebar.  
2. Ensure "Enable Keyboard Manager" is **On**.  
3. Click **Remap a shortcut**.  
4. Click **\+ Add Shortcut Remapping**.

### **Setting up "Switch to Desktop 1" (Win+1)**

* **Physical Shortcut:** Click the pencil or type: Win \+ 1  
* **Action:** Select **Run Program** from the dropdown.  
* **App:** Paste the full path to your exe: C:\\Tools\\desktop\_switcher.exe  
* **Args:** Type: 1  
* **If Running:** Select **Start another instance**.  
* Click **OK**.

### **Setting up "Move Window to Desktop 1" (Win+Shift+1)**

* **Physical Shortcut:** Win \+ Shift \+ 1  
* **Action:** **Run Program**  
* **App:** C:\\Tools\\desktop\_switcher.exe  
* **Args:** Type: move 1  
* **If Running:** Select **Start another instance**.

Repeat these steps for Desktops 2, 3, 4, etc.

## **Troubleshooting**

* **Antivirus:** Since this is a custom executable that manipulates Windows internals, you may need to add an exclusion for desktop\_switcher.exe in Windows Defender if it gets flagged.  
* **Win Key Conflicts:** If Win+1 opens a Taskbar app instead of switching desktops, you can either unpin items from your Taskbar or use Alt+1 / Ctrl+1 for your hotkeys instead.
