//#[macro_use]
//extern crate cfg_if;

use std::error;
use std::io;
use std::net::ToSocketAddrs;

use futures::Future;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_io::io::{flush, read_to_end, write_all};

use env_logger;

use tls_api;

use tls_api::TlsConnectorBuilder as tls_api_TlsConnectorBuilder;

use tokio_tls_api;

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => panic!("{} failed with {:?}", stringify!($e), e),
        }
    };
}

/*
cfg_if! {
    if #[cfg(feature = "force-rustls")] {
        fn assert_bad_hostname_error(err: &Error) {
            let err = err.to_string();
            assert!(err.contains("CertNotValidForName"), "bad error: {}", err);
        }
    } else if #[cfg(any(feature = "force-openssl",
                        all(not(target_os = "macos"),
                            not(target_os = "windows"),
                            not(target_os = "ios"))))] {
        extern crate openssl;

        use openssl::ssl;
        use tls_api::backend::openssl::ErrorExt;

        fn assert_bad_hostname_error(err: &Error) {
            let err = err.get_ref().unwrap();
            let err = err.downcast_ref::<tls_api::Error>().unwrap();
            let errs = match *err.openssl_error() {
                ssl::Error::Ssl(ref v) => v,
                ref e => panic!("not an ssl eror: {:?}", e),
            };
            assert!(errs.errors().iter().any(|e| {
                e.reason() == Some("certificate verify failed")
            }), "bad errors: {:?}", errs);
        }
    } else if #[cfg(any(target_os = "macos", target_os = "ios"))] {
        use tls_api::backend::security_framework::ErrorExt;

        fn assert_bad_hostname_error(err: &Error) {
            let err = err.get_ref().unwrap();
            let err = err.downcast_ref::<tls_api::Error>().unwrap();
            let err = err.security_framework_error();
            assert_eq!(err.message().unwrap(), "The trust policy was not trusted.");
        }
    } else {
        extern crate winapi;

        use tls_api::backend::schannel::ErrorExt;

        fn assert_bad_hostname_error(err: &Error) {
            let err = err.get_ref().unwrap();
            let err = err.downcast_ref::<tls_api::Error>().unwrap();
            let err = err.schannel_error();
            let code = err.raw_os_error().unwrap();
            assert_eq!(code as usize, winapi::CERT_E_CN_NO_MATCH as usize);
        }
    }
}
*/

fn native2io(e: tls_api::Error) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e)
}

pub fn fetch_google<C: tls_api::TlsConnector>() {
    drop(env_logger::try_init());

    // First up, resolve google.com
    let addr = t!("google.com:443".to_socket_addrs()).next().unwrap();

    // Create an event loop and connect a socket to our resolved address.c
    let mut l = t!(Core::new());
    let client = TcpStream::connect(&addr, &l.handle());

    // Send off the request by first negotiating an SSL handshake, then writing
    // of our request, then flushing, then finally read off the response.
    let data = client
        .and_then(move |socket| {
            let builder = t!(C::builder());
            let connector = t!(builder.build());
            tokio_tls_api::connect_async(&connector, "google.com", socket).map_err(native2io)
        })
        .and_then(|socket| write_all(socket, b"GET / HTTP/1.0\r\n\r\n"))
        .and_then(|(socket, _)| flush(socket))
        .and_then(|socket| read_to_end(socket, Vec::new()));

    let (_, data) = t!(l.run(data));

    // any response code is fine
    assert!(data.starts_with(b"HTTP/1.0 "));

    let data = String::from_utf8_lossy(&data);
    let data = data.trim_end();
    assert!(data.ends_with("</html>") || data.ends_with("</HTML>"));
}

// see comment in bad.rs for ignore reason
//#[cfg_attr(all(target_os = "macos", feature = "force-openssl"), ignore)]
//#[test]
pub fn wrong_hostname_error<C: tls_api::TlsConnector>() -> tls_api::Error {
    drop(env_logger::try_init());

    let addr = t!("google.com:443".to_socket_addrs()).next().unwrap();

    let mut l = t!(Core::new());
    let client = TcpStream::connect(&addr, &l.handle());
    let data = client.and_then(move |socket| {
        let builder = t!(C::builder());
        let connector = t!(builder.build());
        tokio_tls_api::connect_async(&connector, "rust-lang.org", socket).map_err(native2io)
    });

    let res = l.run(data);
    assert!(res.is_err());
    let err: io::Error = res.unwrap_err();
    let inner: Box<error::Error> = err.into_inner().unwrap();
    *inner.downcast().unwrap()
}
