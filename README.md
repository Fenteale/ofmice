# OF Mice
The Open Fortress launcher.

## Building
there's a --feature steam_wrangler that requires the steam sdk to be installed and env var `STEAM_SDK_LOCATION` to be set to it.

TODO: Other miscellanious system dependencies
(use ldd to find out what libs it uses)

### Building in Windows
To build in windows, you need to install gtk through [MSYS2](https://www.msys2.org/).
The MSYS packages needed are `mingw-w64-x86_64-gtk3` `mingw-w64-x86_64-toolchain`  and `mingw-w64-x86_64-clang`. You can install them with the following command:
```
pacman -S mingw-w64-x86_64-gtk3 mingw-w64-x86_64-toolchain mingw-w64-x86_64-clang
```

Also, there are a few environment variables that need to be set.  `GTK_LIB_DIR` needs to be set where the gtk package installed its libraries.  By default this is `C:\msys64\mingw64\lib`.  As mentioned above, `STEAM_SDK_LOCATION` must be set to use steam_wrangler.  In addition, your `PATH` should include the path to MSYS's mingw binaries.  By default this is `C:\msys64\mingw64\bin`.

After that is setup, to build and run the project, you might need to add `--target=x86_64-pc-windows-gnu` to the cargo options like so:
```
cargo run --target=x86_64-pc-windows-gnu
```

If for some reason it's not letting you compile for that target, you may have to run the command:
```
rustup target add x86_64-pc-windows-gnu
```

## Finished
* Basic GTK skeleton
* Pull SSDK path from steamworks and ensure TF2 is installed.
* Download index, clean install, patch apply
* run game
* Launch options
* Progress bar (module, with push/pop obj that gets passed around)

## To-Do
### Backend things
* installation::InstalledBin - Integrity checksum
* Date (last successful update or up-to-date, build date)
* Index generation and patch trimming (bin)
* Write bash script that generates patches and tarballs from svn
    (basic patch logic already complete)
* Modify gameinfo.txt to hard-code TF2 Location. (hashsum it after)
* `ofserver` binary that is basically `ofmice` but for dedicated servers

### UI things
* disable launch button if it outlook is bad
* investigate russian translation
* make button look like update button when updates needed
* add a close button and disable decorations with https://developer.gnome.org/gtk3/stable/GtkWindow.html#gtk-window-set-decorated

### distribution things
* Launcher self update detection
* Double check the licenses of the crates and steamworks.
* Double check builds are for i686 rather than x64
* (Cross?) Build Windows binaries
