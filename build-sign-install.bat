@echo off
setlocal

REM === Ruta del proyecto descompilado ===
set UNPACK_DIR=apk_unpack

REM === Nombre del APK de salida ===
set OUTPUT_APK=faena-mod.apk
set FINAL_APK=faena-final.apk

REM === Clave de firma ===
set KEYSTORE=my-release-key.jks
set ALIAS=my-key-alias

echo.
echo [1/4] Recompilando APK...
apktool b %UNPACK_DIR% -o %OUTPUT_APK%
if errorlevel 1 (
    echo ❌ Error al recompilar el APK.
    pause
    exit /b 1
)

echo.
echo [2/4] Firmando APK...
apksigner sign --ks %KEYSTORE% --ks-key-alias %ALIAS% --out %FINAL_APK% %OUTPUT_APK%
if errorlevel 1 (
    echo ❌ Error al firmar el APK.
    pause
    exit /b 1
)

echo.
echo [3/4] Instalando en el dispositivo (si está conectado)...
adb install -r %FINAL_APK%
if errorlevel 1 (
    echo ⚠️ No se pudo instalar automáticamente. Hazlo manualmente si lo necesitas.
)

echo.
echo ✅ Proceso completado.
pause