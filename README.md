![Logo](./assets/logo.png)

Notes R Ust es una aplicación de terminal para gestionar notas. Permite crear, buscar, actualizar y borrar notas de manera sencilla y eficiente.

## Características

- **Crear notas**: Agrega nuevas notas con un título y contenido.
- **Buscar notas**: Encuentra notas por su título.
- **Actualizar notas**: Modifica el contenido de notas existentes.
- **Borrar notas**: Elimina notas que ya no necesites.

## Requisitos

Asegúrate de tener instalado [Rust](https://www.rust-lang.org/tools/install) y [Cargo](https://doc.rust-lang.org/cargo/) en tu sistema.

## Instalación

Para instalar y ejecutar la aplicación, sigue estos pasos:

1. Clona el repositorio:

```bash
git clone git@github.com:gorkamu/notes-rust.git
cd notes-rust
```

2. Compila el proyecto usando Cargo:

```bash
cargo build --release
```

3. Ejecuta la aplicación:

```bash
./target/release/notes-r-ust
```

## Dependencias

Este proyecto utiliza las siguientes dependencias:

- **chrono**: Para manejar fechas y horas.
- **inquire**: Para crear interfaces de usuario interactivas en la terminal.
- **rusqlite**: Para interactuar con bases de datos SQLite.
- **ansi_term**: Para imprimir texto en colores en la terminal.

## Contribuciones

Las contribuciones son bienvenidas. Si deseas contribuir, por favor abre un issue o envía un pull request.
