@echo off
chcp 65001 > nul

:: Ruta del proyecto
set PROJECT_DIR=%~dp0
set APK_NAME=faena
set APK_ORIGINAL=%PROJECT_DIR%target\debug\apk\%APK_NAME%.apk
set APKTOOL=%PROJECT_DIR%apktool\apktool.bat
set APKTOOL_DIR=%PROJECT_DIR%build\apktool
set APK_UNPACKED=%APKTOOL_DIR%\%APK_NAME%
set APK_REBUILT=%APKTOOL_DIR%\%APK_NAME%_custom.apk
set APK_ALIGNED=%PROJECT_DIR%aligned.apk
set APK_SIGNED=%PROJECT_DIR%signed.apk
set CUSTOM_MANIFEST=%PROJECT_DIR%android\AndroidManifest.xml

setlocal EnableDelayedExpansion

if defined ANDROID_HOME (
    set SDK_DIR=%ANDROID_HOME%
) else if defined ANDROID_SDK_ROOT (
    set SDK_DIR=%ANDROID_SDK_ROOT%
) else (
    echo ERROR: No se encontró la variable ANDROID_HOME ni ANDROID_SDK_ROOT
    pause
    exit /b 1
)

for /f "tokens=* delims=" %%i in ('dir /b /ad "%SDK_DIR%\build-tools" ^| sort /r') do (
    set BUILD_TOOLS_PATH=%SDK_DIR%\build-tools\%%i
    if exist "!BUILD_TOOLS_PATH!\zipalign.exe" (
        goto :foundTools
    )
)

:foundTools
set ZIPALIGN=%BUILD_TOOLS_PATH%\zipalign.exe
set APKSIGNER=%BUILD_TOOLS_PATH%\apksigner.bat

if not exist "%ZIPALIGN%" (
    echo ERROR: zipalign.exe no encontrado.
    pause
    exit /b 1
)

if not exist "%APKSIGNER%" (
    echo ERROR: apksigner.bat no encontrado.
    pause
    exit /b 1
)


rd /S /Q "%APK_UNPACKED%"

echo 0 - CREANDO APK
cargo apk build || goto :error

echo 1 - DESEMPACANDO CON APKTOOL
call "%APKTOOL%" d -f "%APK_ORIGINAL%" -o "%APK_UNPACKED%" || goto :error

echo 1.5 - CORRIGIENDO apktool.yml
powershell -Command "$ymlPath='%APK_UNPACKED%\apktool.yml'; if (Test-Path $ymlPath) { $content=Get-Content $ymlPath; $startIndex=$content.IndexOf('doNotCompress:'); if ($startIndex -ge 0) { $before=$content[0..$startIndex]; $after=$content[($startIndex+1)..($content.Length-1)]; $filteredAfter=$after | Where-Object { $_ -notmatch '^-.*' }; $newList=@('  - resources.arsc', '  - lib/arm64-v8a/libfaena.so'); $newContent=$before + $newList + $filteredAfter; $newContent | Set-Content $ymlPath -Encoding UTF8; Write-Host 'apktool.yml corregido.' } else { Write-Host 'No se encontró doNotCompress en apktool.yml' } } else { Write-Host 'No se encontró apktool.yml' }"

echo 2 - REEMPLAZANDO MANIFEST
copy /Y "%CUSTOM_MANIFEST%" "%APK_UNPACKED%\AndroidManifest.xml" || goto :error

echo 3 - COPIANDO res/
xcopy /E /I /Y "%PROJECT_DIR%android\res" "%APK_UNPACKED%\res"

echo 4 - RECOMPILANDO APK
call "%APKTOOL%" b --use-aapt2 "%APK_UNPACKED%" -o "%APK_REBUILT%" || goto :error

echo 5 - ALINEANDO APK
"%ZIPALIGN%" -f -p 4 "%APK_REBUILT%" "%APK_ALIGNED%" || goto :error

echo 6 - FIRMANDO APK
call "%APKSIGNER%" sign --ks my-release-key.jks --ks-key-alias my-key-alias --ks-pass pass:123456 --key-pass pass:123456 --out "%APK_SIGNED%" "%APK_ALIGNED%" || goto :error

echo ✅ APK firmada: %APK_SIGNED%
adb install -r "%APK_SIGNED%"
pause
exit /b 0

:error
echo ❌ ERROR en el proceso
pause
exit /b 1
