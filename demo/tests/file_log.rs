use std::path::{Path, PathBuf};
use std::process::Command;

/// Diretório temporário por teste; removido no drop.
struct DirTemporario(PathBuf);

impl DirTemporario {
    fn novo(nome: &str) -> Self {
        let marca = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("relógio antes de 1970")
            .as_nanos();
        let caminho = std::env::temp_dir().join(format!(
            "demo-log-{nome}-{}-{marca}",
            std::process::id()
        ));
        std::fs::create_dir_all(&caminho).expect("criar diretório temporário");
        Self(caminho)
    }

    fn caminho(&self) -> &Path {
        &self.0
    }

    fn arquivos(&self) -> Vec<String> {
        std::fs::read_dir(&self.0)
            .map(|entradas| {
                entradas
                    .filter_map(|entrada| entrada.ok())
                    .map(|entrada| entrada.file_name().to_string_lossy().into_owned())
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl Drop for DirTemporario {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Data UTC do dia, no formato `yyyy-MM-dd` que a rotação usa.
fn data_utc_hoje() -> String {
    let segundos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("relógio antes de 1970")
        .as_secs() as i64;
    let dias = segundos.div_euclid(86_400);
    // `civil_from_days`, de Howard Hinnant.
    let z = dias + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let ano = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let dia = doy - (153 * mp + 2) / 5 + 1;
    let mes = if mp < 10 { mp + 3 } else { mp - 9 };
    let ano = if mes <= 2 { ano + 1 } else { ano };
    format!("{ano:04}-{mes:02}-{dia:02}")
}

fn executar(modo: &str, dir: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_demo"))
        .arg(modo)
        .env("DEMO_LOG_DIR", dir)
        .output()
        .expect("falha ao executar o binário demo")
}

#[test]
fn baseline_com_demo_log_dir_cria_o_arquivo_do_dia() {
    let dir = DirTemporario::novo("baseline-arquivo");
    let out = executar("baseline", dir.caminho());
    assert!(
        out.status.success(),
        "binário falhou: status={:?} stderr={}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );

    let esperado = dir.caminho().join(format!("demo.log.{}", data_utc_hoje()));
    assert!(
        esperado.is_file(),
        "arquivo do dia ausente em {:?}; encontrados: {:?}",
        esperado,
        dir.arquivos()
    );
}

#[test]
fn arquivo_nao_tem_codigos_ansi() {
    let dir = DirTemporario::novo("sem-ansi");
    let out = executar("baseline", dir.caminho());
    assert!(
        out.status.success(),
        "binário falhou: status={:?} stderr={}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );

    let esperado = dir.caminho().join(format!("demo.log.{}", data_utc_hoje()));
    let conteudo = std::fs::read_to_string(&esperado).expect("ler o arquivo do dia");
    assert!(
        !conteudo.contains('\u{1b}'),
        "arquivo contém códigos ANSI: {conteudo:?}"
    );
}

#[test]
fn baseline_arquivo_tem_spans_de_io_e_nao_de_cpu() {
    let dir = DirTemporario::novo("baseline-conteudo");
    let out = executar("baseline", dir.caminho());
    assert!(
        out.status.success(),
        "binário falhou: status={:?} stderr={}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );

    let esperado = dir.caminho().join(format!("demo.log.{}", data_utc_hoje()));
    let conteudo = std::fs::read_to_string(&esperado).expect("ler o arquivo do dia");
    for span in ["handle", "load_customer", "persist"] {
        assert!(
            conteudo.contains(span),
            "span {span} ausente no arquivo: {conteudo:?}"
        );
    }
    for proibido in ["validate", "price", "apply_rules", "render"] {
        assert!(
            !conteudo.contains(proibido),
            "span de CPU {proibido} não deveria estar no arquivo: {conteudo:?}"
        );
    }
}

#[test]
fn aop_arquivo_tem_um_span_por_etapa_sob_handle() {
    let dir = DirTemporario::novo("aop-conteudo");
    let out = executar("aop", dir.caminho());
    assert!(
        out.status.success(),
        "binário falhou: status={:?} stderr={}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );

    let esperado = dir.caminho().join(format!("demo.log.{}", data_utc_hoje()));
    let conteudo = std::fs::read_to_string(&esperado).expect("ler o arquivo do dia");
    for etapa in [
        "validate",
        "load_customer",
        "price",
        "apply_rules",
        "persist",
        "render",
    ] {
        assert!(
            conteudo.contains(etapa),
            "span {etapa} ausente no arquivo: {conteudo:?}"
        );
        assert!(
            conteudo.contains(&format!("handle:{etapa}")),
            "span {etapa} não está sob handle: {conteudo:?}"
        );
    }
    assert!(
        conteudo.contains("time.busy") && conteudo.contains("time.idle"),
        "arquivo sem campos de duração: {conteudo:?}"
    );
}

#[test]
fn terminal_continua_com_o_trace_nos_dois_modos() {
    let casos: [(&str, &[&str]); 2] = [
        ("baseline", &["handle", "load_customer", "persist"]),
        (
            "aop",
            &[
                "handle",
                "validate",
                "load_customer",
                "price",
                "apply_rules",
                "persist",
                "render",
            ],
        ),
    ];

    for (modo, esperados) in casos {
        let dir = DirTemporario::novo(&format!("stdout-{modo}"));
        let out = executar(modo, dir.caminho());
        assert!(
            out.status.success(),
            "binário falhou em {modo}: status={:?} stderr={}",
            out.status,
            String::from_utf8_lossy(&out.stderr)
        );

        let stdout = String::from_utf8_lossy(&out.stdout);
        for span in esperados {
            assert!(
                stdout.contains(span),
                "span {span} ausente no stdout de {modo}: {stdout:?}"
            );
        }
        assert!(
            stdout.contains("time.busy"),
            "stdout de {modo} sem duração: {stdout:?}"
        );
    }
}

#[test]
fn sem_demo_log_dir_usa_logs_sob_o_cwd() {
    let dir = DirTemporario::novo("padrao");
    let out = Command::new(env!("CARGO_BIN_EXE_demo"))
        .arg("baseline")
        .current_dir(dir.caminho())
        .env_remove("DEMO_LOG_DIR")
        .output()
        .expect("falha ao executar o binário demo");
    assert!(
        out.status.success(),
        "binário falhou: status={:?} stderr={}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );

    let esperado = dir
        .caminho()
        .join("logs")
        .join(format!("demo.log.{}", data_utc_hoje()));
    assert!(
        esperado.is_file(),
        "arquivo padrão ausente em {esperado:?}; encontrados: {:?}",
        dir.arquivos()
    );
}

#[test]
fn retencao_mantem_no_maximo_sete_arquivos_diarios() {
    let dir = DirTemporario::novo("retencao");
    for dia in 1..=8 {
        let data = format!("2026-11-{dia:02}");
        std::fs::write(
            dir.caminho().join(format!("demo.log.{data}")),
            b"anterior\n",
        )
        .expect("semear um arquivo antigo");
    }

    let out = executar("baseline", dir.caminho());
    assert!(
        out.status.success(),
        "binário falhou: status={:?} stderr={}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );

    // A crate poda pela data de criação (btime), não pela data do nome; com
    // fallback para a data do nome quando o sistema não informa btime.
    // Arquivos semeados no mesmo instante têm o mesmo btime, então não
    // provam qual some; o contrato que a nossa configuração garante — e o
    // que este teste afirma — é o teto de 7. Com um arquivo por dia, o btime
    // acompanha a data e o mais antigo é o que sai.
    let arquivos = dir.arquivos();
    let diarios: Vec<&String> = arquivos
        .iter()
        .filter(|nome| nome.starts_with("demo.log."))
        .collect();
    assert!(
        diarios.len() <= 7,
        "esperava no máximo 7 arquivos diários, encontrei {}: {diarios:?}",
        diarios.len()
    );
    let hoje = format!("demo.log.{}", data_utc_hoje());
    assert!(
        arquivos.iter().any(|nome| nome == &hoje),
        "o arquivo do dia deveria existir; encontrados: {arquivos:?}"
    );
}
