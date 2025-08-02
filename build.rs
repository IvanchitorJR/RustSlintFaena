use std::fs;
use std::path::Path;

fn main() {
    // Ruta donde cargo-apk genera el manifest
    let target_manifest = Path::new("target/debug/apk/AndroidManifest.xml");

    // Tu manifest personalizado
    let custom_manifest = Path::new("android/AndroidManifest.xml");

    // Copiar si ya existe el generado
    if target_manifest.exists() {
        println!("cargo:warning=Overwriting AndroidManifest.xml...");
        fs::copy(custom_manifest, target_manifest)
            .expect("No se pudo copiar el AndroidManifest.xml personalizado");
    }

    // Recompilar si el manifest cambia
    println!("cargo:rerun-if-changed=android/AndroidManifest.xml");

    // Compilar slint
    slint_build::compile("ui/pages/main.slint").unwrap();
}
