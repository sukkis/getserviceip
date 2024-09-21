use actix_web::{
    dev::Server,
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::IpAddr;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

#[derive(Deserialize, Serialize, Clone)]
pub struct IpInfo {
    pub hostname: String,
    pub ip_v6: String,
    pub ip_v4: String,
}

type AppState = Arc<Mutex<Vec<IpInfo>>>;

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/list_all")]
pub async fn list_all(data: Data<AppState>) -> impl Responder {
    let data_guard = data.lock().unwrap();
    HttpResponse::Ok().json(&*data_guard)
}

#[post("/ip")]
pub async fn ip(req_body: web::Json<IpInfo>, data: Data<AppState>) -> impl Responder {
    let validation_result = verify_info(&req_body);
    if validation_result != "valid" {
        return HttpResponse::BadRequest().json(json!({ "error": validation_result }));
    }

    let mut my_data = data.lock().unwrap(); // Add error handling later!
    let ip_info = req_body.into_inner();
    my_data.push(ip_info.clone());
    HttpResponse::Ok().json(ip_info)
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Host {
    pub hostname: String,
}

#[post("/host_details")]
pub async fn host_details(req_body: web::Json<Host>, data: Data<AppState>) -> impl Responder {
    let hostname = req_body.hostname.to_string();
    let mut info_vec: Vec<IpInfo> = Vec::new();
    let my_data = data.lock().unwrap(); // Add error handling later!
    for info in my_data.iter() {
        if info.hostname == hostname {
            info_vec.push(info.clone());
        }
    }
    HttpResponse::Ok().json(info_vec)
}

fn verify_info(req_body: &IpInfo) -> String {
    let ipv6_valid = match req_body.ip_v6.parse::<IpAddr>() {
        Ok(v6) => v6.is_ipv6(),
        Err(_) => false,
    };

    let ipv4_valid = match req_body.ip_v4.parse::<IpAddr>() {
        Ok(v4) => v4.is_ipv4(),
        Err(_) => false,
    };

    if ipv6_valid && ipv4_valid {
        "valid".to_string()
    } else if !ipv6_valid {
        "Invalid IP v6 address".to_string()
    } else {
        "Invalid IP v4 address".to_string()
    }
}

pub fn run(listener: TcpListener, state: AppState) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(health_check)
            .service(ip)
            .service(list_all)
            .service(host_details)
    })
    .listen(listener)?
    .run();

    Ok(server)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_info_valid_ips() {
        let ip_info = IpInfo {
            hostname: "example.com".to_string(),
            ip_v6: "2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string(),
            ip_v4: "192.168.0.1".to_string(),
        };
        let result = verify_info(&ip_info);
        assert_eq!(result, "valid");
    }

    #[test]
    fn test_verify_info_invalid_ipv6() {
        let ip_info = IpInfo {
            hostname: "example.com".to_string(),
            ip_v6: "invalid_ipv6".to_string(),
            ip_v4: "192.168.0.1".to_string(),
        };
        let result = verify_info(&ip_info);
        assert_eq!(result, "Invalid IP v6 address");
    }

    #[test]
    fn test_verify_info_invalid_ipv4() {
        let ip_info = IpInfo {
            hostname: "example.com".to_string(),
            ip_v6: "2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string(),
            ip_v4: "invalid_ipv4".to_string(),
        };
        let result = verify_info(&ip_info);
        assert_eq!(result, "Invalid IP v4 address");
    }

    #[test]
    fn test_verify_info_invalid_both_ips() {
        let ip_info = IpInfo {
            hostname: "example.com".to_string(),
            ip_v6: "invalid_ipv6".to_string(),
            ip_v4: "invalid_ipv4".to_string(),
        };
        let result = verify_info(&ip_info);
        assert_eq!(result, "Invalid IP v6 address");
    }
}
