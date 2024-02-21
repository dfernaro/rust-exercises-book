use std::io::Write;

#[derive(Debug)]
enum Mes {
    ENERO,
    FEBRERO,
    MARZO,
    ABRIL,
    MAYO,
    JUNIO,
    JULIO,
    AGOSTO,
    SEPTIEMBRE,
    OCTUBRE,
    NOVIEMBRE,
    DICIEMBRE,
}

#[derive(Debug)]
enum Especialidad {
    IT,
    RECURSOS_HUMANOS,
    MARKETING,
}

struct Empleado {
    nombre: String,
    apellidos: String,
    edad: u32,
    especialidad: Especialidad,
    mes: Mes,
}

fn main() {
    // Empleado 1
    let empleado1: Empleado = Empleado {
        nombre: String::from("Luis"),
        apellidos: String::from("Rodríguez"),
        edad: 51,
        especialidad: Especialidad::IT,
        mes: Mes::FEBRERO,
    };

    // Empleado 2
    let empleado2: Empleado = Empleado {
        nombre: String::from("Marina"),
        apellidos: String::from("Flores"),
        edad: 42,
        especialidad: Especialidad::MARKETING,
        mes: Mes::SEPTIEMBRE,
    };

    // Se guardan los empleados en una lista
    let empleados: [Empleado; 2] = [empleado1, empleado2];
    let total = 1;

    // Se crea un fichero llamado empleados.txt
    let mut fichero = std::fs::File::create("empleados.txt").expect("Error durante la creación");

    // Recorrer la lista de empleados y guardar la información
    for empleado in empleados {
        let titulo: String = format!("Empleado {}:\n", total);
        let contenido: String = format!(
            "- {}\n- {}\n- {}\n- {:?}\n- {:?}\n\n",
            empleado.nombre, empleado.apellidos, empleado.edad, empleado.especialidad, empleado.mes
        );

        fichero
            .write_all(titulo.as_bytes())
            .expect("Error durante la escritura");
        fichero
            .write_all(contenido.as_bytes())
            .expect("Error durante la escritura");
    }
}
