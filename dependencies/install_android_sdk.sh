# Create the SDK root directory
mkdir -p $HOME/android-sdk/cmdline-tools

# Download the tools (update the version number if a newer one is available)
curl -o cmdline-tools.zip https://dl.google.com/android/repository/commandlinetools-linux-14742923_latest.zip

# Unzip and move to the 'latest' subdirectory
unzip cmdline-tools.zip
mv cmdline-tools $HOME/android-sdk/cmdline-tools/latest
rm cmdline-tools.zip
