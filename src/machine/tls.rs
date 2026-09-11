// Using this while tls is disabled in warp
mod tls {
    use std::net::SocketAddr;
    pub trait Accept {
        type IO: warp::hyper::rt::Read + warp::hyper::rt::Write + Send + Unpin + 'static;
        type AcceptError: std::fmt::Debug;
        type Accepting: warp::Future<Output = Result<(Self::IO, Option<SocketAddr>), Self::AcceptError>>
            + Send
            + 'static;
        #[allow(async_fn_in_trait)]
        async fn accept(&mut self) -> Result<Self::Accepting, std::io::Error>;
    }

    struct Tls<A>(pub(super) A);

    impl<A: Accept> Accept for Tls<A> {
        type IO = hyper_util::rt::TokioIo<tokio::net::TcpStream>;
        type AcceptError = std::convert::Infallible;
        type Accepting =
           std::future::Ready<Result<(Self::IO, Option<SocketAddr>), Self::AcceptError>>;
        async fn accept(&mut self) -> Result<Self::Accepting, std::io::Error> {
            // TODO: finish this code
            let (io, addr) = self.0.accept().await?;
            Ok(std::future::ready(Ok((
                hyper_util::rt::TokioIo::new(io),
                addr,
            ))))
        }
    }
}