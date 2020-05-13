# bindings

A program written in Rust which contains hardcoded global keybindings that work across X.

Table of Contents:<br>
[Usage](#usage)<br>
[Modification](#modification)<br>
[Modification/Finding the keysym of the key you want to bind](#how-to-find-the-keysym-of-the-key-you-want-to-bind)<br>
[Modification/How to bind the key and make it do something](#how-to-actually-bind-the-key-and-make-it-do-something)<br>
[Thanks!](#thanks)

## Usage

After modifying to your liking, `cargo build` and `sudo ./target/debug/bindings` to check that it works.<br>

**Due to the way `libinput` works, the program must be run as root.**<br>

After you think you have it the way you like it, `cargo build --release`<br>
and either move `./target/release/bindings` to somewhere in your `$PATH`<br>
or `cargo install --path .` (inside the root dir of the git repo) and ensure<br>
`$HOME/.cargo/bin` is in your `$PATH`.

Note: Has the same deps as `inputbot`, so please check out [their repo](https://github.com/obv-mikhail/InputBot).

## Modification

You'll need to modify the source to fit your needs. Currently, it is<br>
set up for my usage which is setting the XF86Audio keys to run my<br>
specified `mpc` commands.

### How to find the keysym of the key you want to bind

I use the program called `xev`. When you run it, a big blank window should open.<br>
When it does, press the key you want to find the keysym of (I'll use `XF86AudioPlay`)<br>
for this example. After that, close the window and look through the output. You'll see<br>
a lot of it, but the main section you want to look for will look like this:
```
KeyPress event, serial 35, synthetic NO, window 0x1400001,
    root 0x294, subw 0x416ee3, time 688908125, (184,95), root:(184,95),
    state 0x10, keycode 172 (keysym 0x1008ff14, XF86AudioPlay), same_screen YES,
    XLookupString gives 0 bytes: 
    XmbLookupString gives 0 bytes: 
    XFilterEvent returns: False
```
The part you'd want is the `0x1008ff14` part of `keysym 0x1008ff14` (or the equivalent for your key anyways).

### How to actually bind the key and make it do something

Now here's the part you've been waiting for, how to bind the key and how to make it<br>
do something (for example, run the command `mpc toggle`).

You need to use `OtherKey` since `inputbot` doesn't currently support media keys yet,<br>
so to just bind it you'd need to use:
```rust
use inputbot::handle_input_events;
use inputbot::KeybdKey::*;

fn main() {
    OtherKey(0x1008ff14).bind(||);
	handle_input_events();
}
```
Of course, this on it's own will fail since there is nothing for it to actuall do inside of `.bind(||)`.<br>
Now to make it actually run `mpc toggle`, you'd need to do:
```rust
use inputbot::handle_input_events;
use inputbot::KeybdKey::*;
use std::process::Command;

fn main() {
    OtherKey(0x1008ff14).bind(|| { Command::new("mpc").arg("toggle").output(); });
	handle_input_events();
}
```
And there we go. After compiling and running, you should be able to press `Xf86AudioPlay`<br>
and have it run the command `mpc toggle`.

## Thanks

This program was possible because of [inputbot](https://github.com/obv-mikhail/InputBot), a library<br>
for creating keybindings among other things.
