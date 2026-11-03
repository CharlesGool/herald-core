# herald-core

[English](../en/README.md) · Español · [简体中文](../../README.md)

[![Licencia](https://img.shields.io/badge/license-GPL--3.0--or--later-blue.svg)](../../LICENSE)

Núcleo sin root: un broker que se ejecuta en un dominio SELinux independiente, la herramienta de línea de comandos `heraldctl` y la interfaz de plugins, a la que otros parches se conectan mediante la interfaz. **El producto es el parche GSI `herald-core`** (`make dist` → `dist/herald-core/`), que se usa en TB321FU-GSI-Mods.

Documentación: [Diseño](DESIGN.md) · [Interfaz de plugins](../plugin-api.md) · [Cambios](CHANGELOG.md) · [Registro de mantenimiento](LOG.md) · [Historial](HISTORY.md) · [Avisos de terceros](THIRD_PARTY_NOTICES.md)

## Introducción

| Directorio | Contenido |
|---|---|
| `patch/` | Código fuente del parche, ver `patch/README.md` |
| `docs/plugin-api.md` | Descripción de la interfaz de plugins |
| `engine/` | Código fuente del motor de autenticación (HeraldKeymint), bifurcado de Oh My Keymint; los identificadores se han cambiado a herald y solo se conserva lo necesario para compilar `keymint` e `inject`. `make engine` compila; la explicación está en `engine/README.herald.md` |
| `engine/src/` | Demonio keymint (implementación de KeyMint que sustituye al backend keystore2, keybox, RKP local) |
| `engine/injector/` | Inyector inject (inyección ptrace en keystore2, enruta las solicitudes por App) |
| `engine/common/`、`boringssl/`、`ta/`、`wire/`、`derive/`、`watchdog/`、`tests/` | Bibliotecas base como los crates kmr-* de AOSP KeyMint |
| `engine/soter-ta/`、`pif-common/` | TA de software Soter de Qualcomm (adaptado de ommega) y definiciones Soter compartidas |
| `engine/aidl/`、`proto/`、`runtime/`、`assets/` | Interfaces AIDL, protobuf, la parte Java del puente RKP V1 y los recursos compilados en el binario |
| `engine/scripts/` | `build-android.sh` (llamado por `make engine`), `build-boringssl.sh` (compila BoringSSL con una versión fija) |
| `engine/docs/` | Documentación upstream (configuración, FAQ, fuentes de terceros); las secciones exclusivas del modo módulo están marcadas |

Las partes upstream que solo sirven al modo módulo de Magisk/KernelSU (`webui/`、`template/`、`pif-spoof/`、`hal/`、scripts de empaquetado, etc.) ya se eliminaron; el motivo está en `engine/README.herald.md`.

**Relación con TB321FU-GSI-Mods**: aquí está la única fuente. La copia sincronizada en Mods, `patches/required/general/herald-core`, se actualiza tras cambiar este repositorio ejecutando `make sync-core` en Mods (`make unit` comprueba que ambos lados coinciden). La visión general del diseño está en [docs/DESIGN.md](DESIGN.md).

## Requisitos

- Compilar el motor (`make engine`): host de compilación Linux, Android NDK, BoringSSL y toolchain Rust nightly; ver `engine/README.herald.md` y `.env.example`.
- Solo parches (`make dist`): sin requisitos adicionales.

## Instalación

No requiere instalación independiente. Como parche GSI, TB321FU-GSI-Mods lo incorpora en la imagen del sistema durante la construcción (`make sync-core` sincroniza el `patch/` de este repositorio).

## Guía

```bash
make dist      # Genera el parche GSI en dist/herald-core/
make engine    # Compilación cruzada del motor de autenticación keymint e inject (requiere toolchain)
```

Para el uso de la línea de comandos en el dispositivo, ver `patch/README.md` (`heraldctl selftest` / `diag` / `pair`, etc.); para el desarrollo de plugins, ver [docs/plugin-api.md](../plugin-api.md).

## Desinstalación

No se instala por separado; basta con quitar el parche `herald-core` de las opciones de construcción de TB321FU-GSI-Mods y volver a construir.

## Agradecimientos

Este proyecto basa su motor de autenticación en [Oh My Keymint](https://github.com/qwq233/OhMyKeymint) y en otros proyectos de código abierto; la atribución completa, las fuentes y las licencias están en [docs/THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) y `engine/NOTICE`.

## Licencia

Este repositorio tiene varias licencias que coexisten, cada una para su parte, sin sobrescribirse:

| Parte | Licencia | SPDX | Texto completo |
|---|---|---|---|
| `patch/`、`docs/`、archivos del directorio raíz (el núcleo en sí) | GNU GPL v3.0 o posterior | `GPL-3.0-or-later` | `LICENSE` |
| Cuerpo principal de `engine/` (versión modificada de Oh My Keymint) | General License Agreement v4 + Oh My Keymint License (**solo uso no comercial**; la versión modificada debe incluir una declaración de cambios y no puede usar el nombre original) | — | `engine/LICENSE.md`、`engine/LICENSE-2` |
| `engine/soter-ta/` (adaptado de ommega) | GNU AGPL v3.0 o posterior | `AGPL-3.0-or-later` | `engine/LICENSE-AGPL-3.0.txt` |
| Crates kmr-* bajo `engine/` (de AOSP) | Apache License 2.0 | `Apache-2.0` | `engine/licenses/Apache-2.0.txt` |
| Código RKP local bajo `engine/` adaptado de Duck-ToolBox / rkp-v1-oracle | MIT | `MIT` | `engine/licenses/` |

Los binarios de `engine/` solo pueden usarse en escenarios no comerciales. La atribución y las declaraciones de modificación están en `NOTICE`, `engine/NOTICE` y [docs/THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md); el `NOTICE.engine` del directorio raíz es solo un marcador que apunta a `engine/NOTICE`.
