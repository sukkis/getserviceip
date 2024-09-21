use actix_web::{dev::Server, get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::IpAddr;
use std::net::TcpListener;

#[derive(Deserialize, Serialize)]
struct IpInfo {
    hostname: String,
    ip_v6: String,
    ip_v4: String,
}

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[post("/ip")]
pub async fn ip(req_body: web::Json<IpInfo>) -> impl Responder {
    let validation_result = verify_info(&req_body);
    if validation_result != "valid" {
        return HttpResponse::BadRequest().json(json!({ "error": validation_result }));
    }

    HttpResponse::Ok().json(req_body.into_inner())
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

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(ip))
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
