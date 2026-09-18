// Using this while tls is disabled in warp
pub mod tls {
    //use std::net::SocketAddr;
    // pub trait Accept {
    //     type IO: warp::hyper::rt::Read + warp::hyper::rt::Write + Send + Unpin + 'static;
    //     type AcceptError: std::fmt::Debug;
    //     type Accepting: warp::Future<Output = Result<(Self::IO, Option<SocketAddr>), Self::AcceptError>>
    //         + Send
    //         + 'static;
    //     #[allow(async_fn_in_trait)]
    //     async fn accept(&mut self) -> Result<Self::Accepting, std::io::Error>;
    // }

    // pub struct Tls(pub tokio::net::TcpListener);

    // impl warp::server::accept::Accept for Tls {
    //     type IO = hyper_util::rt::TokioIo<tokio::net::TcpStream>;
    //     type AcceptError = std::convert::Infallible;
    //     type Accepting =
    //         std::future::Ready<Result<(Self::IO, Option<SocketAddr>), Self::AcceptError>>;
    //     async fn accept(&mut self) -> Result<Self::Accepting, std::io::Error> {
    //         let (io, addr) = <tokio::net::TcpListener>::accept(&self.0).await?;
    //         Ok(std::future::ready(Ok((
    //             hyper_util::rt::TokioIo::new(io),
    //             Some(addr),
    //         ))))
    //     }
    // }

    // impl<A: Accept> Accept for Tls<A> {
    //     type IO = hyper_util::rt::TokioIo<tokio::net::TcpStream>;
    //     type AcceptError = std::convert::Infallible;
    //     type Accepting =
    //        std::future::Ready<Result<(Self::IO, Option<SocketAddr>), Self::AcceptError>>;
    //     async fn accept(&mut self) -> Result<Self::Accepting, std::io::Error> {
    //         // TODO: finish this code
    //         let (io, addr): (Self::IO, Option<SocketAddr>) = self.0.accept().await?;
    //         Ok(std::future::ready(Ok((
    //             hyper_util::rt::TokioIo::new(io),
    //             addr,
    //         ))))
    //     }
    // }
}

// fn foo() {
//     tls::Tls();
// }