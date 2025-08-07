use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_successful_run() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("optimizador-de-rutas")?;

    cmd.arg("-12.11797,-76.98541")
       .arg("-12.10000,-76.99000");

    cmd.assert()
       .success()
       .stdout(predicate::str::contains("Ruta más corta encontrada."))
       .stdout(predicate::str::contains("Distancia total:"));

    Ok(())
}