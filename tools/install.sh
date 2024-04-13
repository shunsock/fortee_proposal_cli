SOURCE_PATH=bin/fortee
DEFAULT_COMMAND_PATH=/usr/local/bin/fortee

# Start installation
echo "[Fortee Cli Installer] Installing command to $DEFAULT_COMMAND_PATH ..."

mkdir -p ~/.fortee
mkdir -p ~/.fortee/html
mkdir -p ~/.fortee/image
mkdir -p ~/.fortee/json
mkdir -p ~/.fortee/test
touch ~/.fortee/test/test.txt

# Copy Our Source Binary to Default Command Path
sudo cp $SOURCE_PATH $DEFAULT_COMMAND_PATH

# Check if command installed successfully
if [ $? -ne 0 ]; then
  echo "[Fortee Cli Installer] Failed to install command!"
  exit 1
fi
echo "[Fortee Cli Installer] Command installed successfully!"
