use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use lazy_static::lazy_static;
use std::path::{Path, PathBuf};
use tokio::fs;
use futures::future::FutureExt;

mod settings;
use settings::Settings;


lazy_static! {
	static ref CFG: Settings = Settings::new().unwrap();
}


async fn req_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

	let pathstr: String = (&CFG.service.homedir).to_owned() + req.uri().path(); 
	let path = PathBuf::from(&pathstr).canonicalize();
		
	
	match (req.method(), path) {
		(&Method::GET, Ok(p)) if p.starts_with(&CFG.service.homedir)
			&& p.is_file()
		=> {
			
			Ok(Response::new(Body::wrap_stream(fs::read(pathstr).into_stream())))
	
		}
		(&Method::GET, Ok(p)) if p.starts_with(&CFG.service.homedir)
			&& p.is_dir()
			&& p.join(&CFG.service.idxfile).is_file()
		=> {
		
			Ok(Response::new(Body::wrap_stream(fs::read(p.join(&CFG.service.idxfile)).into_stream())))
			
		}
		(_, Ok(p)) if p.starts_with(&CFG.service.homedir) && !p.exists()
		=> {
		
			let mut not_found = Response::new(Body::from("404".to_owned()));
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
			
		}
		_ => {
		
            let mut forbidden = Response::new(Body::from("403".to_owned()));
			*forbidden.status_mut() = StatusCode::FORBIDDEN;
			Ok(forbidden)
        }
	}
}



async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

	if !Path::new(&CFG.service.homedir).is_dir() {
		panic!("Home directory {} does not exist.", CFG.service.homedir);
	}	
	
    let addr = (CFG.service.host, CFG.service.port).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(req_handler)) });

    let server = Server::bind(&addr).serve(service);
    println!("Listening on http://{}", addr);
    
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
