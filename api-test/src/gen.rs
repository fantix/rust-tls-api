use std::env;
use std::fs;

const TEMPLATE: &str = "\
use tls_api::TlsAcceptor;
use tls_api::TlsConnector;

#[test]
fn version() {
    tls_api_test::test_version::<CRATE::TlsConnector, CRATE::TlsAcceptor>();
}

#[test]
fn google() {
    tls_api_test::test_google::<CRATE::TlsConnector>();
}

#[test]
fn connect_bad_hostname() {
    tls_api_test::connect_bad_hostname::<CRATE::TlsConnector, _>(drop);
}

#[test]
fn connect_bad_hostname_ignored() {
    tls_api_test::connect_bad_hostname_ignored::<CRATE::TlsConnector>();
}

#[test]
fn client_server_der() {
    tls_api_test::test_client_server_der::<
        CRATE::TlsConnector,
        CRATE::TlsAcceptor,
    >();
}

#[test]
fn client_server_dyn_der() {
    tls_api_test::test_client_server_dyn_der(
        CRATE::TlsConnector::TYPE_DYN,
        CRATE::TlsAcceptor::TYPE_DYN,
    );
}

#[test]
fn client_server_pkcs12() {
    tls_api_test::test_client_server_pkcs12::<
        CRATE::TlsConnector,
        CRATE::TlsAcceptor,
    >();
}

#[test]
fn alpn() {
    tls_api_test::test_alpn::<CRATE::TlsConnector, CRATE::TlsAcceptor>()
}
";

/// Called from impl crates to generate the common set of tests
pub fn gen_tests() {
    let crate_name = env::var("CARGO_PKG_NAME").unwrap().replace("-", "_");

    let out_dir = env::var("OUT_DIR").unwrap();

    let g = TEMPLATE.replace("CRATE", &crate_name);
    let g = format!("// {}generated\n\n{}", "@", g);

    fs::write(format!("{}/tests_generated.rs", out_dir), g).unwrap();
}
