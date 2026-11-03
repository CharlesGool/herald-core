# LOG — Registro de mantenimiento

[English](../en/LOG.md) · Español · [简体中文](../LOG.md)

## Registro

Los cambios de versión están en [CHANGELOG.md](CHANGELOG.md); el historial de requisitos y los traspasos antiguos están en [HISTORY.md](HISTORY.md).

## Defectos

Ninguno por ahora.

## Limitaciones

- Los binarios `keymint`/`inject` de `engine/` están sujetos a GLA v4 y solo pueden usarse de forma no comercial.

## Decisiones

| Fecha | Decisión | Estado | Motivo |
|---|---|---|---|
| 2026-11-01 | Renombrar engine por completo de omk a herald | Aceptada | La cláusula 3 de la Oh My Keymint License no permite usar el nombre original fuera de lo autorizado |
| 2026-11-04 | El directorio de trabajo del broker se mantiene en `/data/local/tmp` | Aceptada | Moverlo a `/data/adb` hace que la etiqueta SELinux impida leer las respuestas (regresión en dispositivo real de V1.2) |

## Traspaso

No hay trabajo en curso. Rama actual `main`, sincronizada con `origin/main`.
