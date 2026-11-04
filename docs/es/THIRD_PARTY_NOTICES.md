# THIRD_PARTY_NOTICES — Avisos de terceros

[English](../en/THIRD_PARTY_NOTICES.md) · Español · [简体中文](../THIRD_PARTY_NOTICES.md)

Materiales de terceros distribuidos en este repositorio y sus obligaciones de atribución, licencia y cumplimiento. El núcleo (`patch/`, `docs/`, archivos del directorio raíz) es de desarrollo propio y se publica bajo GPL-3.0-or-later; los materiales de terceros se concentran en `engine/`.

## Avisos de terceros

### Inventario de componentes

| Componente | Versión/base | Upstream | Licencia | Uso | Titularidad de los derechos | Obligaciones de cumplimiento | Fecha de verificación |
|---|---|---|---|---|---|---|---|
| Oh My Keymint | bafe5ba(1.3.5) | github.com/qwq233/OhMyKeymint、github.com/ITxiao6666/OhMyKeymint | General License Agreement v4 + Oh My Keymint License (solo no comercial) | Cuerpo principal de `engine/`, renombrado y simplificado | James Clef (qwq233) | Uso no comercial; incluir declaración de modificaciones (`engine/NOTICE`); no usar el nombre original | 2026-11-04 |
| AOSP KeyMint (crates kmr-*) | AOSP | android.googlesource.com | Apache-2.0 | `boringssl/`、`common/`、`derive/`、`ta/`、`wire/`、`tests/`、`watchdog/` de `engine/` | The Android Open Source Project | Conservar los avisos; los cambios están declarados archivo por archivo en la sección 4 de `engine/NOTICE` | 2026-11-04 |
| Duck-ToolBox | — | github.com/eltavine/Duck-ToolBox | MIT | Adaptado en `engine/src/local_rkp/` | Autores de Duck-ToolBox | Conservar los avisos | 2026-11-04 |
| rkp-v1-oracle | — | github.com/LazyBonesLZY/rkp-v1-oracle | MIT | Adaptado en `engine/src/rkp_v1_bridge.rs` | Autores de rkp-v1-oracle | Conservar los avisos | 2026-11-04 |
| BoringSSL | ed9209cf | boringssl.googlesource.com | Según el upstream (OpenSSL/ISC, etc.) | Enlazado al compilar de forma cruzada; no se distribuye con el código fuente | Google | Dependencia de compilación; forma de obtenerlo en `engine/scripts/build-boringssl.sh` | 2026-11-04 |

### Textos de licencia

Ruta del texto completo en el repositorio:

- GPL-3.0-or-later (núcleo): `LICENSE`
- General License Agreement v4 + Oh My Keymint License: `engine/LICENSE.md`、`engine/LICENSE-2`
- Apache-2.0 (crates kmr-*): `engine/licenses/Apache-2.0.txt`
- MIT (Duck-ToolBox, rkp-v1-oracle): `engine/licenses/Duck-ToolBox.txt`、`engine/licenses/rkp-v1-oracle.txt`
- Declaración completa de modificaciones de `engine/`, archivo por archivo: `engine/NOTICE`; más fuentes: `engine/docs/THIRD_PARTY.md`

### Disponibilidad del código fuente

Los binarios `keymint` e `inject` generados desde `engine/` están sujetos a las obligaciones de disponibilidad del código fuente de GLA v4 y GPL-3.0. El código fuente correspondiente completo es el directorio `engine/` de este repositorio; el método de compilación está en `engine/README.herald.md` y `engine/scripts/`. BoringSSL no se distribuye con este repositorio: `engine/scripts/build-boringssl.sh` lo obtiene del upstream con una versión fija.

### Revisión de cumplimiento

- Componentes copyleft: GPL-3.0-or-later (núcleo `patch/`), GLA v4 (cuerpo principal de `engine/`, solo no comercial).
- Análisis de separación: `keymint`/`inject` de `engine/` son binarios independientes, que el parche GSI `attestation-fix` lanza como servicio de init; tienen licencias distintas a las de `patch/` (GPL-3.0) y no se sobrescriben entre sí (ver `NOTICE`).
- GLA v4 / Oh My Keymint License: solo uso no comercial; los binarios de `engine/` solo pueden usarse en escenarios no comerciales.
- Registros de la revisión: este archivo y `engine/NOTICE`.
