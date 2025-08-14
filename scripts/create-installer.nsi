# TriageIR NSIS Installer Script
# Creates a professional Windows installer for TriageIR

!define PRODUCT_NAME "TriageIR"
!define PRODUCT_VERSION "1.0.0"
!define PRODUCT_PUBLISHER "TriageIR Development Team"
!define PRODUCT_WEB_SITE "https://github.com/triageir/triageir"
!define PRODUCT_DIR_REGKEY "Software\Microsoft\Windows\CurrentVersion\App Paths\triageir-cli.exe"
!define PRODUCT_UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}"
!define PRODUCT_UNINST_ROOT_KEY "HKLM"

# Modern UI
!include "MUI2.nsh"

# General
Name "${PRODUCT_NAME} ${PRODUCT_VERSION}"
OutFile "TriageIR-Setup.exe"
InstallDir "$PROGRAMFILES64\TriageIR"
InstallDirRegKey HKLM "${PRODUCT_DIR_REGKEY}" ""
ShowInstDetails show
ShowUnInstDetails show
RequestExecutionLevel admin

# Interface Settings
!define MUI_ABORTWARNING
!define MUI_ICON "${NSISDIR}\Contrib\Graphics\Icons\modern-install.ico"
!define MUI_UNICON "${NSISDIR}\Contrib\Graphics\Icons\modern-uninstall.ico"

# Welcome page
!insertmacro MUI_PAGE_WELCOME

# License page
!define MUI_LICENSEPAGE_TEXT_TOP "Please review the license terms before installing ${PRODUCT_NAME}."
!insertmacro MUI_PAGE_LICENSE "LICENSE.txt"

# Components page
!insertmacro MUI_PAGE_COMPONENTS

# Directory page
!insertmacro MUI_PAGE_DIRECTORY

# Start menu page
var ICONS_GROUP
!define MUI_STARTMENUPAGE_NODISABLE
!define MUI_STARTMENUPAGE_DEFAULTFOLDER "TriageIR"
!define MUI_STARTMENUPAGE_REGISTRY_ROOT "${PRODUCT_UNINST_ROOT_KEY}"
!define MUI_STARTMENUPAGE_REGISTRY_KEY "${PRODUCT_UNINST_KEY}"
!define MUI_STARTMENUPAGE_REGISTRY_VALUENAME "NSIS:StartMenuDir"
!insertmacro MUI_PAGE_STARTMENU Application $ICONS_GROUP

# Installation page
!insertmacro MUI_PAGE_INSTFILES

# Finish page
!define MUI_FINISHPAGE_RUN "$INSTDIR\TriageIR-GUI.exe"
!define MUI_FINISHPAGE_SHOWREADME "$INSTDIR\README.md"
!insertmacro MUI_PAGE_FINISH

# Uninstaller pages
!insertmacro MUI_UNPAGE_INSTFILES

# Language files
!insertmacro MUI_LANGUAGE "English"

# Reserve files
!insertmacro MUI_RESERVEFILE_LANGDLL

# Installer sections
Section "TriageIR CLI Engine" SEC01
  SectionIn RO
  SetOutPath "$INSTDIR\CLI"
  SetOverwrite ifnewer
  File /r "CLI\*.*"
  
  # Create CLI wrapper script
  FileOpen $0 "$INSTDIR\triageir-cli.bat" w
  FileWrite $0 "@echo off$\r$\n"
  FileWrite $0 "cd /d $\"$INSTDIR\CLI$\"$\r$\n"
  FileWrite $0 "triageir-cli.exe %*$\r$\n"
  FileClose $0
SectionEnd

Section "TriageIR GUI Interface" SEC02
  SetOutPath "$INSTDIR\GUI"
  SetOverwrite ifnewer
  File /r "GUI\*.*"
  
  # Create GUI wrapper script
  FileOpen $0 "$INSTDIR\TriageIR-GUI.bat" w
  FileWrite $0 "@echo off$\r$\n"
  FileWrite $0 "cd /d $\"$INSTDIR\GUI$\"$\r$\n"
  FileWrite $0 "start $\"$\" $\"TriageIR.exe$\"$\r$\n"
  FileClose $0
SectionEnd

Section "Documentation" SEC03
  SetOutPath "$INSTDIR\docs"
  SetOverwrite ifnewer
  File /r "docs\*.*"
  
  SetOutPath "$INSTDIR"
  File "README.md"
  File "INSTALL.md"
  File "VERSION.txt"
  File "CLI-README.md"
  File "GUI-README.md"
  File "CLI-USAGE.md"
  File "CLI-PERFORMANCE.md"
SectionEnd

Section "Test Scripts and Examples" SEC04
  SetOutPath "$INSTDIR\test-scripts"
  SetOverwrite ifnewer
  File /r "test-scripts\*.*"
  
  SetOutPath "$INSTDIR\examples"
  SetOverwrite ifnewer
  File /r "examples\*.*"
SectionEnd

Section -AdditionalIcons
  SetOutPath $INSTDIR
  !insertmacro MUI_STARTMENU_WRITE_BEGIN Application
  CreateDirectory "$SMPROGRAMS\$ICONS_GROUP"
  CreateShortCut "$SMPROGRAMS\$ICONS_GROUP\TriageIR CLI.lnk" "$INSTDIR\triageir-cli.bat" "" "$INSTDIR\CLI\triageir-cli.exe"
  CreateShortCut "$SMPROGRAMS\$ICONS_GROUP\TriageIR GUI.lnk" "$INSTDIR\TriageIR-GUI.bat" "" "$INSTDIR\GUI\TriageIR.exe"
  CreateShortCut "$SMPROGRAMS\$ICONS_GROUP\User Manual.lnk" "$INSTDIR\docs\USER_MANUAL.md"
  CreateShortCut "$SMPROGRAMS\$ICONS_GROUP\Uninstall.lnk" "$INSTDIR\uninst.exe"
  !insertmacro MUI_STARTMENU_WRITE_END
SectionEnd

Section -Post
  WriteUninstaller "$INSTDIR\uninst.exe"
  WriteRegStr HKLM "${PRODUCT_DIR_REGKEY}" "" "$INSTDIR\CLI\triageir-cli.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayName" "$(^Name)"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "UninstallString" "$INSTDIR\uninst.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayIcon" "$INSTDIR\CLI\triageir-cli.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayVersion" "${PRODUCT_VERSION}"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "URLInfoAbout" "${PRODUCT_WEB_SITE}"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "Publisher" "${PRODUCT_PUBLISHER}"
  
  # Add to PATH
  EnVar::SetHKLM
  EnVar::AddValue "PATH" "$INSTDIR"
SectionEnd

# Section descriptions
!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
  !insertmacro MUI_DESCRIPTION_TEXT ${SEC01} "The core TriageIR CLI engine for forensic data collection. This component is required."
  !insertmacro MUI_DESCRIPTION_TEXT ${SEC02} "The graphical user interface for TriageIR. Provides an intuitive way to run scans and view results."
  !insertmacro MUI_DESCRIPTION_TEXT ${SEC03} "Complete documentation including user manual, developer guide, and API reference."
  !insertmacro MUI_DESCRIPTION_TEXT ${SEC04} "Test scripts, validation tools, and usage examples."
!insertmacro MUI_FUNCTION_DESCRIPTION_END

Function un.onUninstSuccess
  HideWindow
  MessageBox MB_ICONINFORMATION|MB_OK "$(^Name) was successfully removed from your computer."
FunctionEnd

Function un.onInit
  MessageBox MB_ICONQUESTION|MB_YESNO|MB_DEFBUTTON2 "Are you sure you want to completely remove $(^Name) and all of its components?" IDYES +2
  Abort
FunctionEnd

Section Uninstall
  !insertmacro MUI_STARTMENU_GETFOLDER "Application" $ICONS_GROUP
  Delete "$INSTDIR\uninst.exe"
  Delete "$INSTDIR\triageir-cli.bat"
  Delete "$INSTDIR\TriageIR-GUI.bat"
  Delete "$INSTDIR\README.md"
  Delete "$INSTDIR\INSTALL.md"
  Delete "$INSTDIR\VERSION.txt"
  Delete "$INSTDIR\CLI-README.md"
  Delete "$INSTDIR\GUI-README.md"
  Delete "$INSTDIR\CLI-USAGE.md"
  Delete "$INSTDIR\CLI-PERFORMANCE.md"

  RMDir /r "$INSTDIR\CLI"
  RMDir /r "$INSTDIR\GUI"
  RMDir /r "$INSTDIR\docs"
  RMDir /r "$INSTDIR\test-scripts"
  RMDir /r "$INSTDIR\examples"

  Delete "$SMPROGRAMS\$ICONS_GROUP\Uninstall.lnk"
  Delete "$SMPROGRAMS\$ICONS_GROUP\User Manual.lnk"
  Delete "$SMPROGRAMS\$ICONS_GROUP\TriageIR GUI.lnk"
  Delete "$SMPROGRAMS\$ICONS_GROUP\TriageIR CLI.lnk"

  RMDir "$SMPROGRAMS\$ICONS_GROUP"
  RMDir "$INSTDIR"

  DeleteRegKey ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}"
  DeleteRegKey HKLM "${PRODUCT_DIR_REGKEY}"
  
  # Remove from PATH
  EnVar::SetHKLM
  EnVar::DeleteValue "PATH" "$INSTDIR"
  
  SetAutoClose true
SectionEnd