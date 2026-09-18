use bytes::{Bytes, buf::Reader};
use std::sync::{Arc, Condvar, Mutex};
use tokio::sync::Notify;

use warp::{filters::BoxedFilter, http, reply::Reply};

pub struct HttpListener {
    pub incoming: std::sync::mpsc::Receiver<HttpRequest>,
    pub warp_shutdown: Arc<Notify>,
}

pub struct HttpRequest {
    pub request_data: HttpRequestData,
    pub response: HttpResponse,
}

pub type HttpResponse = Arc<(Mutex<bool>, Mutex<Option<warp::reply::Response>>, Condvar)>;

pub struct HttpRequestData {
    pub method: http::Method,
    pub headers: http::HeaderMap,
    pub path: String,
    pub query: String,
    pub body: Reader<Bytes>,
}

// tls
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};

use tokio_rustls::rustls::pki_types::pem::PemObject;
use tokio_rustls::TlsAcceptor;

pub fn init_tls(key: String, cert: String) -> TlsAcceptor {
    let config = Arc::new(
                ServerConfig::builder()
                    .with_no_client_auth()
                    .with_single_cert(
                        CertificateDer::pem_file_iter(&cert).expect("certificate").collect::<Result<_, _>>().expect("certificate loaded"),
                        PrivateKeyDer::from_pem_file(&key).expect("private key"),
                    ).expect("config"),
            );
    TlsAcceptor::from(config)
}

pub fn http_server_tls<Addr>(addr: Addr, shutdown: Arc<Notify>, serve: BoxedFilter<(impl Reply + 'static,)>, key: String, cert: String) -> Result<(), std::io::Error>
where
    Addr: std::net::ToSocketAddrs
{
    let runtime = tokio::runtime::Handle::current();
    let _guard = runtime.enter();

    match std::net::TcpListener::bind(addr) {
        Ok(acceptor) => {
            let _ = acceptor.set_nonblocking(true);

            let tokio_acceptor = tokio::net::TcpListener::from_std(acceptor).expect("TCP socket not async");

            let tls_acceptor = init_tls(key, cert); // tls

            runtime.spawn(async move {
                use hyper::server::conn::http1;
                use hyper_util::rt::TokioIo;

                let listener = tokio_acceptor;
                let graceful = hyper_util::server::graceful::GracefulShutdown::new();
                let mut signal = std::pin::pin!(async move { shutdown.notified().await });

                // We start a loop to continuously accept incoming connections
                loop {
                    tokio::select! {
                        Ok((stream, _addr)) = listener.accept() => {
                            let serve = serve.clone();

                            // tls
                            // TODO: can speedup initialization step, see https://github.com/rustls/tokio-rustls/blob/HEAD/examples/server.rs#L63
                            let stream = tls_acceptor.accept(stream).await;

                            if let Err(err) = stream {
                                eprintln!("Error initializing TLS connection: {:?}", err);
                                continue;
                            }
                            let stream = stream.unwrap();
                            //};
                            // tls end

                            // Spawn a tokio task to serve multiple connections concurrently
                            tokio::task::spawn(async move {
                                // Finally, we bind the incoming connection to our `hello` service
                                if let Err(err) = http1::Builder::new().serve_connection(TokioIo::new(stream), hyper_util::service::TowerToHyperService::new(warp::service(serve))).await
                                {
                                    eprintln!("Error serving connection: {:?}", err);
                                }
                            });
                        },
                        _ = &mut signal => {
                            drop(listener);
                            break;
                        }
                    }                    
                }

                tokio::select! {
                    _ = graceful.shutdown() => {
                        // all connections gracefully closed
                    },
                    _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
                        eprintln!("timed out wait for all connections to close");
                    }
                }
            });

            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}

pub fn http_server<Addr>(addr: Addr, shutdown: Arc<Notify>, serve: BoxedFilter<(impl Reply + 'static,)>) -> Result<(), std::io::Error>
where
    Addr: std::net::ToSocketAddrs
{
    let runtime = tokio::runtime::Handle::current();
    let _guard = runtime.enter();

    match std::net::TcpListener::bind(addr) {
        Ok(acceptor) => {
            let _ = acceptor.set_nonblocking(true);

            let tokio_acceptor = tokio::net::TcpListener::from_std(acceptor).expect("TCP socket not async");

            runtime.spawn(async move {

                // lets use hyper directly!
                use std::convert::Infallible;
                use std::net::SocketAddr;

                use http_body_util::Full;
                use hyper::body::Bytes;
                use hyper::server::conn::http1;
                use hyper::service::service_fn;
                use hyper::{Request, Response};
                use hyper_util::rt::TokioIo;
                use tokio::net::TcpListener;

                let listener = tokio_acceptor;
                let graceful = hyper_util::server::graceful::GracefulShutdown::new();
                let mut signal = std::pin::pin!(async move { shutdown.notified().await });

                // We start a loop to continuously accept incoming connections
                loop {
                    tokio::select! {
                        Ok((stream, _addr)) = listener.accept() => {
                            let serve = serve.clone();

                            // tls begin
                            // let stream = match ssl_server {
                            //     Some((ref key, ref cert)) => {

                            //         use tokio_rustls::rustls::ServerConfig;
                            //         use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};

                            //         use tokio_rustls::rustls::pki_types::pem::PemObject;
                            //         //use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};
                            //         use tokio_rustls::rustls::server::Acceptor;
                            //         use tokio_rustls::server::TlsStream;
                            //         use tokio_rustls::{LazyConfigAcceptor, TlsAcceptor};

                            //         let config = Arc::new(
                            //                 ServerConfig::builder()
                            //                     .with_no_client_auth()
                            //                     .with_single_cert(
                            //                         CertificateDer::pem_file_iter(&cert).expect("certificate").collect::<Result<_, _>>().expect("certificate loaded"),
                            //                         PrivateKeyDer::from_pem_file(&key).expect("private key"),
                            //                     ).expect("config"),
                            //             );
                            //         let acceptor = TlsAcceptor::from(config.clone());

                            //         acceptor.accept(stream)
                            //     },
                            //     None => stream,
                            // };
                            // tls end

                            // Use an adapter to access something implementing `tokio::io` traits as if they implement
                            // `hyper::rt` IO traits.
                            let io = TokioIo::new(stream);

                            // Spawn a tokio task to serve multiple connections concurrently
                            tokio::task::spawn(async move {
                                // Finally, we bind the incoming connection to our `hello` service
                                if let Err(err) = http1::Builder::new()
                                    // `service_fn` converts our function in a `Service`
                                    .serve_connection(io, hyper_util::service::TowerToHyperService::new(warp::service(serve)))
                                    //.serve_connection(io, service_fn(async |_: Request<hyper::body::Incoming>| -> Result<Response<Full<Bytes>>, Infallible> {
                                    //    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
                                    //}))
                                    .await
                                {
                                    eprintln!("Error serving connection: {:?}", err);
                                }
                            });
                        },
                        _ = &mut signal => {
                            drop(listener);
                            //eprintln!("graceful shutdown signal received");
                            // stop the accept loop
                            break;
                        }
                    }

                //    let (stream, _) = listener.accept().await.expect("not connected");
                    
                }

                tokio::select! {
                    _ = graceful.shutdown() => {
                        //eprintln!("all connections gracefully closed");
                    },
                    _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
                        eprintln!("timed out wait for all connections to close");
                    }
                }
            });

            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}