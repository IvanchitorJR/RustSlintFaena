use crate::modelo::Usuario;
use reqwest::Client;
use std::sync::Arc;
use slint::Weak;

use crate::MainWindow;

pub async fn verificar_credenciales(
    cliente: Arc<Client>,
    correo: String,
    password: String,
    ui_handle: Weak<MainWindow>
){
    let url = "https://faena-backend.onrender.com/api/usuarios"; //url para la api rest con los usuarios
    match cliente.get(url).send().await { //enviamos la peticion y vemos las posibles respuestas
        Ok(res) => { //si recibimos una respuesta
            match res.json::<Vec<Usuario>>().await {
                Ok(usuarios)=>{ //si existen usuarios o pudo la app recibir y deserializar la respuesta en un vector de usuarios(struct)
                    let mut encontrado: bool = false;
                    for usuario in usuarios{
                        if usuario.email==correo && usuario.contrase_a==password{
                            encontrado=true;
                            break;
                        }
                    } 
                    slint::invoke_from_event_loop(move ||{
                        if let Some(ui) = ui_handle.upgrade(){
                            if encontrado{
                                ui.set_empty_error("Bienvenido a Faena".into());
                                println!("Welcome to faena");
                                ui.set_pantalla_actual(3); //pantalla startMenu 
                            }
                            else{
                                ui.set_empty_error("Usuario y/o contrasena incorrectas".into());
                                ui.set_empty_error_text_color(ui.get_rojo());
                                ui.set_input_border_color(ui.get_border());
                                println!("No hay");
                            }
                        }
                    }).unwrap();
                }
                Err(err)=>{
                    eprintln!("Error al recibir los usuarios: {:?}", err);
                }
            }
        }
        Err(err)=>{
                eprint!("Error al hacer la peticion, {:?}", err);
            }
    }
}