@echo off
chcp 65001 > nul
echo =============================
echo     CONFIGURACIÓN INICIAL
echo =============================

:: CONFIGURACIÓN DEL PROYECTO
set PROJECT_DIR=%~dp0
set APK_NAME=faena
set APK_ORIGINAL=%PROJECT_DIR%target\debug\apk\%APK_NAME%.apk
set APKTOOL=C:\Users\carlos\projects\apktool\apktool.bat
set APKTOOL_DIR=%PROJECT_DIR%build\apktool
set APK_UNPACKED=%APKTOOL_DIR%\%APK_NAME%
set APK_REBUILT=%APKTOOL_DIR%\%APK_NAME%_custom.apk
set APK_ALIGNED=%PROJECT_DIR%aligned.apk
set APK_SIGNED=%PROJECT_DIR%signed.apk
set CUSTOM_MANIFEST=%PROJECT_DIR%android\AndroidManifest.xml

:: Aquí sin comillas
set ZIPALIGN=C:\Users\Carlos\AppData\Local\Android\Sdk\build-tools\35.0.0\zipalign.exe
set APKSIGNER=C:\Users\Carlos\AppData\Local\Android\Sdk\build-tools\35.0.0\apksigner.bat

echo APK_ORIGINAL = %APK_ORIGINAL%
echo APK_UNPACKED = %APK_UNPACKED%
echo APKTOOL= %APKTOOL%

pause

rd /S /Q "%APK_UNPACKED%"

echo ============================================
echo     1. DESCOMPRIMIENDO APK CON APKTOOL
echo ============================================
echo Ejecutando: "%APKTOOL%" d -f "%APK_ORIGINAL%" -o "%APK_UNPACKED%"
call "%APKTOOL%" d -f "%APK_ORIGINAL%" -o "%APK_UNPACKED%"
if errorlevel 1 (
    echo ❌ ERROR al descomprimir el APK.
    pause
    exit /b 1
)

powershell -Command "(Get-Content '%APK_UNPACKED%\apktool.yml') -replace 'doNotCompress:.*', 'doNotCompress:`n  - resources.arsc' | Set-Content '%APK_UNPACKED%\apktool.yml'"


echo ============================================
echo     2. REEMPLAZANDO AndroidManifest.xml
echo ============================================
copy /Y "%CUSTOM_MANIFEST%" "%APK_UNPACKED%\AndroidManifest.xml"
if errorlevel 1 (
    echo ❌ ERROR al copiar el manifest.
    pause
    exit /b 1
)

echo COPIANDO CARPETA RES
xcopy /E /I /Y "C:\Users\Carlos\projects\slint\appfaena\rustslintfaena\android\res" "C:\Users\Carlos\projects\slint\appfaena\RustSlintFaena\build\apktool\faena\res"

echo ============================================
echo     3. REEMPACANDO APK CON APKTOOL (usando AAPT2)
echo ============================================

call "%APKTOOL%" b --use-aapt2 "%APK_UNPACKED%" -o "%APK_REBUILT%"

if errorlevel 1 (
    echo ❌ ERROR al reconstruir el APK.
    pause
    exit /b 1
)

echo ============================================
echo     4. ALINEANDO APK CON ZIPALIGN
echo ============================================
echo Ejecutando: "%ZIPALIGN%" -f -p 4 "%APK_REBUILT%" "%APK_ALIGNED%"
"%ZIPALIGN%" -f -p 4 "%APK_REBUILT%" "%APK_ALIGNED%"
if errorlevel 1 (
    echo ❌ ERROR al alinear el APK.
    pause
    exit /b 1
)

echo APK_ALIGNED = %APK_ALIGNED%
echo APK_SIGNED = %APK_SIGNED%
echo APKSIGNER = %APKSIGNER%
echo ============================================
echo     5. FIRMANDO APK CON APKSIGNER
echo ============================================
echo Ejecutando: "%APKSIGNER%" sign --ks my-release-key.jks --ks-key-alias my-key-alias --ks-pass pass:123456 --key-pass pass:123456 --out "%APK_SIGNED%" "%APK_ALIGNED%"

call "%APKSIGNER%" sign --ks my-release-key.jks --ks-key-alias my-key-alias --ks-pass pass:123456 --key-pass pass:123456 --out "%APK_SIGNED%" "%APK_ALIGNED%"
if errorlevel 1 (
    echo ❌ ERROR al firmar el APK.
    pause
    exit /b 1
)

echo ✅ APK firmado correctamente: %APK_SIGNED%
echo INSTALANDO...
adb install -r C:\Users\Carlos\projects\slint\appfaena\RustSlintFaena\signed.apk
pause
