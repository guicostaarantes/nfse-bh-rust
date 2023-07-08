use std::io::Cursor;
use std::iter;

use quick_xml::{events::Event, Writer};
use rand::Rng;

pub fn trim_x509_certificate(cert: &str) -> String {
    cert.lines().fold(String::new(), |mut result, line| {
        if !line.starts_with("-----") {
            result.push_str(line);
        }
        result
    })
}

pub fn xml_events_to_xml_string(events: &[Event]) -> String {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    events.iter().for_each(|e| {
        writer.write_event(e).unwrap();
    });
    String::from_utf8(writer.into_inner().into_inner()).unwrap()
}

pub fn generate_random_rps() -> String {
    const CHARSET: &[u8] = b"0123456789";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(12).collect()
}

pub fn recepcionar_lote_rps_request_wrapper(content: &str) -> String {
    format!(
        r##"<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:ws="http://ws.bhiss.pbh.gov.br"><soapenv:Body><ws:RecepcionarLoteRpsRequest><nfseCabecMsg><![CDATA[<?xml version="1.0" encoding="UTF-8"?><cabecalho xmlns="http://www.abrasf.org.br/nfse.xsd" versao="1.00"><versaoDados>1.00</versaoDados></cabecalho>]]></nfseCabecMsg><nfseDadosMsg><![CDATA[<?xml version="1.0" encoding="UTF-8"?>{content}]]></nfseDadosMsg></ws:RecepcionarLoteRpsRequest></soapenv:Body></soapenv:Envelope>"##
    )
}
