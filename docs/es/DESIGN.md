# DESIGN — Diseño

[English](../en/DESIGN.md) · Español · [简体中文](../DESIGN.md)

## Objetivos de diseño

- Ofrecer la capacidad base y el modo de acceso para que los servicios del sistema ejecuten operaciones como root a través de un intermediario, sin depender de herramientas root; el núcleo en sí no incluye ninguna función concreta.
- El canal de comandos no exige privilegios al llamador (línea de comandos `heraldctl`; la App de frontend emparejada) y ejecuta por delegación según una lista blanca.
- El motor de autenticación toma el control de KeyMint desde el backend keystore2 y ofrece atestación de claves del dispositivo; se conecta como plugin, no como parte integrada del núcleo.
- Desacoplado de la ROM base: el núcleo es un script de shell más un `resetprop`, así que cambiar de GSI no requiere modificarlo.

No objetivos: el núcleo no implementa suplantación de identidad (spoofing), batería, rendimiento ni otras funciones concretas; estas las aportan los parches de función como plugins.

## Arquitectura

| Módulo | Responsabilidad |
|---|---|
| broker (`patch/files/herald-start.sh`) | Se ejecuta en el dominio SELinux independiente `herald`, se inicia tras boot_completed; sondea los archivos de solicitud y ejecuta por delegación según la lista blanca |
| `heraldctl` (`patch/files/heraldctl`) | Interfaz de línea de comandos sin privilegios; entrega las solicitudes al broker a través del canal de comandos |
| Interfaz de plugins | Otros parches colocan archivos en el directorio de plugins para añadir al broker y a heraldctl comandos, demonios y autocomprobaciones |
| Motor de autenticación (`engine/`, HeraldKeymint) | `keymint` (demonio KeyMint, keybox, RKP local) e `inject` (inyección ptrace en keystore2); bifurcado de Oh My Keymint |

Hay dos canales de comandos: la línea de comandos (`/data/local/tmp/herald`, escribible por shell, solo contiene req/resp) y la App de frontend emparejada (clave aleatoria del dispositivo + verificación de uid). Las respuestas se escriben primero en un archivo temporal y después se renombran, para evitar leer datos a medio escribir.

Responsabilidades de los directorios de nivel superior:

- `patch/` — código fuente del parche GSI del núcleo (broker, heraldctl, sepolicy, init); producto `make dist` → `dist/herald-core/`.
- `engine/` — código fuente del motor de autenticación (bifurcación, ver `engine/README.herald.md`).
- `docs/` — documentación del proyecto.

## Restricciones de diseño

- La interfaz de plugins (`heraldp_*` del broker y sus variables; `send` / `heraldctl_cmd_*` de heraldctl) es el contrato del que dependen otros parches; los cambios deben mantener la compatibilidad.
- El directorio de trabajo de las respuestas debe estar en `/data/local/tmp`, porque solo con el dominio shell en enforcing se puede leer una respuesta con la etiqueta `shell_data_file`. Guardarlo en `/data/adb` hace que la etiqueta SELinux no coincida y la respuesta no se pueda leer (comprobado en dispositivo real con V1.2).
- El dominio del broker está en permissive, pero el directorio del canal solo contiene req/resp; los registros y el directorio de trabajo están aislados para que un enlace simbólico no pueda redirigir escrituras hechas como root.

## Interfaz externa

La App de frontend envía y recibe comandos a través del canal emparejado (`files/herald_req`, clave aleatoria + verificación de uid); el protocolo está en la documentación de la interfaz del backend del proyecto. El canal de línea de comandos está orientado a uid 0/2000.

## Extensión

El mecanismo de plugins se describe en `docs/plugin-api.md`: plugins del broker (`heraldp_cmd_*`, etc.) y plugins de heraldctl (`heraldctl_cmd_*`), además de las convenciones para las etiquetas de directorios de payload y para añadir reglas SELinux.
