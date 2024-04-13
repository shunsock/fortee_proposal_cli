SOURCE_PATH=bin/fortee
DEFAULT_COMMAND_PATH=/usr/local/bin/fortee

# Delete command
echo "[Fortee Cli Uninstaller] delete $DEFAULT_COMMAND_PATH ..."
sudo rm -rf $DEFAULT_COMMAND_PATH
if [ $? -ne 0 ]; then
  echo "[Fortee Cli Uninstaller] Failed to uninstall!"
  exit 1
fi

# Delete hidden file
echo "[Fortee Cli Uninstaller] delete hidden file ~/fortee ..."
rm -rf ~/.fortee
if [ $? -ne 0 ]; then
  echo "[Fortee Cli Uninstaller] Failed to uninstall!"
  exit 1
fi

# Finish
echo "[Fortee Cli Uninstaller] uninstalled successfully!"
