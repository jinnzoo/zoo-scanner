use zoo_scanner::utils::init;
use zoo_scanner::gen;
use zoo_scanner::objects;

fn main() {
    // parse args
    let (urls, enable_xss, enable_directoryt, enable_osci) = 
        init::parse_args();

    //read config
    let config_path = "./conf.toml";
    let checked_paths = init::read_config(config_path);

    //run xss, directory traversal, os command injection scan
    type ScanKind = (bool, fn(&[String]) -> Vec<objects::payload::Payload>);
    let gens: [ScanKind;3] = 
        [(enable_xss, gen::xss::generate_xssp), 
         (enable_directoryt, gen::directoryt::generate_directorytp),
         (enable_osci, gen::osci::generate_oscip)];
    let mut easy = init::init_easy();
    for j in gens.iter() {
        if !j.0 {
            continue;
        }
        let payloads = j.1(&urls);
        for payload in payloads {
            println!("{}", payload.get_url());
            easy.url(payload.get_url()).unwrap();
            easy.referer(payload.get_referer()).unwrap();
            easy.useragent(payload.get_useragent()).unwrap();
            easy.cookie(payload.get_cookie()).unwrap();
            easy.perform().unwrap();
        }
    }
    drop(easy);

    //run path check
    let mut easy = init::init_easy();
    let checked_urls =
        gen::pathcheck::generate_pathcheckp(&urls, &checked_paths);
    println!("{:?}", checked_urls);
    for url in checked_urls {
        easy.url(&url).unwrap();
        easy.perform().unwrap();
    }
}
