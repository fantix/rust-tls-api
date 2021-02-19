use crate::socket::AsyncSocket;
use crate::stream_box::TlsStreamBox;
use crate::BoxFuture;
use crate::TlsStream;
use crate::X509Cert;

/// A builder for `TlsConnector`s.
pub trait TlsConnectorBuilder: Sized + Sync + Send + 'static {
    type Connector: TlsConnector;

    type Underlying;

    fn underlying_mut(&mut self) -> &mut Self::Underlying;

    fn set_alpn_protocols(&mut self, protocols: &[&[u8]]) -> crate::Result<()>;

    fn set_verify_hostname(&mut self, verify: bool) -> crate::Result<()>;

    fn add_root_certificate(&mut self, cert: &X509Cert) -> crate::Result<&mut Self>;

    fn build(self) -> crate::Result<Self::Connector>;
}

/// A builder for client-side TLS connections.
pub trait TlsConnector: Sized + Sync + Send + 'static {
    type Builder: TlsConnectorBuilder<Connector = Self>;

    const IMPLEMENTED: bool;
    const SUPPORTS_ALPN: bool;

    fn builder() -> crate::Result<Self::Builder>;

    fn connect<'a, S>(
        &'a self,
        domain: &'a str,
        stream: S,
    ) -> BoxFuture<'a, crate::Result<TlsStream<S>>>
    where
        S: AsyncSocket;

    fn connect_dyn<'a, S>(
        &'a self,
        domain: &'a str,
        stream: S,
    ) -> BoxFuture<'a, crate::Result<TlsStreamBox>>
    where
        S: AsyncSocket,
    {
        BoxFuture::new(async move { self.connect(domain, stream).await.map(TlsStreamBox::new) })
    }
}
