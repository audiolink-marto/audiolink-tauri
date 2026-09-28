// Piloto AudioLink Ensayo — ventana Tauri única, sin barra de navegador,
// que carga la hoja ensayo.html ya publicada en GitHub Pages. Firebase Auth,
// Firestore y Cloudinary funcionan igual: son llamadas HTTPS normales,
// no dependen de que haya un navegador visible alrededor.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error corriendo la app de AudioLink");
}
