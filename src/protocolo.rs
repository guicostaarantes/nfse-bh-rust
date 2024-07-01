use quick_xml::events::BytesEnd;
use quick_xml::events::BytesStart;
use quick_xml::events::BytesText;
use quick_xml::events::Event;

pub struct Protocolo {
    protocolo: String,
    cnpj: String,
    inscricao_municipal: String,
}

impl Protocolo {
    fn new(protocolo: String, cnpj: String, inscricao_municipal: String) -> Self {
        Self {
            protocolo,
            cnpj,
            inscricao_municipal,
        }
    }
}

impl Protocolo {
    pub fn from_yaml(yaml: &serde_yaml::Mapping) -> Result<Self, String> {
        let cnpj = match yaml.get("cnpj") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                serde_yaml::Value::Number(it) => Ok(format!("{}", it)),
                _ => Err("bad yaml input: cnpj"),
            },
            None => Err("bad yaml input: cnpj"),
        }?;

        let inscricao_municipal = match yaml.get("inscricao_municipal") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                serde_yaml::Value::Number(it) => Ok(format!("{}", it)),
                _ => Err("bad yaml input: inscricao_municipal"),
            },
            None => Err("bad yaml input: inscricao_municipal"),
        }?;

        let protocolo = String::new();

        Ok(Self::new(protocolo, cnpj, inscricao_municipal))
    }
}

impl Protocolo {
    pub fn set_protocolo(mut self, protocolo: String) -> Self {
        self.protocolo = protocolo;
        self
    }
}

impl Protocolo {
    pub fn protocolo_xml_events(&self) -> Vec<Event> {
        let mut events = Vec::new();

        let mut elem = BytesStart::new("ConsultarLoteRpsEnvio");
        elem.push_attribute(("xmlns", "http://www.abrasf.org.br/nfse.xsd"));
        elem.push_attribute(("versao", "1.00"));
        events.push(Event::Start(elem));

        let elem = BytesStart::new("Prestador");
        events.push(Event::Start(elem));

        let elem = BytesStart::new("Cnpj");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.cnpj.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("Cnpj");
        events.push(Event::End(elem));

        let elem = BytesStart::new("InscricaoMunicipal");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.inscricao_municipal.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("InscricaoMunicipal");
        events.push(Event::End(elem));

        let elem = BytesEnd::new("Prestador");
        events.push(Event::End(elem));

        let elem = BytesStart::new("Protocolo");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.protocolo.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("Protocolo");
        events.push(Event::End(elem));

        let elem = BytesEnd::new("ConsultarLoteRpsEnvio");
        events.push(Event::End(elem));

        events
    }
}

#[cfg(test)]
mod tests {
    use crate::protocolo::Protocolo;
    use crate::utils;

    #[test]
    fn should_create_protocolo() {
        let yaml: serde_yaml::Mapping = serde_yaml::from_str(
            "
cnpj: cnpj_prestador
inscricao_municipal: inscricao_municipal_prestador
",
        )
        .unwrap();

        let protocolo = Protocolo::from_yaml(&yaml)
            .unwrap()
            .set_protocolo(String::from("protocolo_123"));

        assert_eq!(
            utils::xml_events_to_xml_string(&protocolo.protocolo_xml_events()),
            String::from(
                r##"<ConsultarLoteRpsEnvio xmlns="http://www.abrasf.org.br/nfse.xsd" versao="1.00"><Prestador><Cnpj>cnpj_prestador</Cnpj><InscricaoMunicipal>inscricao_municipal_prestador</InscricaoMunicipal></Prestador><Protocolo>protocolo_123</Protocolo></ConsultarLoteRpsEnvio>"##
            )
        );
    }
}
