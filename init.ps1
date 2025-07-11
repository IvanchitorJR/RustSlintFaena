# Configura el entorno
$env:JAVA_HOME = "C:\Program Files\Eclipse Adoptium\jdk-11.0.27.6-hotspot"
$env:ANDROID_HOME = "C:\Users\ivand\AppData\Local\Android\Sdk"
$env:ANDROID_PLATFORM = "android-30"
$env:ANDROID_BUILD_TOOLS_VERSION = "36.0.0"
$env:ANDROID_D8_JAR = "$env:ANDROID_HOME\build-tools\$env:ANDROID_BUILD_TOOLS_VERSION\d8.jar"
$env:Path = "$env:JAVA_HOME\bin;$env:ANDROID_HOME\platform-tools;$env:Path"

# Verifica herramientas
Write-Host "`n✅ JAVA Version:"
java -version

Write-Host "`n✅ ADB Devices:"
adb devices

Write-Host "`n🚀 Compilando e instalando APK..."
cargo apk run