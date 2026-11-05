; Lintel Windows installer — Inno Setup script
; Build with:  iscc lintel.iss
;
; This script is checked into the repo so a human (or CI) can build the
; Windows installer from the release zip contents.

#define MyAppName "Lintel"
#define MyAppVersion "0.5.0"
#define MyAppPublisher "Perchemop"
#define MyAppExeName "lintel.exe"
#define MyAppURL "https://github.com/Perchemop/lintel-desktop"

[Setup]
AppId={0F9A1E2C-4D5B-4C6A-8E9F-1D2A3B4C5D6E}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
LicenseFile=LICENSE-MIT
OutputDir=output
OutputBaseFilename=lintel-windows-x86_64-setup
SetupIconFile=lintel.png
Compression=lzma
SolidCompression=yes
PrivilegesRequired=admin
ChangesAssociations=no

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "staging\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{commondesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent
