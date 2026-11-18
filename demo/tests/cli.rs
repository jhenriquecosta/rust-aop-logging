use std::process::Command;

#[test]
fn baseline_imprime_o_trace_e_sai_com_zero() {
    let out = Command::new(env!("CARGO_BIN_EXE_demo"))
        .arg("baseline")
        .output()
        .expect("falha ao executar o binário demo");

    assert!(
        out.status.success(),
        "binário falhou: status={:?} stderr={}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    for esperado in ["handle", "load_customer", "persist"] {
        assert!(stdout.contains(esperado), "span {esperado} ausente em: {stdout}");
    }
    for proibido in ["price", "apply_rules", "render"] {
        assert!(
            !stdout.contains(proibido),
            "span de CPU {proibido} não deveria aparecer em: {stdout}"
        );
    }
    assert!(
        stdout.contains("time.busy"),
        "saída sem campos de duração (time.busy): {stdout}"
    );
}
