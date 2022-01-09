#!/bin/sh
# if arg1 is a file
if [ ! -f "$1" ]; then
 echo "file doesnt exist"
 exit
fi

#checks if applications & pixmaps folder are avaliable.
if [ ! -d "~/.local/share/applications" ]; then
 mkdir ~/.local/share/applications
fi
if [ ! -d "~/.local/share/pixmaps" ]; then
 mkdir ~/.local/share/pixmaps
fi

ogDIRNAME=`pwd`

#creates a folder to extract the appimage in it
mkdir /tmp/appimagedesktop/
cd /tmp/appimagedesktop/

#extracts the appimage
outpu=`$1 --appimage-extract`
echo "done extracting"

cd /tmp/appimagedesktop/squashfs-root

#get every file in /tmp/appimagedesktop/squashfs-root that ends with .desktop
DESKTOPFILENAME=squashfs-root/`ls -a | grep "\.desktop$"`
PNGFILENAME=squashfs-root/`ls -a | grep "\.png$"`
SVGFILENAME=squashfs-root/`ls -a | grep "\.svg$"`

# get png if there is
cp /tmp/appimagedesktop/${PNGFILENAME} ~/.local/share/pixmaps/
# get svg if there is
cp /tmp/appimagedesktop/${SVGFILENAME} ~/.local/share/pixmaps/

#cat into the desktop file and get the executable to replace
NAMEEXEC=`cat ${DESKTOPFILENAME} | grep "Exec=" | cut -b 6- | cut -d' ' -f 1`
#used for preventing any duplicates from grep
varEXEC=`echo ${NAMEEXEC} | cut -d' ' -f 1`

cd ${ogDIRNAME}

#use the bin that is built eariler
./appimage-desktop /tmp/appimagedesktop/${DESKTOPFILENAME} Exec=${varEXEC} Exec=${1} > ~/.local/share/applications/${1##*/}.desktop

#remove everything
rm -rf /tmp/appimagedesktop/
