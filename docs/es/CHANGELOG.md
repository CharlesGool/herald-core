# CHANGELOG — Registro de cambios

[English](../en/CHANGELOG.md) · Español · [简体中文](../CHANGELOG.md)

## V1.5 — 2026-11-04

### Eliminado
- Se eliminan todas las implementaciones de SOTER (Qualcomm/Tencent): injector `hook/soter.rs`/`soter_watch.rs`, keymint `soter_beta.rs`/`soter_hal.rs`, el crate `soter-ta`, y pif-common `soter.rs`/`soter_blob.rs` (no se activan en TB321FU).
- Se quita también la AGPL-3.0 que venía con ello: la AGPL solo entraba por el soter-ta de ommega; se eliminan `engine/LICENSE-AGPL-3.0.txt` y `licenses/ommega-soter-ta.txt`, y NOTICE/THIRD_PARTY_NOTICES/README se actualizan en consecuencia.

### Corregido
- Llamada colgante de resetprop a `--soter-watch`, ya eliminado: antes reinyectaba keystore2 cada 60 segundos (defecto HIGH que quedó al eliminar SOTER).

### Cambiado
- El injector añade la selección de destino `inject --target keystore2|system_server`; `entry` elige la extensión según el nombre del proceso. La ruta de autenticación de keystore sigue siendo idéntica byte a byte, y la sesión RPC de HERALD solo se establece en la rama de keystore.
- Se añade `PkgManagerInterceptor` transparente (P2.0, no se activa por defecto) para validar a bajo nivel el ocultamiento de la lista de aplicaciones.

### Revisión
- P2 (inyectar en system_server para interceptar `IPackageManager` y ocultar la lista de aplicaciones) quedó bloqueado por SELinux según la prueba de riesgo en dispositivo real (con system_server en enforcing no se puede cargar el cuerpo de inyección `herald_exec`; permitirlo chocaría con neverallow y debilitaría la seguridad). Se **aparca**; se pasa a `services.jar`. Detalles en `docs/injector-capability-design.md`.

## V1.4 — 2026-11-04

### Añadido
- La interfaz de plugins del broker añade `HERALDP_ASYNC`: los plugins pueden declarar comandos lentos para ejecutarlos en segundo plano sin bloquear el bucle principal (opcional; el conjunto vacío por defecto mantiene el comportamiento sin cambios).

## V1.3 — 2026-11-04

### Corregido
- Se devuelve el directorio de trabajo del broker a `/data/local/tmp`, corrigiendo la regresión en dispositivo real de V1.2 (respuestas vacías por la etiqueta SELinux).

## V1.2 — 2026-11-03

### Cambiado
- Endurecimiento de seguridad del motor: ya no hace panic con versiones de Android desconocidas, `keymint.dat` se escribe de forma atómica, y un fallo al analizar keybox no sobrescribe el archivo del disco.

## V1.1 — 2026-11-03

### Cambiado
- Sondeo adaptativo del broker (las operaciones seguidas casi no esperan; cuando está inactivo vuelve a intervalos más largos).
- Endurecimiento del shell del núcleo: las escrituras de respuestas y tokens de emparejamiento usan O_EXCL, validación de nombres de clave de Settings, y `heraldctl log` lee a través del broker.

## V1.0 — 2026-11-01

### Añadido
- Primera versión: broker, `heraldctl`, interfaz de plugins y el motor de autenticación (keymint, inject) bifurcado de Oh My Keymint.
