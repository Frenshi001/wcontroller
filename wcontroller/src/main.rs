use std::process::Command;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::fs;
use std::thread;
use std::time::Duration;

/////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////// PACKAGE ZONE ///////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////

///Esta función crea el directorio principal donde se van a alojar los distintos archivos de configuración 
///también crea dentro del directorio "Package" el archivo de configuración "package.cfg" donde se aloja el estándar de paquetes 
///instalados actualmente para ser comparados a futuro además de mostrar la cantidad de paquetes disponibles. 
fn package_list() -> std::io::Result<()> {
	let _ = fs::create_dir("/etc/wcontroller")?;
	let _ = fs::create_dir("/etc/wcontroller/Packages")?;
	let directorio = "/etc/wcontroller/Packages";
	let patron = "package.cfg";
	let first_route = format!("{}/{}", directorio, patron);
	let info_package = Command::new("sh")
		.arg("-c")
		.arg("apt list --installed | awk -F/ '{print $1}'")
		.output()
		.expect("Error al ejecutar el comando");
	
	let convert_str = String::from_utf8(info_package.stdout)
		.expect("No se pudo convertir");
	
	if Path::new(&first_route).exists() {
		println!("...");
	} else {
		let mut _archivo_nuevo = File::create(&first_route)?;
		_archivo_nuevo.write_all(b"== LISTA DE PAQUETES INSTALADOS ===\n\n")?;
		_archivo_nuevo.write_all(convert_str.as_bytes())?;
		thread::sleep(Duration::from_secs(2));
		println!("Archivo de configuracion de paquetes creado con exito");
		println!("Paquetes instalados actualmente: {}\n",convert_str.lines().count());
	    thread::sleep(Duration::from_secs(1));
	}
	
	Ok (())
}

///Función encargada de tomar el archivo de configuración temporal y compararlo contra el de configuración estándar.
fn get_diferencias(a: &str, b: &str) -> String {
            Command::new("grep")
            .args(["-Fxv", "-f", a, b])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().into())
            .unwrap_or_default()
}

///Función encargada de crear el archivo de configuración temporal "package_list.cfg", vaciar la información de los paquetes,
///comparar la información obtenida usando la función "get_diferencias", los resultados se guardan en una variable "diffs" y
///por último se elimina el archivo temporal.
fn compare_package() -> std::io::Result<String> {
        let directorio = "/tmp/";
        let patron = "package_list.cfg";
        let ruta_completa2 = format!("{}/{}", directorio, patron);
        
	    let info_package = Command::new("sh")
                .arg("-c")
                .arg("apt list --installed | awk -F/ '{print $1}'")
                .output()
                .expect("Error al ejecutar el comando");

        let convert_str = String::from_utf8(info_package.stdout)
                .expect("No se pudo convertir");

        let mut _archivo_nuevo = File::create(&ruta_completa2)?;
        _archivo_nuevo.write_all(b"== LISTA DE PAQUETES INSTALADOS ===\n\n")?;
        _archivo_nuevo.write_all(convert_str.as_bytes())?;

	let directorio = "/etc/wcontroller/Packages";
	let patron = "package.cfg";
	let first_route = format!("{}/{}", directorio, patron);
        
        let diffs = get_diferencias(&first_route, &ruta_completa2);
        fs::remove_file(&ruta_completa2)
		.expect("no se pudo borrar");
        Ok (diffs)
}

/// "delete_package" se encarga de tomar el resultado de la función anterior, determinar si posee contenido o esta vacía
///dependiendo del resultado se muestra un mensaje de verificación para luego mostrar una lista de paquetes que serán
///eliminados, una vez culminada la eliminación muestra un mensaje de éxito; si la función consultada posee un resultado vacío
/// simplemente muestra un mensaje que indica ausencia de paquetes instalados de manera externa.
fn delete_package() -> std::io::Result<()> {
	let diffs = compare_package()?;
	let _cantidad_lineas = diffs.lines().filter(|l| !l.trim().is_empty()).count();
	if diffs > 0.to_string() {
	    println!("Verificando paquetes:\n");
	    thread::sleep(Duration::from_secs(2));
	    println!("Paquetes instalados de manera externa :");
	    println!("***************************************\n");
	    println!("{}\n", diffs);
	    thread::sleep(Duration::from_secs(5));
	    for delete in diffs.lines() {
		    let delete = delete.trim();
		    if delete.is_empty() {
			    continue;
		    }
		
		    println!("Eliminando paquete: {}", delete);
		
		    let _output = Command::new("sudo")
			    .arg("apt")
			    .arg("remove")
			    .arg("--purge")
			    .arg("-y")
			    .arg(delete)
			    .output()
			    .expect("fallo al ejecutar el comando");
	    }
	    println!("");
	    println!("Todos los paquetes externos han sido eliminados");
	    thread::sleep(Duration::from_secs(2));
	    Command::new("clear").status().unwrap();
	    let _ = bienvenida ();
	    
    } else {
        println!("No existen paquetes externos\n");
        thread::sleep(Duration::from_secs(2));
        Command::new("clear").status().unwrap();
        let _ = bienvenida ();
    }
    Ok(())    
}

/////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////// DOCUMENTS ZONE ///////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////

///crea dentro del directorio "Documents" el archivo de configuración "documents.cfg" donde se aloja el estándar de archivos y 
///documentos actualmente para ser comparados a futuro, seguido de un mensaje de éxito al crear.
fn documents_list() -> std::io::Result<()> {
	    let _ = fs::create_dir("/etc/wcontroller/Documents")?;
        let directorio = "/etc/wcontroller/Documents";
        let patron = "documents.cfg";
        let second_route = format!("{}/{}", directorio, patron);
        let list = vec!["1000", "1001"];
        
        if Path::new(&second_route).exists() {
            println!("...")
            } else {
                let mut _archivo_nuevo = File::create(&second_route)?;
                _archivo_nuevo.write_all(b"== LISTA DE DOCUMENTOS EN EL EQUIPO ===\n\n")?;
                for iter in &list {
                    let comando = format!("cat /etc/passwd | grep {} | awk -F: '{{print $6}}'", iter);
                    let info_number_one = Command::new("sh")
                        .arg("-c")
                        .arg(&comando)
                        .output()
                        .expect("Error al ejecutar el comando");
            
                    let directorio_usuario = String::from_utf8(info_number_one.stdout)
                    .expect("Error al convertir")
                    .trim()
                    .to_string();
            
            
                    let find = Command::new("find")
                        .arg(&directorio_usuario)
                        .output()
                        .expect("Error al ejecutar");
                
                    let search = String::from_utf8(find.stdout)
		                .expect("No se pudo convertir");
		        
                      
                    
                    _archivo_nuevo.write_all(format!("  {}\n\n", search).as_bytes())?;
                    
                   
                }     
                thread::sleep(Duration::from_secs(1));
                println!("Archivo de configuración de documentos creado con éxito\n");
            }
        
        
        Ok (())
}

///Crea un archivo temporal donde vacía la información de los documentos y archivos para luego ser
///comparada con el archivo de configuración original; una vez lleno se invoca la función
///"get_diferencias" para conseguir las diferencias entre los archivos y guardarlas dentro de la variable
///"diffs", por último se elimina el archivo temporal y se retorna "diffs". 
fn compare_documents() -> std::io::Result<String> {
        let directorio = "/tmp/";
        let patron = "documents_list.cfg";
        let ruta_completa3 = format!("{}/{}", directorio, patron);
        let list = vec!["1000", "1001"];
        
        let mut _archivo_nuevo = File::create(&ruta_completa3)?;
        _archivo_nuevo.write_all(b"== LISTA DE DOCUMENTOS EN EL EQUIPO ===\n\n")?;
        for iter in &list {
            let comando = format!("cat /etc/passwd | grep {} | awk -F: '{{print $6}}'", iter);
            let info_number_one = Command::new("sh")
                .arg("-c")
                .arg(&comando)
                .output()
                .expect("Error al ejecutar el comando");
            
            let directorio_usuario = String::from_utf8(info_number_one.stdout)
            .expect("Error al convertir")
            .trim()
            .to_string();
            
            
            let find = Command::new("find")
                .arg(&directorio_usuario)
                .output()
                .expect("Fallo en ejecutar");
                
            let search = String::from_utf8(find.stdout)
		        .expect("No se pudo convertir");
		        
                    
                    
            _archivo_nuevo.write_all(format!("  {}\n\n", search).as_bytes())?;        
        }

	let directorio = "/etc/wcontroller/Documents";
	let patron = "documents.cfg";
	let second_route = format!("{}/{}", directorio, patron);
        
        let diffs = get_diferencias(&second_route, &ruta_completa3);
        fs::remove_file(&ruta_completa3)
		.expect("no se pudo borrar");
        Ok (diffs)
}

///Declara una variable con el mismo nombre del retorno de la función anterior "diffs", dentro de ella 
///se encuentra el resultado obtenido por la función "get_diferencias" luego se filtra para contar la 
///cantidad de lineas dentro de la variable; una vez comprobada se toma una decisión...
///-si la cantidad es mayor a cero "0" se verifica, muestra la lista de archivos y documentos a eliminar,
///elimina y por ultimo muestra un mensaje de éxito al eliminar.
///-si la cantidad es igual a cero "0" imprime en pantalla "no existen documentos externos".
fn delete_documents() -> std::io::Result<()> {
	let diffs = compare_documents()?;
	let _cantidad_lineas = diffs.lines().filter(|l| !l.trim().is_empty()).count();
	if _cantidad_lineas > 0 {
	    println!("Verificando documentos:\n");
	    thread::sleep(Duration::from_secs(2));
	    println!("Lista de documentos creados de manera externa :");
	    println!("***********************************************\n");
	    println!("{}\n", diffs);
	    thread::sleep(Duration::from_secs(5));
	    for delete in diffs.lines() {
		    let delete = delete.trim();
		    if delete.is_empty() {
			    continue;
		    }
		
		    println!("Eliminando documento: {}", delete);
		
		    let _output = Command::new("rm")
			    .arg(delete)
			    .output()
			    .expect("fallo al ejecutar el comando");
	    }
	    println!("");
	    println!("Todos los documentos externos han sido eliminados");
	    thread::sleep(Duration::from_secs(2));
	    Command::new("clear").status().unwrap();
	    let _ = bienvenida ();
	    
    } else {
        println!("No existen documentos externos");
        thread::sleep(Duration::from_secs(2));
        Command::new("clear").status().unwrap();
        let _ = bienvenida ();
    }
    Ok(())    
}


/////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////// USERS ZONE ////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////

fn users_list() -> std::io::Result<()> {
	let _ = fs::create_dir("/etc/wcontroller/Users")?;
	let directorio = "/etc/wcontroller/Users";
	let patron = "users.cfg";
	let third_route = format!("{}/{}", directorio, patron);
	let name_users = Command::new("sh")
		.arg("-c")
		.arg("cat /etc/passwd | grep /home | awk -F: '{print $1}'")
		.output()
		.expect("Error al ejecutar el comando");
	
	let convert_str = String::from_utf8(name_users.stdout)
		.expect("No se pudo convertir");
	
	if Path::new(&third_route).exists() {
		println!("...")
	} else {
		let mut _archivo_nuevo = File::create(&third_route)?;
		_archivo_nuevo.write_all(b"== LISTA DE USUARIOS OFICIALES ===\n\n")?;
		_archivo_nuevo.write_all(convert_str.as_bytes())?;
		thread::sleep(Duration::from_secs(1));
		println!("Archivo de configuracion de usuarios creado con exito\n");
	    
	}
	
	Ok (())
}

fn compare_users() -> std::io::Result<String> {
        
        let directorio = "/tmp/";
        let patron = "users_list.cfg";
        let ruta_completa4 = format!("{}/{}", directorio, patron);
	    let info_users = Command::new("sh")
                .arg("-c")
                .arg("cat /etc/passwd | grep /home | awk -F: '{print $1}'")
                .output()
                .expect("Error al ejecutar el comando");

        let convert_str = String::from_utf8(info_users.stdout)
                .expect("No se pudo convertir");

        let mut _archivo_nuevo = File::create(&ruta_completa4)?;
        _archivo_nuevo.write_all(b"== LISTA DE USUARIOS OFICIALES ===\n\n")?;
        _archivo_nuevo.write_all(convert_str.as_bytes())?;

	    let directorio = "/etc/wcontroller/Users";
	    let patron = "users.cfg";
	    let third_route = format!("{}/{}", directorio, patron);
        
        let diffs = get_diferencias(&third_route, &ruta_completa4);
        fs::remove_file(&ruta_completa4)
		.expect("no se pudo borrar");
        Ok (diffs)
}

fn delete_users() -> std::io::Result<()> {
	let diffs = compare_users()?;
	let _cantidad_lineas = diffs.lines().filter(|l| !l.trim().is_empty()).count();
	if _cantidad_lineas > 0 {
	    println!("Verificando usuarios:\n");
	    thread::sleep(Duration::from_secs(2));
	    println!("Usuarios creados de manera externa :");
        println!("************************************\n");
	    println!("{}\n", diffs);
	    thread::sleep(Duration::from_secs(5));
	    for delete in diffs.lines() {
		    let delete = delete.trim();
		    if delete.is_empty() {
			    continue;
		    }
		
		    println!("Eliminando usuario: {}", delete);
		
		    let eliminar_procesos = Command::new("pkill")
                .args(&["-KILL", "-u", delete])
                .output()?;
            
            // pkill retorna 0 si mató procesos ó 1 si no encontró procesos
            if eliminar_procesos.status.code() == Some(1) {
                println!("No se encontraron procesos para el usuario: {}\n", delete);
            } else if eliminar_procesos.status.success() {
                println!("Procesos terminados exitosamente\n");
            }
		    
		    let _magic_trick = Command::new("sudo")
			    .arg("userdel")
			    .arg("-f")
			    .arg("-r")
			    .arg(delete)
			    .output()
			    .expect("fallo al ejecutar el comando");
	    }
	    println!("");
	    thread::sleep(Duration::from_secs(2));
	    println!("Todos los usuarios externos han sido eliminados\n");
	    thread::sleep(Duration::from_secs(4));
	    Command::new("clear").status().unwrap();
	    
    } else {
        println!("No existen usuarios externos");
        thread::sleep(Duration::from_secs(2));
        Command::new("clear").status().unwrap();
        
    }
    Ok(())    
}

///Muestra la cabecera de bienvenida y la versión del software.
fn bienvenida () {
	println!("╔══════════════════════════════════════╗");
	println!("║                                      ║");
	println!("║       BIENVENIDA/O AL SISTEMA        ║");
	println!("║   DE VERIFICACION Y ADMINISTRACION   ║");
	println!("║         WSCONTROLLER v0.1.0          ║");
	println!("║                                      ║");
	println!("╚══════════════════════════════════════╝");
	thread::sleep(Duration::from_secs(1));
	
}

///controla el orden de ejecuciones del software, posee una comprobación de los archivos de configuración
///para determinar si existen o no, si es positivo ejecuta las verificaciones con los archivos temporales
///y proceder con las eliminaciones pertinentes; si es negativa procede a crear los archivos iniciales de
///configuración.
///Continua imprimiendo en pantalla la forma de un dedo indicando que todo se ejecutó con éxito y un 
///mensaje de agradecimiento.
fn main () -> std::io::Result<()> {
    let _ = bienvenida ();
	let pack = Path::new ("/etc/wcontroller/Packages/package.cfg");
	let doc = Path::new ("/etc/wcontroller/Documents/documents.cfg");
	let usr = Path::new ("/etc/wcontroller/Users/users.cfg");
	
	if pack.exists() && doc.exists() && usr.exists() {
	    println!("...\n");
	    let _ = delete_package();
	    let _ = delete_documents();
	    let _ = delete_users();
	
	} else {
	    println!();
	    println!("Creando instancias necesarias para el funcionamiento del software\n");
	    let _ = package_list(); 
	    let _ = documents_list();
	    let _ = users_list();
	    thread::sleep(Duration::from_secs(1));
	    
	}
	println!("  ╔═╗\n  ║ ║\n  ║ ╚══════\n ║      ═══╝\n ║      ════╝\n ║      ═══╝\n ╚════════╝\n");
    println!("Muchas gracias...");
    thread::sleep(Duration::from_secs(1));
	
	Ok(())
}
