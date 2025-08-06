PROJECTO FAENA APLICACION MOVIL
-
-
-
Para poder hacer la APK se necesita ejecutar el script build_app.bat, el cual hace la APK para android y la instala ahi mismo.
-
Se necesita definir las siguiente variables de entorno (PowerShell para este caso): 
$env:JAVA_HOME
$env:ANDROID_HOME
$env:ANDROID_NDK_HOME
$env:NDK_HOME
-
Framework necesario para apktool desde un celular android:
apktool if framework-res.apk
