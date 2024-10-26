# Sistema Solar con Shaders en Rust

Este proyecto implementa un sistema solar básico utilizando shaders personalizados en Rust. El enfoque principal es la creación de cuerpos celestes como el Sol, planetas rocosos, y gigantes gaseosos mediante shaders que generan efectos de color y texturas sin el uso de texturas externas. Este README describe la estructura del proyecto, las instrucciones de uso, y detalles técnicos.

## Descripción del Proyecto

Este proyecto simula un sistema solar con varios cuerpos celestes renderizados mediante un **software renderer** en Rust. Cada cuerpo celeste utiliza shaders específicos que simulan efectos de atmósfera, superficie, nubes y anillos. 

### Objetivos
- Crear shaders interesantes que permitan variar colores utilizando parámetros disponibles en el software.
- Diseñar y renderizar cuerpos celestes, como el Sol, planetas rocosos (Mercurio, Venus, Tierra, Marte) y gigantes gaseosos (Júpiter, Saturno).
- Implementar efectos visuales dinámicos, como:
  - **Anillos** en planetas gaseosos.
  - **Nubes y atmósferas** en planetas terrestres.
  - **Movimiento de la superficie** en planetas como la Tierra.
  - **Efectos emisivos** para el Sol.


### Planetas Incluidos
- **Sol**: Shader con efecto emisivo simulando llamaradas y brillo.
- **Mercurio**: Planeta rocoso cercano al Sol, simulado con tonos grisáceos.
- **Venus**: Efecto atmosférico utilizando tonos pálidos y patrones suaves.
- **Tierra**: Superficie detallada con colores para tierra, agua, y nubes en movimiento.
- **Marte**: Planeta rocoso con tonos rojizos y textura irregular.
- **Júpiter**: Gigante gaseoso con bandas y movimiento en la atmósfera.
- **Saturno**: Anillos generados mediante shaders, simulando el aspecto característico.

## Instrucciones de Uso

### Prerrequisitos
- **Rust** instalado en el sistema. Puedes instalar Rust desde [Rust-lang.org](https://www.rust-lang.org/).
- Dependencias del proyecto especificadas en `Cargo.toml`.

### Ejecución del Proyecto

1. Clona el repositorio:
   ```bash
   git clone https://github.com/tu_usuario/sistema-solar-shaders.git
   cd sistema-solar-shaders
´´´

2. Compila el proyecto:
   ```bash
   cargo build --release
   ```

3. Ejecuta el binario generado:
   ```bash
   cargo run --release
   ```

### Controles del Sistema Solar

- **Movimiento de la cámara**: Utiliza las teclas de flechas para mover la cámara alrededor del sistema solar.
- **Zoom**: Utiliza las teclas `w` y `s` para acercar o alejar la cámara.
- **Salir**: Presiona `Esc` para salir del programa.

## Detalles Técnicos

### Implementación
- **Software Renderer**: El proyecto utiliza un software renderer que calcula los colores de los píxeles en pantalla mediante shaders personalizados.
- **Shaders**: Cada cuerpo celeste tiene un shader específico que define los colores y efectos visuales.
- **Movimiento**: Se implementa un sistema de coordenadas y movimiento para simular la rotación de los planetas alrededor del Sol.

### Shaders Implementados
- **Shader de Sol**: Efecto emisivo con variación de color.
- **Shader de Mercurio**: Tonalidades grises y efecto de superficie rocosa.
- **Shader de Venus**: Efecto atmosférico con colores pálidos y suaves.
- **Shader de Tierra**: Superficie con textura de agua, tierra y nubes en movimiento.
- **Shader de Marte**: Superficie rojiza con textura irregular y montañas.
- **Shader de Júpiter**: Efecto de bandas y movimiento en la atmósfera.
- **Shader de Saturno**: Anillos generados mediante shaders