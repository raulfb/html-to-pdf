# html-to-pdf
Generar un PDF a partir de una plantilla html en la que substituimos una serie de datos usando minijinja. Esos datos pueden ser obtenidos de una API.

## Requisitos.

Debido a que usamos la libreria wkhtmltopdf para crear el pdf tenemos que tener instalado en nuestro equipo el programa wkhtmltopdf. Se puede descargar desde su [web](https://wkhtmltopdf.org/downloads.html) 

## Configurar .env
Crear un archivo .env con el contenido del archivo example.env 
## Iniciar proyecto

Para iniciar el proyecto hay que ejecutar el siguiente comando:
```rust 
    cargo run
```
## Estructura

- En la carpeta **/plantillas** es donde se guardan las plantillas html que se usaran para crear el pdf.
- En la carpeta **/pdf** es donde se guardará el pdf.
- En la carpeta **/src** se encuentran los siguientes archivos:
    - **main.rs**: se obtinenen los datos que vamos a substituir en el pdf de la api de dummyjson.
    - **ejemploSencillo.rs**: se obtienen los datos que vamos a substituir los definimos nosotros. Para ejecutar este archivo hay que descomentar la linea # path = "src/ejemploSencillo.rs" que está en el archivo Cargo.toml.
    - **ejemploProducto.rs**: se obtienen los datos que vamos a substituir los definimos nosotros. Para ejecutar este archivo hay que descomentar la linea # path = "src/ejemploProducto.rs" que está en el archivo Cargo.toml.Este ejemplo fue realizado para probar el bucle for.

## Minijinja

### Variables
En el archivo html debemos de poner entre "{{}}" los valores que queramos substituir. Por ejemplo:

```html
<p>Nombre: {{nombre}}</p>
<p>Apellidos: {{apellidos}}</p>
<p>Edad:{{edad}}</p>
```
### Comentarios
Para crear comentarios:

```html
 {#Comentario#}
 ```

### if/else
Ejemplo de como se usaría un if/else:
```html
{% if nombre == 'Zacarias' %}
    <p>Se llama Zacarias</p>
{% else %}
    <p>No se llama Zacarias</p>
{% endif %}
```

### bucle for
Ejemplo de como se haría un bucle:
```html
{% for imagen in imagenes %}
    {{ imagen }}
    <br>
{% endfor %}
```

## Documentación
[Minijinja](https://docs.rs/minijinja/latest/minijinja/index.html)
[wkhtmltopdf](https://wkhtmltopdf.org/)
[crate wkhtmltopdf](https://crates.io/crates/wkhtmltopdf)
