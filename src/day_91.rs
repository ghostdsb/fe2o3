pub mod attack_on_titan {
  use std::error::Error;
  use std::thread;
  use std::time::Duration;

  pub fn search() -> Result<(), Box<dyn Error>> {
    // let resp = reqwest::blocking::get("https://aotonline.org/season-4-episode-8-eng-sub.html")?;
    // println!("reponse: {:?}", resp);
    scout();
    Ok(())
  }

  fn scout() -> Result<(), Box<dyn Error>> {
    for i in 1..20 {
      thread::spawn(move || {
        println!("hi number {} from the spawned thread!", i);
        let resp = reqwest::blocking::get(format!(
          "https://aotonline.org/season-4-episode-{}-eng-sub.html",
          i
        ));
        match resp {
          Ok(resp) => println!("reponse: {:?}", resp),
          Err(err) => println!("error: {:?}", err),
        }
      })
      .join()
      .unwrap();
    }

    Ok(())
  }
}

// reponse:
// Ok(
//     Response
//     {
//         url: Url
//         {
//             scheme: "https",
//             cannot_be_a_base: false,
//             username: "",
//             password: None,
//             host: Some(Domain("aotonline.org")),
//             port: None,
//             path: "/season-4-episode-80-eng-sub.html",
//             query: None,
//             fragment: None
//         },
//         status: 404,
//         headers: {
//             "date": "Sat, 05 Feb 2022 05:40:52 GMT",
//             "content-type": "text/html; charset=UTF-8",
//             "transfer-encoding": "chunked",
//             "connection": "keep-alive",
//             "x-powered-by": "PHP/7.1.33",
//             "x-frame-options": "SAMEORIGIN",
//             "cf-cache-status": "DYNAMIC",
//             "expect-ct": "max-age=604800, report-uri=\"https://report-uri.cloudflare.com/cdn-cgi/beacon/expect-ct\"",
//             "report-to": "{\"endpoints\":[{\"url\":\"https:\/\/a.nel.cloudflare.com\/report\/v3?s=T6ME9sZ6Ujq0HVEpklG4YYDIj83YB%2BYLRbJreDIQWXL2rJKxHa4Dk7JsaI47CoBe5lumkErp%2Fc4uk6CLOoyktOqyrLg1ezDRFIj0paXqylo5eg09UOOnF8vrFQHwMH%2Fa\"}],\"group\":\"cf-nel\",\"max_age\":604800}",
//             "nel": "{\"success_fraction\":0,\"report_to\":\"cf-nel\",\"max_age\":604800}",
//             "server": "cloudflare",
//             "cf-ray": "6d89ce76ae617743-LHR",
//             "alt-svc": "h3=\":443\"; ma=86400, h3-29=\":443\"; ma=86400"
//         }
//     }
// )

// reponse:
// Response
// {
//     url: Url
//     {
//         scheme: "https",
//         cannot_be_a_base: false,
//         username: "",
//         password: None,
//         host: Some(Domain("aotonline.org")),
//         port: None,
//         path: "/season-4-episode-8-eng-sub.html",
//         query: None,
//         fragment: None
//     },
//     status: 200,
//     headers: {
//         "date": "Sat, 05 Feb 2022 06:00:44 GMT",
//         "content-type": "text/html; charset=UTF-8",
//         "transfer-encoding": "chunked",
//         "connection": "keep-alive",
//         "x-powered-by": "PHP/7.1.33",
//         "x-frame-options": "SAMEORIGIN",
//         "cf-cache-status": "DYNAMIC",
//         "expect-ct": "max-age=604800, report-uri=\"https://report-uri.cloudflare.com/cdn-cgi/beacon/expect-ct\"",
//         "report-to": "{\"endpoints\":[{\"url\":\"https:\/\/a.nel.cloudflare.com\/report\/v3?s=iVF7Zhfueb1w6KS2NAes%2FbuhREKBS45LpgQdiIjHksquvKtPAw%2ButPd7uankWSXCdW1QzN8JaZRonIckzF87I3UDDELWnFsvgi2TKxf4HYyM7mXFaQ3jCGPY7Hpdc6gx\"}],\"group\":\"cf-nel\",\"max_age\":604800}",
//         "nel": "{\"success_fraction\":0,\"report_to\":\"cf-nel\",\"max_age\":604800}",
//         "server": "cloudflare",
//         "cf-ray": "6d89eb8b39e67511-LHR",
//         "alt-svc": "h3=\":443\"; ma=86400, h3-29=\":443\"; ma=86400"
//     }
// }
