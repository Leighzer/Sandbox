REM https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/xcopy
REM /E Copies all subdirectories, even if they are empty. Use /e with the /s and /t command-line options.
REM /I causes xcopy to assume that the destination is a directory if the source is more than one file or a directory
REM /S Copies directories and subdirectories, unless they are empty. If you omit /s, xcopy works within a single directory.
REM /F Displays source and destination file names while copying.
REM /Y Suppresses prompting to confirm that you want to overwrite an existing destination file.
Xcopy /E /I /S /F /Y "%AppData%\..\LocalLow\IronGate\Valheim\" "C:\ValheimBackup\"