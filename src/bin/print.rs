use nfse_bh_rust::curl::Request;
use nfse_bh_rust::curl::RequestMethod;
use nfse_bh_rust::lote_rps::LoteRps;
use nfse_bh_rust::nfse::Nfse;
use nfse_bh_rust::protocolo::Protocolo;
use nfse_bh_rust::utils::xml_events_to_xml_string;

fn main() -> Result<(), String> {
    let stdin = std::io::stdin();
    let args = std::env::args().collect::<Vec<String>>();

    let default_yaml_file_name = String::from("input.yml");
    let input_contents = args.get(1).unwrap_or(&default_yaml_file_name);

    let input_contents =
        std::fs::read_to_string(&input_contents).expect("unable to read input file");
    let input_contents = serde_yaml::from_str(&input_contents);

    let input_contents = match &input_contents {
        Ok(yaml) => match yaml {
            serde_yaml::Value::Mapping(it) => Ok(it),
            _ => Err("bad yaml input"),
        },
        Err(_) => Err("bad yaml input"),
    }?;

    let lote_rps = LoteRps::from_yaml(input_contents)
        .unwrap()
        .get_rpses()
        .map(|rps| (rps.uniquely_identify(), rps.nome_arquivo.clone()))
        .collect::<Vec<_>>();

    let production = match input_contents.get("producao") {
        Some(it) => match it {
            serde_yaml::Value::Bool(it) => Ok(it),
            _ => Err("bad yaml input: producao"),
        },
        None => Err("bad yaml input: producao"),
    }?;

    let certificado_pem_file = match input_contents.get("certificado_pem") {
        Some(it) => match it {
            serde_yaml::Value::String(it) => Ok(it),
            _ => Err("bad yaml input: certificado_pem"),
        },
        None => Err("bad yaml input: certificado_pem"),
    }?;

    print!(
        "Digite o número de protocolo no ambiente de {}: ",
        if *production { "PRODUÇÃO" } else { "teste" }
    );
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let mut protocolo = String::new();
    stdin.read_line(&mut protocolo).unwrap();
    protocolo.pop(); // remove \n

    let protocolo = Protocolo::from_yaml(&input_contents)
        .unwrap()
        .set_protocolo(protocolo);

    let content = xml_events_to_xml_string(&protocolo.protocolo_xml_events());

    let request_data = format!(
        r##"<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:ws="http://ws.bhiss.pbh.gov.br"><soapenv:Body><ws:ConsultarLoteRpsRequest><nfseCabecMsg><![CDATA[<?xml version="1.0" encoding="UTF-8"?><cabecalho xmlns="http://www.abrasf.org.br/nfse.xsd" versao="1.00"><versaoDados>1.00</versaoDados></cabecalho>]]></nfseCabecMsg><nfseDadosMsg><![CDATA[<?xml version="1.0" encoding="UTF-8"?>{content}]]></nfseDadosMsg></ws:ConsultarLoteRpsRequest></soapenv:Body></soapenv:Envelope>"##
    );

    let req = Request::new()
        .set_certificate_path(Some(certificado_pem_file.to_string()))
        .set_url(if *production {
            String::from("https://bhissdigitalws.pbh.gov.br/bhiss-ws/nfse")
        } else {
            String::from("https://bhisshomologaws.pbh.gov.br/bhiss-ws/nfse")
        })
        .set_header(
            String::from("Accept"),
            Some(String::from("application/xml")),
        )
        .set_header(String::from("Content-Type"), Some(String::from("text/xml")))
        .set_header(
            String::from("SOAPAction"),
            Some(String::from(
                "http://ws.bhiss.pbh.gov.br/ConsultarLoteRpsEnvio",
            )),
        )
        .set_method(RequestMethod::POST(request_data));

    let (status_code, data) = req.run().unwrap();

    let data = String::from_utf8(data).unwrap();

    if status_code != 200 {
        return Err(format!(
            "error in request (status {}), {}",
            status_code, data
        ));
    }

    let data = data
        .split_once("<outputXML>")
        .ok_or(String::from("expected outputXML tag"))?
        .1
        .split_once("</outputXML>")
        .ok_or(String::from("expected outputXML tag"))?
        .0;

    let data = data
        .replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">");

    let nfses = data
        .strip_prefix("<?xml version='1.0' encoding='UTF-8'?><ConsultarLoteRpsResposta xmlns=\"http://www.abrasf.org.br/nfse.xsd\"><ListaNfse>")
        .ok_or(String::from("bad xml prefix"))?
        .strip_suffix("</ListaNfse></ConsultarLoteRpsResposta>")
        .ok_or(String::from("bad xml suffix"))?
        .split("<CompNfse")
        .skip(1)
        .map(|nf| {
            let xml = format!("<?xml version='1.0' encoding='UTF-8'?><CompNfse{nf}");
            let nfse = Nfse::from_xml_string(&xml).unwrap();
            let ui = nfse.uniquely_identify();
            (ui, xml)
        })
        .collect::<Vec<(String, String)>>();

    let dir_name = format!("output-{}-{}", protocolo.protocolo, chrono::Utc::now().format("%Y-%m-%d-%H-%M"));

    std::fs::create_dir(&dir_name).unwrap();

    nfses.iter().for_each(|(ui, xml)| {
        let nome_arquivo = lote_rps.iter().find(|rps| *ui == rps.0).unwrap().1.clone();
        let mut xml_file =
            std::fs::File::create_new(&format!("{dir_name}/{nome_arquivo}_NFS.xml")).unwrap();
        std::io::Write::write_all(&mut xml_file, xml.as_bytes()).unwrap();

        let chave_acesso: Result<String, String> = 'a: {
            let chave = match xml
                .split_once("<OutrasInformacoes>Chave de acesso no Ambiente de Dados Nacional: ")
            {
                Some(s) => s.1,
                None => {
                    break 'a Err(String::from("can not find chave_acesso"));
                }
            };

            let chave = match chave.split_once(".</OutrasInformacoes>") {
                Some(s) => s.0,
                None => {
                    break 'a Err(String::from("can not find chave_acesso"));
                }
            };

            if chave.len() != 50 {
                panic!("bad size for chave_acesso");
            }

            Ok(chave.to_string())
        };

        match chave_acesso {
            Ok(chave) => {
                let req = Request::new()
                    .set_certificate_path(Some(certificado_pem_file.to_string()))
                    .set_url(format!(
                        "https://sefin.nfse.gov.br/sefinnacional/danfse/{chave}"
                    ))
                    .set_method(RequestMethod::GET);

                let (status_code, data) = req.run().unwrap();

                if status_code == 200 {
                    let mut pdf_file =
                        std::fs::File::create_new(&format!("{dir_name}/{nome_arquivo}_NFS.pdf")).unwrap();
                    std::io::Write::write_all(&mut pdf_file, &data).unwrap();
                } else {
                    println!("skipping pdf for {nome_arquivo}: {status_code}");
                }
            }
            Err(e) => {
                println!("skipping pdf for {nome_arquivo}: {e}");
            }
        }
    });

    Ok(())
}
