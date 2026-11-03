# CHANGELOG — Registro de cambios

[English](../en/CHANGELOG.md) · Español · [简体中文](../CHANGELOG.md)

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
