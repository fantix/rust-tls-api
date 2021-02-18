use crate::Error;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use tls_api::runtime::AsyncRead;
use tls_api::runtime::AsyncWrite;
use void::Void;

pub struct TlsConnectorBuilder(Void);
pub struct TlsConnector(Void);

impl tls_api::TlsConnectorBuilder for TlsConnectorBuilder {
    type Connector = TlsConnector;

    type Underlying = Void;

    fn underlying_mut(&mut self) -> &mut Void {
        &mut self.0
    }

    const SUPPORTS_ALPN: bool = false;

    fn set_alpn_protocols(&mut self, _protocols: &[&[u8]]) -> tls_api::Result<()> {
        Err(tls_api::Error::new(Error))
    }

    fn set_verify_hostname(&mut self, _verify: bool) -> tls_api::Result<()> {
        Err(tls_api::Error::new(Error))
    }

    fn add_root_certificate(&mut self, _cert: &tls_api::X509Cert) -> tls_api::Result<&mut Self> {
        Err(tls_api::Error::new(Error))
    }

    fn build(self) -> tls_api::Result<TlsConnector> {
        Err(tls_api::Error::new(Error))
    }
}

impl tls_api::TlsConnector for TlsConnector {
    type Builder = TlsConnectorBuilder;

    fn builder() -> tls_api::Result<TlsConnectorBuilder> {
        Err(tls_api::Error::new(Error))
    }

    fn connect<'a, S>(
        &'a self,
        _domain: &'a str,
        _stream: S,
    ) -> Pin<Box<dyn Future<Output = tls_api::Result<tls_api::TlsStream<S>>> + Send + 'a>>
    where
        S: AsyncRead + AsyncWrite + fmt::Debug + Unpin + Send + Sync + 'static,
    {
        Box::pin(async { Err(tls_api::Error::new(Error)) })
    }
}
