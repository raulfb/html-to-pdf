# html-to-pdf
Generar un PDF a partir de una plantilla html en la que substituimos una serie de datos usando minijinja.

## Requisitos.

Debido a que usamos la libreria wkhtmltopdf para crear el pdf tenemos que tener instalado en nuestro equipo el programa wkhtmltopdf. Se puede descargar desde su [web](https://wkhtmltopdf.org/downloads.html) 

## Iniciar proyecto

Para iniciar el proyecto hay que ejecutar el siguiente comando:
```rust 
    cargo run
```

## Estructura

En la carpeta /plantillas es donde se guardan las plantillas html que se usaran para crear el pdf.
En la carpeta /pdf es donde se guardará el pdf.

## Minijinja

En el archivo html debemos de poner entre "{{}}" los valores que queramos substituir. Por ejemplo:
```html
    <p>Nombre: {{nombre}}</p>
    <p>Apellidos: {{apellidos}}</p>
    <p>Edad:{{edad}}</p>
```

## Documentación
[Minijinja](https://docs.rs/minijinja/latest/minijinja/index.html)
[wkhtmltopdf](https://wkhtmltopdf.org/)
[crate wkhtmltopdf](https://crates.io/crates/wkhtmltopdf)

