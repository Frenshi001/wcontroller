use std::process::Command;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::fs;
use std::thread;
use std::time::Duration;

fn package_list() -> std::io::Result<()> {
	let directorio = "/home/wlopez";
	let patron = "package.txt";
	let first_route = format!("{}/{}", directorio, patron);
	let info_package = Command::new("sh")
		.arg("-c")
		.arg("apt list --installed | awk -F/ '{print $1}'")
		.output()
		.expect("Error al ejecutar el comando");
	
	let convert_str = String::from_utf8(info_package.stdout)
		.expect("No se pudo convertir");
	
	if Path::new(&first_route).exists() {
		println!("...")
	} else {
		let mut _archivo_nuevo = File::create(&first_route)?;
		_archivo_nuevo.write_all(b"== LISTA DE PAQUETES INSTALADOS ===\n\n")?;
		_archivo_nuevo.write_all(convert_str.as_bytes())?;
		println!("Archivo package creado con éxito");
		println!("total de paquetes: {}",convert_str.lines().count());
	}
	
	Ok (())
}

fn get_diferencias(a: &str, b: &str) -> String {
            Command::new("grep")
            .args(["-Fxv", "-f", a, b])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().into())
            .unwrap_or_default()
}

fn compare_package() -> std::io::Result<String> {
        let directorio = "/home/wlopez";
        let patron = "package_list.txt";
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

	let directorio = "/home/wlopez";
	let patron = "package.txt";
	let first_route = format!("{}/{}", directorio, patron);
        let patron = "package_list.txt";
        let ruta_completa2 = format!("{}/{}", directorio, patron);
        
        let diffs = get_diferencias(&first_route, &ruta_completa2);
//	println!("Diferencias:\n{}", diffs);
        fs::remove_file(&ruta_completa2)
		.expect("no se pudo borrar");
        Ok (diffs)
}

fn delete_package() -> std::io::Result<()> {
	let diffs = compare_package()?;
	let _cantidad_lineas = diffs.lines().filter(|l| !l.trim().is_empty()).count();
	if diffs > 0.to_string() {
	    println!("Lista de paquetes instalados de manera externa :");
	    println!("");
	    println!("{}\n", diffs);
	    println!("");
	    thread::sleep(Duration::from_secs(3));
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
	    println!("Todos los paquetes externos fueron eliminados");
	    
	    
    } else {
        println!("");
    }
    Ok(())    
}

// main() original funciona
fn main () -> std::io::Result<()> {
	let _ = package_list();
	let _ = delete_package();  // ← Ahora sin parámetro
	Ok(())
}
