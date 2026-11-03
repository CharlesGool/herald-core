# herald-core

[English](../en/README.md) · Español · [简体中文](../../README.md)

Núcleo sin root: un broker que se ejecuta en un dominio SELinux independiente, la herramienta de línea de comandos `heraldctl` y la interfaz de plugins, a la que otros parches se conectan mediante la interfaz.
**El producto es el parche GSI `herald-core`** (`make dist` → `dist/herald-core/`), que se usa en TB321FU-GSI-Mods.

| Directorio | Contenido |
|---|---|
| `patch/` | Código fuente del parche, ver `patch/README.md` |
| `docs/plugin-api.md` | Descripción de la interfaz de plugins |
| `engine/` | Código fuente del motor de autenticación (HeraldKeymint), bifurcado de Oh My Keymint, con los identificadores cambiados a herald; solo se conserva la parte necesaria para compilar `keymint` e `inject`. `make engine` compila; la explicación está en `engine/README.herald.md` |
| `engine/src/` | Demonio keymint (implementación de KeyMint que reemplaza el backend de keystore2, keybox, RKP local) |
| `engine/injector/` | Inyector inject (inyección ptrace en keystore2, enruta las solicitudes por App) |
| `engine/common/`, `boringssl/`, `ta/`, `wire/`, `derive/`, `watchdog/`, `tests/` | Bibliotecas base como los crates kmr-* de AOSP KeyMint |
| `engine/soter-ta/`, `pif-common/` | TA de software Soter de Qualcomm (adaptado de ommega) y definiciones Soter compartidas |
| `engine/aidl/`, `proto/`, `runtime/`, `assets/` | Interfaces AIDL, protobuf, la parte Java del puente RKP V1 y los archivos de recursos compilados en el binario |
| `engine/scripts/` | `build-android.sh` (llamado por `make engine`), `build-boringssl.sh` (compila BoringSSL con una versión fija) |
| `engine/docs/` | Documentación upstream (configuración, FAQ, fuentes de terceros); las secciones exclusivas del modo módulo están marcadas |

Las partes upstream que solo sirven al modo módulo de Magisk/KernelSU (`webui/`, `template/`, `pif-spoof/`, `hal/`, scripts de empaquetado, etc.) ya se eliminaron; el motivo está en `engine/README.herald.md`.

**Relación con TB321FU-GSI-Mods**: aquí está la única fuente. La copia sincronizada en Mods, `patches/required/general/herald-core`, se actualiza tras cambiar este repositorio ejecutando `make sync-core` en Mods (`make unit` comprueba que ambos lados coinciden).

## Licencia

En este repositorio conviven varias licencias, cada una para su parte, sin sobrescribirse:

| Parte | Licencia | Texto completo |
|---|---|---|
| `patch/`, `docs/`, archivos del directorio raíz (el núcleo en sí) | GPL-3.0-or-later | `LICENSE` |
| Cuerpo principal de `engine/` (versión modificada de Oh My Keymint) | General License Agreement v4 + Oh My Keymint License (**solo uso no comercial**; la versión modificada debe incluir una declaración de cambios y no puede usar el nombre original) | `engine/LICENSE.md`, `engine/LICENSE-2` |
| `engine/soter-ta/` (adaptado de ommega) | AGPL-3.0-or-later | `engine/LICENSE-AGPL-3.0.txt` |
| Crates kmr-* bajo `engine/` (de AOSP) | Apache-2.0 | `engine/licenses/Apache-2.0.txt` |
| Código RKP local bajo `engine/` adaptado de Duck-ToolBox / rkp-v1-oracle | MIT | `engine/licenses/` |

Los binarios de `engine/` solo pueden usarse en escenarios no comerciales.

Atribución y declaraciones de modificación:

- `NOTICE`: nota del núcleo y resumen de las fuentes de engine/;
- `engine/NOTICE`: declaración completa de modificaciones de engine/ (obra original, autores originales, fuente y commit base bafe5ba, modificador CharlesGool, cambios desde 2026-11-01 y lista de archivos Apache-2.0 modificados);
- `engine/docs/THIRD_PARTY.md`: fuentes de los componentes de terceros en engine/.

`NOTICE.engine` del directorio raíz es solo un marcador que apunta a `engine/NOTICE`.
