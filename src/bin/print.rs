use nfse_bh_rust::curl::Request;
use nfse_bh_rust::curl::RequestMethod;
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

    println!("Code: {}\n", status_code);
    println!("{}", data);

    Ok(())
}
