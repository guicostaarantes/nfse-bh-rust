use nfse_bh_rust::algorithms::CanonicalizationAlgorithm;
use nfse_bh_rust::algorithms::DigestAlgorithm;
use nfse_bh_rust::algorithms::SignatureAlgorithm;
use nfse_bh_rust::curl::Request;
use nfse_bh_rust::curl::RequestMethod;
use nfse_bh_rust::lote_rps::LoteRps;
use nfse_bh_rust::signature::XmlSignature;
use nfse_bh_rust::utils::recepcionar_lote_rps_request_wrapper;
use nfse_bh_rust::utils::trim_x509_certificate;
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

    let private_key = match input_contents.get("certificado_key") {
        Some(it) => match it {
            serde_yaml::Value::String(it) => {
                let key_contents = std::fs::read_to_string(it)
                    .map_err(|_| "could not read file at certificado_key")?;
                <rsa::RsaPrivateKey as rsa::pkcs8::DecodePrivateKey>::from_pkcs8_pem(&key_contents)
                    .map_err(|_| "file at certificado_key is not valid private key")
            }
            _ => Err("bad yaml input: certificado_key"),
        },
        None => Err("bad yaml input: certificado_key"),
    }?;

    let certificate = match input_contents.get("certificado_cer") {
        Some(it) => match it {
            serde_yaml::Value::String(it) => {
                let cer_contents = std::fs::read_to_string(it)
                    .map_err(|_| "could not read file at certificado_cer")?;
                Ok(trim_x509_certificate(&cer_contents))
            }
            _ => Err("bad yaml input: certificado_cer"),
        },
        None => Err("bad yaml input: certificado_cer"),
    }?;

    let certificado_pem_file = match input_contents.get("certificado_pem") {
        Some(it) => match it {
            serde_yaml::Value::String(it) => Ok(it),
            _ => Err("bad yaml input: certificado_pem"),
        },
        None => Err("bad yaml input: certificado_pem"),
    }?;

    let signature = XmlSignature::new(
        CanonicalizationAlgorithm::ExclusiveXMLCanonicalization,
        SignatureAlgorithm::RsaSha1(private_key),
        DigestAlgorithm::Sha1,
        certificate,
    );

    let mut lote_rps = LoteRps::from_yaml(input_contents).unwrap();

    lote_rps
        .get_rpses()
        .for_each(|rps| rps.sign(signature.clone()));

    lote_rps.sign(signature);

    let request_data = recepcionar_lote_rps_request_wrapper(&xml_events_to_xml_string(
        &lote_rps.enviar_lote_rps_envio_events(),
    ));

    print!(
        "Digite SIM para confirmar a emissão de {} notas fiscais em ambiente de {}: ",
        lote_rps.get_rpses().len(),
        if *production { "PRODUÇÃO" } else { "teste" }
    );
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let mut confirm = String::new();
    stdin.read_line(&mut confirm).unwrap();
    confirm.pop(); // remove \n

    if &confirm != "SIM" {
        return Err(String::from("confirmation failed"));
    }

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
                "http://ws.bhiss.pbh.gov.br/RecepcionarLoteRps",
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

    let protocolo = data
        .split_once("Protocolo&gt;")
        .unwrap()
        .1
        .split_once("&lt;/Protocolo")
        .unwrap()
        .0;

    println!("Enviado com sucesso! Protocolo: {}", protocolo);

    Ok(())
}
