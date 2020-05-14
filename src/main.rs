// Global keybindings, created by Phate6660 when he couldn't figure out how to assign multimedia keys in EXWM.
// I will be adding on to this as I find more keys that are hard/impossible to assign in EXWM/Emacs.

use inputbot::handle_input_events;
use inputbot::KeybdKey::*;
use std::process::Command;

fn main() {
	// General keys
	OtherKey(0xff61).bind(|| { Command::new("/home/valley/scripts/general").arg("scr").output(); }); // <Print> -> `general scr`

	// Multimedia keys
	OtherKey(0x1008ff17).bind(|| { Command::new("mpc").arg("next").output(); });   // XF86AudioNext -> `mpc next`
	OtherKey(0x1008ff14).bind(|| { Command::new("mpc").arg("toggle").output(); }); // XF86AudioPlay -> `mpc toggle`
	OtherKey(0x1008ff16).bind(|| { Command::new("mpc").arg("prev").output(); });   // XF86AudioPrev -> `mpc prev`
	OtherKey(0x1008ff15).bind(|| { Command::new("mpc").arg("cdprev").output(); }); // XF86AudoStop -> `mpc cdprev`
	handle_input_events();
}
