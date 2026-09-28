# Piloto: AudioLink Ensayo standalone (Tauri)

Esto arma una ventana de Windows sin navegador visible que carga
`ensayo.html` directo desde tu sitio en GitHub Pages. Firebase Auth,
Firestore y Cloudinary funcionan igual que siempre — son llamadas
HTTPS normales, no dependen de que haya un navegador alrededor.

Este proyecto **no se compiló acá** (el entorno donde se armó no tiene
Rust ni el SDK de Windows). Hay dos formas de conseguir el `.exe`:

- **Sin instalar nada ni tocar código** (abajo, "Vía 0") — dejás que
  GitHub lo compile por vos, en la nube.
- **Compilando en tu propia PC con Windows** (más abajo, pasos 1 a 4)
  — requiere instalar Rust y editar un archivo.

## Vía 0 — que GitHub lo compile por vos (recomendada)

1. Entrá a https://github.com y creá una cuenta gratis si no tenés.
2. Creá un repositorio nuevo (botón verde "New"), privado o público, el
   nombre no importa.
3. En la página del repo recién creado, buscá el link
   "uploading an existing file" (o arrastrá los archivos de esta
   carpeta directo sobre la página) y subí **todo el contenido** de
   esta carpeta (`audiolink-tauri-pilot`) — todas las subcarpetas
   incluidas. Confirmá el commit.
4. Andá a la pestaña **"Actions"** del repositorio (arriba). GitHub va
   a preguntar si querés habilitar Actions — decí que sí.
5. En la lista de la izquierda, hacé clic en **"Compilar AudioLink
   Ensayo (.exe)"**.
6. Aparece un botón **"Run workflow"** (a la derecha) — hacé clic, y
   en el campo de texto que aparece pegá la URL completa de tu
   `ensayo.html` (ej. `https://audiolink-marto.github.io/AUDIOLINK/ensayo.html`).
   Confirmá con el botón verde.
7. Esperá unos 5-10 minutos (podés recargar la página). Cuando el
   círculo de la izquierda se pone ✅ verde, entrá a esa ejecución y
   bajá al final, a la sección **"Artifacts"** — ahí está el `.exe`
   para descargar, dentro de un .zip.

No tocaste ningún archivo de código en ningún momento — la URL se usa
solo durante esa compilación puntual, en la máquina de GitHub.

---

## Compilar en tu propia PC (alternativa, si preferís no usar GitHub)

## 1) Antes de nada: editá la URL

Abrí `src-tauri/tauri.conf.json` y cambiá esta línea por la URL real
de tu sitio en GitHub Pages:

```json
"url": "https://audiolink-marto.github.io/AUDIOLINK/ensayo.html",
```

## 2) Instalar lo necesario (una sola vez)

**Lo único obligatorio es Rust** — Tauri compila la ventana nativa con
Rust, no hay forma de saltear eso. Node es opcional: solo hace falta
si preferís esa vía en vez de la de Cargo.

1. **Rust**: https://www.rust-lang.org/tools/install (instalador `.exe`,
   dejá todo por defecto).
2. **WebView2**: en Windows 10/11 actualizado ya viene instalado. Si
   Windows se queja al abrir la app, bajalo de
   https://developer.microsoft.com/microsoft-edge/webview2/ (Evergreen
   Bootstrapper).
3. Reiniciá la terminal/PC después de instalar Rust, para que el PATH
   se actualice.

## 3) Compilar

### Opción A — sin instalar Node (recomendada si no lo tenés)

Instalás la interfaz de Tauri como un programa de Rust en vez de un
paquete de Node (una sola vez):

```powershell
cargo install tauri-cli --version "^2" --locked
```

Tarda varios minutos la primera vez (compila la herramienta misma).
Después, parado en la carpeta `src-tauri` de este proyecto:

```powershell
cd src-tauri
cargo tauri build
```

### Opción B — con Node (si ya lo tenés instalado)

Desde la raíz del proyecto:

```powershell
npm install
npm run build
```

Las dos opciones hacen lo mismo — usá la que te resulte más cómoda.
Al terminar, el instalador queda en (ruta relativa a `src-tauri`):

```
target\release\bundle\nsis\AudioLink Ensayo_0.1.0_x64-setup.exe
```

Ese `.exe` es el instalador que le podés pasar a cualquier músico —
al correrlo, instala la app con su propio ícono en el menú de inicio.

## 4) Probar sin instalar (opcional, más rápido para iterar)

Sin Node, parado en `src-tauri`: `cargo tauri dev`
Con Node, parado en la raíz: `npm run dev`

Abre la ventana directo, sin generar el instalador — útil para probar
que el login y la carga de audio andan bien antes de compilar el
`.exe` final.


## Qué NO incluye este piloto todavía

- **Ícono real**: `src-tauri/icons/icon.ico` es un círculo dorado de
  relleno. Reemplazalo por tu logo cuando quieras (mismo nombre de
  archivo, formato `.ico` con varios tamaños adentro — hay
  conversores online gratuitos tipo "png to ico").
- **Firma de código**: sin firmar, Windows va a mostrar "Editor
  desconocido" la primera vez que alguien instale. No bloquea la
  instalación, solo es una advertencia.
- **Un menú para navegar entre `ensayo.html`, `musico.html`,
  `proyecto.html`, etc.** — este piloto es UNA sola ventana con UNA
  sola hoja, a propósito, para probar que el enfoque funciona antes de
  construir la navegación completa.
- **CSP deshabilitada** (`"csp": null` en tauri.conf.json) — para que
  la página remota cargue sin pelearse con la política de seguridad
  que Tauri inyecta por defecto. Antes de repartir esto más en serio,
  conviene revisar/ajustar eso en vez de dejarlo desactivado.
