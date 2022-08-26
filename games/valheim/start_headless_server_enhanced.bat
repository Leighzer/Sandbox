@echo off
set SteamAppId=892970

REM This bat runs steamcmd.exe a few directories up to update the server to latest
REM It then backs up saves/characters in AppData\LocalLow to a backup folder
REM Finally starts up the server

..\..\..\steamcmd.exe +login anonymous +app_update 896660 +quit

echo :
echo :
echo :
echo Backing up Valheim files...
echo :
echo :
echo :
REM https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/xcopy
REM /E Copies all subdirectories, even if they are empty. Use /e with the /s and /t command-line options.
REM /I causes xcopy to assume that the destination is a directory if the source is more than one file or a directory
REM /S Copies directories and subdirectories, unless they are empty. If you omit /s, xcopy works within a single directory.
REM /F Displays source and destination file names while copying.
REM /Y Suppresses prompting to confirm that you want to overwrite an existing destination file.
Xcopy /E /I /S /F /Y "%AppData%\..\LocalLow\IronGate\Valheim\" "C:\ValheimBackup\"

echo "Starting server PRESS CTRL-C to exit"

REM Tip: Make a local copy of this script to avoid it being overwritten by steam.
REM NOTE: Minimum password length is 5 characters & Password cant be in the server name.
REM NOTE: You need to make sure the ports 2456-2458 is being forwarded to your server through your local router & firewall.
valheim_server -nographics -batchmode -name "YOUR_SERVER_NAME" -port 2456 -world "YOUR_WORLD_NAME" -password "YOUR_PASSWORD"

