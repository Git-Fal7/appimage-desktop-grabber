# appimage-desktop-grabber
a little handy tool to grab .desktop files in .appimage files

# using it
first you have to build the rust bin. which is used to replace lines (its just used for replacing output of a file, you can replace anything really).

``cargo build --release``

then you grab the bin to the folder which the bash file is in it.

``mv target/release/appimage-desktop ./``

then you execute the bash file

``./grab-desktop.sh (your appimage location)``

and it should put the .desktop file onto your ~/.local/share/application. redirecting all executable to the appimage location.

# rust version
i made a branch that has the [rust version](https://github.com/Git-Fal7/appimage-desktop-grabber/tree/full-rust) to make it just one binary.
