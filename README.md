# Space-Travel 🚀
Este proyecto, desarrollado en Rust, simula un sistema solar ficticio con planetas, lunas, y una nave espacial. Incluye sistemas dinámicos de rotación, órbitas y shaders personalizados para cada cuerpo celeste, haciendo énfasis en el diseño y la interactividad.

## Características Implementadas
### Simulación de Planetas
- Shaders Personalizados: Cada planeta tiene un shader único que simula distintos efectos de superficie y atmósfera.
- Planeta Gaseoso con Anillos: Incluye un sistema de anillos, con simulación de patrones gaseosos.
- Planeta Rocoso con Luna Orbitando: Un planeta rocoso tiene una luna que orbita a su alrededor, con efectos detallados en la superficie.
- Variación Dinámica: Los efectos visuales varían con el tiempo, dando vida al sistema solar.

### Nave Espacial
- La nave espacial se encuentra siempre frente a la vista del usuario, moviéndose a gusto del usuario.
- Un shader especial da un aspecto brillante y realista a la nave.

### Interactividad
- Exploración del Sistema Solar: El usuario puede moverse libremente por el sistema solar con controles de cámara intuitivos.

## Requisitos
Asegúrate de tener Rust y Cargo instalados en tu sistema. Puedes verificarlo ejecutando:
```bash
    cargo --version
```

## Instalación y Configuración
1. **Clona el repositorio**:
    ```bash
    git clone <repository-url>
    ```
2. **Navega al directorio**:
   ```bash
    cd <repository-name>
    ```
3. **Instala las dependencias**:
    ```bash
    cargo add minifb nalgebra-glm tobj rand fastnoise-lite
    ```
3. **Compila y ejecuta el proyecto**:
    ```bash
    cargo run --release
    ```
## Controles
Una vez dentro del programa, puedes interactuar con los planetas utilizando los siguientes controles:
- **Movimiento de Cámara**
  - Flecha Izquierda: Mueve la cámara hacia la izquierda.
  - Flecha Derecha: Mueve la cámara hacia la derecha.
  - Flecha Arriba: Mueve la cámara hacia arriba.
  - Flecha Abajo: Mueve la cámara hacia abajo.
- **Zoom**
  - Q: Acercar (Zoom in).
  - E: Alejar (Zoom out).
- **Rotación del planeta**
  - A: Rotar la nave a la izquierda (eje Y).
  - D: Rotar la nave a la derecha (eje Y).
  - W: Rotar la nave hacia arriba (eje X).
  - S: Rotar la nave hacia abajo (eje X).
- **Salir**
  - Escape: Cierra la aplicación.

## Video simulación
[Aquí](https://youtu.be/F1B6cRi4z-Q) puedes ver el funcionamiento del proyecto.

![image](https://github.com/user-attachments/assets/17e10f10-2872-47c1-88d2-6718fcbdebd3)


