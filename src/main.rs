use wkhtmltopdf::*;
use std::fs;
use minijinja::{Environment, context};

#[tokio::main]
async fn main() {
    // Leemos la plantilla html
    let plantilla_html = fs::read_to_string("../plantillas/ejemplo.html").expect("Error al leer el archivo.");

    // Creamos la plantilla prueba
    let mut env = Environment::new();
    env.add_template("prueba", &plantilla_html).unwrap();
    
    // seleccionamos la plantilla prueba
    let plantilla_prueba = env.get_template("prueba").unwrap();
    
    // Reenderizamos el pdf pasandole los valores que queremos substituir
    let pdf_renderizado= plantilla_prueba.render(context!(nombre => "Zacarias",apellidos =>"Pastor Díaz",edad=>68)).unwrap();
    
    // Iniciamos la aplicacion wkhtmltopdf para crear el pdf
    let pdf_app = PdfApplication::new().expect("Error al iniciar la aplicacion PDF.");
    
    // Generamos el PDF
    let mut pdfout = pdf_app.builder()
        .orientation(Orientation::Landscape)
        .margin(Size::Inches(2))
        .title("PDF de prueba")
        .build_from_html(pdf_renderizado)
        .expect("fallo al crear el pdf");

    // Guardamos el pdf
    pdfout.save("../pdf/prueba.pdf").expect("Error al guardar el pdf!");
    println!("PDF guardado correctamente como: prueba.pdf");
    
}