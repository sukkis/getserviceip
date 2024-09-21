# getserviceip
getserviceip is a Rust-based web service that provides an endpoint to validate and return IP information. It uses the Actix-web framework to handle HTTP requests.

As an example, you might have a Raspberry Pi in local network that gets IP assigned by DHCP, and you reach the device with ssh.
If the address for you device changes, you might not be able to connect it. Instead you could set up a cronjob in the Raspberry Pi to frequently send the IP information to getserviceip. You will always know where your devices can be reached.

## Features
- *Health Check Endpoint:* A simple endpoint to check if the service is running.
- *IP Information Endpoint:* Validates and returns IP information provided in the request body.
- *List All Endpoint:* Returns all the IP information that the service has gathered.

## Endpoints

### Health Check
- *URL:* `/health_check`
- *Method:* `GET`
- *Response:* Returns a plain text "OK" if the service is running.

#### Example
 
``` sh
curl http://localhost:8087/health_check
```

Response:

```
OK
```

### List all
- *URL:* /list_all
- *Method:* GET
- *Response:* Returns as JSON all the IP information that the service has gathered.

#### Example

``` sh
curl http://localhost:8087/list_all
```

Response:

``` json
[
    {
        "hostname": "example1.com",
        "ip_v6": "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
        "ip_v4": "192.168.0.1"
    },
    {
        "hostname": "example2.com",
        "ip_v6": "2001:0db8:85a3:0000:0000:8a2e:0370:7335",
        "ip_v4": "192.168.0.2"
    }
]
```

### Receive IP info 
- *URL:* /ip
- *Method:* POST
- *Request Body:* JSON object containing hostname, ip_v6, and ip_v4.
- *Response:* Returns the same JSON object if the information is valid, otherwise returns an error message.

#### Example

``` sh
curl -X POST http://localhost:8087/ip -H "Content-Type: application/json" -d '{
  "hostname": "example.com",
  "ip_v6": "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
  "ip_v4": "192.168.1.1"
}'
```
Response:

``` json
{
  "hostname": "example.com",
  "ip_v6": "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
  "ip_v4": "192.168.1.1"
}
```

## Running the Service

To run the service, use the following command:

``` sh
cargo run
```

The service will start on http://localhost:8087.

## Dependencies
- [Actix-web](https://actix.rs/)
- [Serde](https://serde.rs/)
- [Serde JSON](https://docs.serde.rs/serde_json/)

## License
This project is licensed under the GPL v3 license. See the [LICENSE](LICENSE) file for details.
