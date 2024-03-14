SOURCE_PATH=target/release/fortee-cli
DEFAULT_COMMAND_PATH=/usr/local/bin/fortee-cli

# Start installation
echo "[Fortee Cli Installer] Installing command to $DEFAULT_COMMAND_PATH ..."

# Copy Our Source Binary to Default Command Path
sudo cp $SOURCE_PATH $DEFAULT_COMMAND_PATH

# Check if command installed successfully
if [ $? -ne 0 ]; then
  echo "[Fortee Cli Installer] Failed to install command!"
  exit 1
fi
echo "[Fortee Cli Installer] Command installed successfully!"
