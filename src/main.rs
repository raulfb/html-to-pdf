use wkhtmltopdf::*;
use std::fs;
use minijinja::{Environment, context};
use reqwest;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] 

// Definimos la estructura Usuario
struct Usuario {
    id: i32,
    first_name: String,
    last_name: String,
    age: u64,
}

#[tokio::main]
async fn main() {
    // Obtenemos la url de la api del .env
    dotenv().ok();
    let url_api = std::env::var("API_URL").expect("No se pudo acceder al valor de la variable de entorno API_URL");
    // Consultamos los datos de la API dummyjson.com
    let url = url_api;
    let res = reqwest::get(url).await.unwrap();

    match res.status() {
        reqwest::StatusCode::OK => {
            match res.json::<Usuario>().await {
                Ok(parseado) => reenderizar_plantilla(&parseado),
                Err(error) => println!("{:?}",error)
            }
        }
        other => {
            panic!("Error al obtener los datos de la API: {:?}", other);
        }
    }    
}

fn reenderizar_plantilla(user: &Usuario){
    // Leemos la plantilla html
    let plantilla_html = fs::read_to_string("./plantillas/ejemplo.html").expect("Error al leer el archivo.");
   
    // Creamos la plantilla prueba
    let mut env = Environment::new();
    env.add_template("prueba", &plantilla_html).unwrap();
   
    // Seleccionamos la plantilla prueba
    let plantilla_prueba = env.get_template("prueba").unwrap();
   
    // Renderizamos el html pasándole los valores que queremos substituir
    let html_renderizado= plantilla_prueba.render(context!(nombre => user.first_name,apellidos =>user.last_name,edad=>user.age)).expect("Error al reenderizar el html.");
    crear_pdf(html_renderizado)

}

fn crear_pdf(pdf:String){
    // Iniciamos la aplicacion wkhtmltopdf para crear el pdf
    let pdf_app = PdfApplication::new().expect("Error al iniciar la aplicacion PDF.");
   
    // Generamos el PDF
    let mut pdfout = pdf_app.builder()
   .orientation(Orientation::Landscape)
   .margin(Size::Inches(2))
   .title("PDF de prueba")
   .build_from_html(pdf)
   .expect("Error al crear el pdf");
    
    // Guardamos el PDF
    pdfout.save("./pdf/prueba.pdf").expect("Error al guardar el pdf!");
    println!("PDF guardado correctamente como: prueba.pdf!");
}