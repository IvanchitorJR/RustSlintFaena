use std::fs;
use std::path::PathBuf;

fn main() {
    println!("Sobrescribiendo el manifest generado...");

    // Ruta al manifest modificado (personalizado)
    let manifest_personalizado = PathBuf::from("android/AndroidManifest.xml");

    // Ruta al manifest generado por cargo-apk
    let manifest_objetivo = PathBuf::from("target/debug/apk/AndroidManifest.xml");

    if !manifest_personalizado.exists() {
        eprintln!("❌ No se encontró el AndroidManifest personalizado.");
        std::process::exit(1);
    }

    if !manifest_objetivo.exists() {
        eprintln!("❌ No se encontró el AndroidManifest generado por cargo-apk.");
        std::process::exit(1);
    }

    match fs::copy(&manifest_personalizado, &manifest_objetivo) {
        Ok(_) => println!("✅ Manifest sobrescrito correctamente."),
        Err(e) => {
            eprintln!("❌ Error al sobrescribir el manifest: {}", e);
            std::process::exit(1);
        }
    }
}