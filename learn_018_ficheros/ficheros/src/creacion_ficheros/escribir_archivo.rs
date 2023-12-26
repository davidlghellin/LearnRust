use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    const CONTENIDO: &str = "Bacon ipsum dolor amet salami t-bone picanha jowl,
turducken pastrami fatback. Pig beef pancetta corned beef andouille rump ground round.
Sausage pork leberkas, drumstick turkey shankle brisket ball tip salami. Shoulder beef
pastrami venison bacon rump sirloin. Filet mignon venison flank ground round spare
ribs boudin shankle buffalo hamburger chislic. Kevin kielbasa corned beef tail burgdoggen
biltong salami, leberkas porchetta.";

    // Esta vez vamos a crear un archivo, pero
    // la referencia de su creacion la vamos a guardar
    // en una variable a la cual se le puede modificar el contenido.
    let mut archivo = File::create("ejemplo.txt")?;

    // Ahora vamos a escribir el contenido, es importante notar que el texto hay que pasarlo como
    // bytes, otra cosa importante a notar es que cuando el programa termine o se termine el scope
    // si fuera una funcion, entonces Rust cerrara el archivo por nosotros, no es necesario que lo
    // hagamos explicitamente
    archivo.write_all(CONTENIDO.as_bytes())?;

    Ok(())
}
