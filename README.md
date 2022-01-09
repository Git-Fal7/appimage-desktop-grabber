# rust branch
this is just the exact same thing, but made with rust.

# appimage-desktop-grabber
a little handy tool to grab .desktop files in .appimage files

# using it
``cargo --build release \
cd target/release \
./appimage-desktop (your appimage file)``

# speeds?
well, both of them are small that it doesn't matter, so here you go.

```
Launching from ~
BASH
$ ~/ time ./grab-desktop.sh ~/firefox.AppImage
real	0m2.631s
user	0m1.531s
sys	0m0.330s

RUST
$ ~/ time ./grab-desktop ~/firefox.AppImage
real	0m4.882s
user	0m1.536s
sys	0m0.405s
```
As you see, shell scripts are apparently faster, this is expected, however, when you put both of them in ``/tmp``, it becomes appearant that shell scripts are winning due to using of ``/tmp`` to extract the appimages.
```
Launching from /tmp
BASH
$ /tmp time ./grab-desktop.sh ~/firefox.AppImage
real	0m1.958s
user	0m1.641s
sys	0m0.258s

RUST
$ /tmp time ./grab-desktop ~/firefox.AppImage
real	0m1.762s
user	0m1.555s
sys	0m0.179s
```
``/tmp`` can be useful to use since almost all appimages are small that you can extract it there. and thats where the rust version will be more viable.
