#!/bin/bash
# Must run with sudo, ie as root user

# kill stupid first-login wizard to prevent the screen preader's audio prompts
kill $(ps aux | grep '[p]iwiz' | awk '{print $2}')

# prevent wizard from loading in future
rm /etc/xdg/autostart/piwiz.desktop

# Update repos
apt update -y
apt upgrade -y

# prevent samba install from prompting and pausing install script
echo "samba-common samba-common/workgroup string  WORKGROUP" | sudo debconf-set-selections
echo "samba-common samba-common/dhcp boolean true" | sudo debconf-set-selections
echo "samba-common samba-common/do_debconf boolean true" | sudo debconf-set-selections

# install dependencies
apt install -y libasound2-dev libudev-dev samba

# move binary
mkdir -p /home/pi/button-board/vocabulary
cp ./bin/* /home/pi/button-board
chmod -R 0777 /home/pi/button-board

# setup systemd service & disable lxpanel for resource hogging
cp scripts/LXDE-autostart/autostart /etc/xdg/lxsession/LXDE-pi/autostart
cp scripts/systemd/button-board.service /etc/systemd/system/button-board.service

# make samba shares writeable
sed -i -e 's/read only = yes/read only = no/g' /etc/samba/smb.conf

# make pi user able to login
echo -ne "raspberry\nraspberry\n" | smbpasswd -a -s pi

systemctl restart smbd

# add startup daemon
systemctl daemon-reload
systemctl enable button-board
systemctl start button-board

# Set ALSA to Headphones:
cp scripts/alsa-config /home/pi/.asoundrc

# Set Pulse Audio to Headphones
echo "set-default-sink 1" >> /etc/pulse/default.pa 

# Force HDMI output so we have audio even if no HDMI is connected.
sed -i -e 's/#hdmi_force_hotplug=1/hdmi_force_hotplug=1/g' /boot/config.txt
sed -i -e 's/#hdmi_drive=2/hdmi_drive=2/g' /boot/config.txt

echo "Dont forget to copy your audio files into: /home/pi/button-board/vocabulary"
echo "Then reboot to finish your setup.."
