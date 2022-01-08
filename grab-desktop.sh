#!/bin/sh

if [ -f "$1" ]; then
 echo "file $1 is a real file"
else
 echo "file doesnt exist"
fi
echo "creating a new file"
ogDIRNAME=`pwd`
mkdir /tmp/appimagedesktop/
cd /tmp/appimagedesktop/
DESKTOPFILENAME=`$1 --appimage-extract | grep ".desktop"`
NAMEEXEC=`cat ${DESKTOPFILENAME} | grep "Exec=" | cut -b 6- | cut -d' ' -f 1`
varEXEC=`echo ${NAMEEXEC} | cut -d' ' -f 1`
cd ${ogDIRNAME}
./appimage-desktop /tmp/appimagedesktop/${DESKTOPFILENAME} Exec=${varEXEC} Exec=${1} > ~/.local/share/applications/${1##*/}.desktop

#awk '/Exec=/{gsub(/"'"${varEXEC}"'"/, ""'"${fullfile}"'"")};{print}' ${DESKTOPFILENAME} > /tmp/onword
#cat ${DESKTOPFILENAME} | sed "s/Exec=$\{varExec}\/Exec=$\{fullfile\}/g" > ~/.local/share/applications/${FILENAME}.desktop
