# Skill: Documentación Estructurada del Proyecto VPNs

## Propósito

Este skill define **cómo se documenta el avance diario** del proyecto `vpnlocal` / `vpns`.
Sirve como plantilla para que cualquier agente genere documentación
estructurada, consistente y reutilizable, **sin regenerar ni reescribir** carpetas
y archivos de días anteriores.

---

## Regla de oro

> **Solo se documenta el avance NUEVO del día.**
> Nunca se vuelve a generar, renombrar ni reescribir una carpeta de documentación
> ya existente. Si un sistema cambió hoy, se **agrega** un sufijo de versión o un
> archivo nuevo dentro de su carpeta; no se borra el anterior.

Excepción permitida: corregir un typo o un dato erróneo en un `.md` existente,
editándolo puntualmente (nunca recreándolo desde cero).

---

## Ubicación

Toda la documentación debe generarse en:

```
/home/eynor/Documents/2_Proyect_Private/9_vpn_Libre/Documentacion/
```

El skill también sirve como referencia rápida para encontrar el código fuente
en el proyecto.

---

## Estructura de carpetas

Cada sistema/funcionalidad documentada vive en su propia carpeta numerada.
El número define el orden histórico de creación (01 es lo primero que se hizo).

```
Documentacion/
├── SKILL.md                        ← este archivo (reglas de documentación)
├── 01_NombreCarpeta/
│   ├── 1_Uso_En_El_Proyecto.md
│   ├── 2_Integracion.md
│   └── 3_Codigo_Y_Funcionamiento.md
├── 02_NombreCarpeta/
│   ├── 1_Uso_En_El_Proyecto.md
│   └── ...
└── 03_NombreCarpeta/
    └── ...
```

### Convenciones de nombres

- **Carpeta**: `NN_NombreSistema` — número de 2 dígitos + guion bajo + Nombre en PascalCase o snake-like legible.
  Ejemplos: `01_Arquitectura_General`, `02_Pool_De_Configs`, `03_Conexion_OpenVPN`.
- **Archivo**: `N_Titulo.md` — número de archivo + guion bajo + Título descriptivo.
  Ejemplos: `1_Uso_En_El_Proyecto.md`, `2_Integracion.md`, `3_Codigo_Y_Funcionamiento.md`.

---

## Los 3 archivos obligatorios por carpeta

Toda carpeta de documentación debe contener, como mínimo, estos 3 archivos.
Se pueden agregar más (4, 5, ...) si el sistema lo requiere.

### 1. `1_Uso_En_El_Proyecto.md` — El "qué" y el "por qué"

Responde: **¿para qué sirve esto dentro del proyecto?** Sin código.
Debe cubrir, **en este orden**:

- **Técnica / Método** (obligatorio): el nombre formal o descripción de la
  técnica/solución usada.
- Qué problema resuelve.
- Dónde se usa (comandos CLI, flujo de conexión).
- Parámetros/archivos involucrados.
- Comportamiento esperado.

### 2. `2_Integracion.md` — El "dónde" y el "con qué"

Responde: **¿cómo está integrado en el binario `vpnlocal`?**
Debe cubrir:
- Archivos del proyecto implicados (rutas absolutas o relativas desde la raíz del repo).
- Módulos Rust (`src/app/`, `src/blocks/`, `src/core/`).
- Dependencias entre módulos.
- Flujo entre componentes.
- Configuración (archivos en `~/.config/vpnlocal/`).

### 3. `3_Codigo_Y_Funcionamiento.md` — El "cómo funciona por dentro"

Responde: **¿cómo funciona el código individualmente, paso a paso?**
Debe cubrir:
- Algoritmo en pasos claros.
- Funciones/estructuras clave con su propósito (`struct`, `fn`).
- Llamadas relevantes (CLI -> runner -> pool -> engine -> network).
- Casos importantes y validaciones.
- Ejemplos de uso con el binario.

---

## Formato de cada archivo `.md`

Todo archivo debe iniciar con este encabezado estándar:

```markdown
# <Número>. <Título Descriptivo>

**Proyecto:** vpnlocal (9_vpn_Libre)
**Sistema:** <Nombre del sistema>
**Fecha de documentación:** YYYY-MM-DD
**Autor:** opencode
**Ruta raíz:** /home/eynor/Documents/2_Proyect_Private/9_vpn_Libre

---

## <secciones...>
```

### Reglas de estilo

- Español, claro y directo.
- Títulos jerárquicos (`#`, `##`, `###`).
- Bloques de código con su lenguaje: ```rust / ```bash / ```text
- Referencias a código como `src/app/runner.rs:45`.
- Tablas para rutas, comandos, estructuras.
- Cerrar cada archivo con una sección `## Notas y pendientes` si aplica.

---

## Mapeo rápido de rutas del proyecto

Para encontrar código rápidamente al documentar:

| Ruta | Propósito |
|---|---|
| `src/main.rs` | Punto de entrada |
| `src/app/runner.rs` | Orquestación de comandos CLI |
| `src/blocks/cli/` | Definición de comandos (`clap`) |
| `src/blocks/pool/loader.rs` | Carga de configs (.conf/.ovpn) |
| `src/blocks/pool/selector.rs` | Selección aleatoria |
| `src/blocks/engine/openvpn.rs` | Conexión OpenVPN (actual) |
| `src/blocks/engine/wireguard.rs` | Conexión WireGuard (legacy) |
| `src/blocks/history/` | Historial de conexiones |
| `src/core/config.rs` | Rutas de configuración |
| `~/.config/vpnlocal/` | Configuración del usuario (pool, history.json, session.json) |
| `vpns/vpngate-configs/` | Configs .ovpn reales de VPNGate |

---

## Flujo para documentar el avance del día

1. **Identificar** qué sistemas se agregaron/modificaron hoy.
2. **Verificar** si ya existe una carpeta `NN_` para ese sistema.
   - Si existe → editar/añadir dentro, NO recrear.
   - Si no existe → crear la siguiente carpeta `NN_NombreSistema` con el
     número inmediato superior al mayor existente.
3. **Crear los 3 archivos** obligatorios dentro de la nueva carpeta.
4. **Registrar ubicaciones exactas** de archivos modificados/creados.
5. No tocar carpetas de días previos.

---

## Checklist antes de dar por cerrada la documentación

- [ ] La carpeta usa `NN_NombreSistema`.
- [ ] Existen `1_Uso_En_El_Proyecto.md`, `2_Integracion.md`, `3_Codigo_Y_Funcionamiento.md`.
- [ ] `1_Uso_En_El_Proyecto.md` empieza nombrando la **Técnica / Método** usado.
- [ ] Cada `.md` tiene el encabezado estándar con fecha, ruta raíz.
- [ ] Se listan rutas reales del código (`src/...`).
- [ ] No se recreó ninguna carpeta anterior.
- [ ] El código citado existe realmente en the proyecto.
