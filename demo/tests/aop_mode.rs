use std::process::Command;

#[test]
fn aop_imprime_um_span_por_etapa() {
    let out = Command::new(env!("CARGO_BIN_EXE_demo"))
        .arg("aop")
        .output()
        .expect("falha ao executar o binário demo");

    assert!(
        out.status.success(),
        "binário falhou: status={:?} stderr={}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    for esperado in [
        "handle",
        "validate",
        "load_customer",
        "price",
        "apply_rules",
        "persist",
        "render",
    ] {
        assert!(stdout.contains(esperado), "span {esperado} ausente em: {stdout}");
    }
}
