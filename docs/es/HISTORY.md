# HISTORY — Historial

[English](../en/HISTORY.md) · Español · [简体中文](../HISTORY.md)

- 2026-11-01: Se crea el proyecto. El núcleo (broker + heraldctl + interfaz de plugins) se desarrolla de forma propia; el motor de autenticación es una bifurcación de Oh My Keymint (base bafe5ba), renombrada a herald. El nombre original era `omk-core`; después se cambió a `herald-core`.
- 2026-11-04: Tras corregir la regresión del directorio de trabajo en dispositivo real, V1.3 se archiva como la versión oficial `user-v1.3` de TB321FU.
- 2026-11-04: V1.5 elimina SOTER (y con él la AGPL), corrige el `--soter-watch` colgante de resetprop (HIGH); el injector pasa a tener selección de destino y transmisión de P2.0. P2 (inyectar en system_server para ocultar la lista de aplicaciones) queda bloqueado por SELinux según la prueba en dispositivo real; se aparca y se pasa a services.jar. Se archiva como la versión ancla `user-v1.4` de TB321FU.
