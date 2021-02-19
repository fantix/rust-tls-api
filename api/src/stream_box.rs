use crate::runtime::AsyncRead;
use crate::runtime::AsyncWrite;
use crate::socket::AsyncSocket;
use crate::TlsStream;
use std::fmt;
use std::io;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

trait TlsStreamBoxDyn: AsyncRead + AsyncWrite + fmt::Debug + Unpin + 'static {
    fn get_alpn_protocol(&self) -> crate::Result<Option<Vec<u8>>>;
}

impl<S: AsyncSocket> TlsStreamBoxDyn for TlsStream<S> {
    fn get_alpn_protocol(&self) -> crate::Result<Option<Vec<u8>>> {
        self.get_alpn_protocol()
    }
}

/// Similar to [`TcpStream`], but without a type parameter.
///
/// Make writing code slightly more concise at cost of some runtime overhead:
/// * extra allocation per connection
/// * extra indirect invocation per operation
#[derive(Debug)]
pub struct TlsStreamBox(Box<dyn TlsStreamBoxDyn>);

impl TlsStreamBox {
    pub fn new<S: AsyncSocket>(stream: TlsStream<S>) -> TlsStreamBox {
        TlsStreamBox(Box::new(stream))
    }

    pub fn get_alpn_protocol(&self) -> crate::Result<Option<Vec<u8>>> {
        self.0.get_alpn_protocol()
    }
}

impl AsyncRead for TlsStreamBox {
    #[cfg(feature = "runtime-tokio")]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().0).poll_read(cx, buf)
    }

    #[cfg(feature = "runtime-async-std")]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.get_mut().0).poll_read(cx, buf)
    }
}

impl AsyncWrite for TlsStreamBox {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.get_mut().0).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().0).poll_flush(cx)
    }

    #[cfg(feature = "runtime-tokio")]
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().0).poll_shutdown(cx)
    }

    #[cfg(feature = "runtime-async-std")]
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().0).poll_close(cx)
    }
}
