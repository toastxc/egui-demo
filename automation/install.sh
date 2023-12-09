# compile packages
git clone https://github.com/toastxc/hypixel-gui.git
cd hypixel-gui
cargo b -r

sudo cp ./target/release/hypixel-gui /bin/hypixel-gui

# install application
echo "[Desktop Entry]
      Name=Hypixel
      Exec=sudo /bin/hypixel-gui
      Icon=/path/to/your/icon.png
      Type=Application
      Categories=Utility;
" | sudo tee /usr/share/applications/hypixel-gui.desktop > /dev/null

# update desktop
sudo update-desktop-database