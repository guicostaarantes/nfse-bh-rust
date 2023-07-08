use nfse_bh_rust::algorithms::CanonicalizationAlgorithm;
use nfse_bh_rust::algorithms::DigestAlgorithm;
use nfse_bh_rust::algorithms::SignatureAlgorithm;
use nfse_bh_rust::lote_rps::LoteRps;
use nfse_bh_rust::signature::XmlSignature;
use nfse_bh_rust::utils::recepcionar_lote_rps_request_wrapper;
use nfse_bh_rust::utils::trim_x509_certificate;
use nfse_bh_rust::utils::xml_events_to_xml_string;

fn main() -> Result<(), String> {
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

    let mut handle = curl::easy::Easy::new();
    let mut list = curl::easy::List::new();
    handle
        .url("https://bhisshomologa.pbh.gov.br/bhiss-ws/nfse")
        .map_err(|e| format!("error in curl: {}", e))?;
    handle
        .ssl_cert(certificado_pem_file)
        .map_err(|e| format!("error in curl: {}", e))?;
    handle
        .post(true)
        .map_err(|e| format!("error in curl: {}", e))?;
    handle
        .post_field_size(request_data.len() as u64)
        .map_err(|e| format!("error in curl: {}", e))?;
    list.append("Accept: application/xml")
        .map_err(|e| format!("error in curl: {}", e))?;
    list.append("Content-Type: text/xml")
        .map_err(|e| format!("error in curl: {}", e))?;
    list.append("SOAPAction: http://ws.bhiss.pbh.gov.br/RecepcionarLoteRps")
        .map_err(|e| format!("error in curl: {}", e))?;
    handle
        .http_headers(list)
        .map_err(|e| format!("error in curl: {}", e))?;
    handle
        .read_function(move |buf| {
            Ok(std::io::Read::read(&mut request_data.as_bytes(), buf).unwrap_or(0))
        })
        .map_err(|e| format!("error in curl: {}", e))?;
    handle
        .write_function(|data| {
            std::io::Write::write_all(&mut std::io::stdout(), data).unwrap();
            Ok(data.len())
        })
        .map_err(|e| format!("error in curl: {}", e))?;
    handle
        .perform()
        .map_err(|e| format!("error in curl: {}", e))?;

    Ok(())
}
