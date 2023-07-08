use quick_xml::events::BytesEnd;
use quick_xml::events::BytesStart;
use quick_xml::events::BytesText;
use quick_xml::events::Event;

use crate::rps::Rps;
use crate::signature::XmlSignature;
use crate::utils;

pub struct LoteRps {
    rpses: Vec<Rps>,
    cnpj: String,
    inscricao_municipal: String,
    signature: Option<XmlSignature>,
}

impl LoteRps {
    fn new(rpses: Vec<Rps>, cnpj: String, inscricao_municipal: String) -> Self {
        Self {
            rpses,
            cnpj,
            inscricao_municipal,
            signature: None,
        }
    }
}

impl LoteRps {
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

        let codigo_municipio = match yaml.get("codigo_municipio") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                serde_yaml::Value::Number(it) => Ok(format!("{}", it)),
                _ => Err("bad yaml input: codigo_municipio"),
            },
            None => Err("bad yaml input: codigo_municipio"),
        }?;

        let rpses = match yaml.get("notas_fiscais") {
            Some(it) => match it {
                serde_yaml::Value::Sequence(it) => Ok(it),
                _ => Err("bad yaml input: notas_fiscais"),
            },
            None => Err("bad yaml input: notas_fiscais"),
        }?;

        let rpses = rpses
            .iter()
            .enumerate()
            .map(|(i, rps)| {
                match Rps::from_yaml(
                    rps.clone(),
                    cnpj.clone(),
                    inscricao_municipal.clone(),
                    codigo_municipio.clone(),
                ) {
                    Ok(rps) => Ok(rps),
                    Err(e) => Err(format!("error in notas_fiscais.{i}: {e}")),
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(rpses, cnpj, inscricao_municipal))
    }
}

impl LoteRps {
    pub fn get_rpses(&mut self) -> std::slice::IterMut<Rps> {
        self.rpses.iter_mut()
    }
}

impl LoteRps {
    pub fn sign(&mut self, mut signature: XmlSignature) {
        signature.load(
            String::from("#lote"),
            utils::xml_events_to_xml_string(&self.lote_rps_xml_events()),
        );

        signature.sign();

        self.signature = Some(signature);
    }
}

impl LoteRps {
    fn lote_rps_xml_events(&self) -> Vec<Event> {
        let mut events = Vec::new();

        let mut elem = BytesStart::new("LoteRps");
        elem.push_attribute(("xmlns", "http://www.abrasf.org.br/nfse.xsd"));
        elem.push_attribute(("Id", "lote"));
        elem.push_attribute(("versao", "1.00"));
        events.push(Event::Start(elem));

        let elem = BytesStart::new("NumeroLote");
        events.push(Event::Start(elem));

        let elem = BytesText::new("1");
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("NumeroLote");
        events.push(Event::End(elem));

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

        let elem = BytesStart::new("QuantidadeRps");
        events.push(Event::Start(elem));

        let qtd_rps = format!("{}", self.rpses.len());
        let elem = BytesText::new(qtd_rps.as_str());
        events.push(Event::Text(elem).into_owned());

        let elem = BytesEnd::new("QuantidadeRps");
        events.push(Event::End(elem));

        let elem = BytesStart::new("ListaRps");
        events.push(Event::Start(elem));

        self.rpses.iter().for_each(|rps| {
            rps.rps_xml_events(false)
                .iter()
                .for_each(|e| events.push(e.to_owned()));
        });

        let elem = BytesEnd::new("ListaRps");
        events.push(Event::End(elem));

        let elem = BytesEnd::new("LoteRps");
        events.push(Event::End(elem));

        events
    }
}

impl LoteRps {
    pub fn enviar_lote_rps_envio_events(&self) -> Vec<Event> {
        let mut events = Vec::new();

        let mut elem = BytesStart::new("EnviarLoteRpsEnvio");
        elem.push_attribute(("xmlns", "http://www.abrasf.org.br/nfse.xsd"));
        elem.push_attribute(("versao", "1.00"));
        events.push(Event::Start(elem));

        self.lote_rps_xml_events()
            .iter()
            .for_each(|e| events.push(e.to_owned()));

        if let Some(signature) = &self.signature {
            signature
                .signature_xml_events()
                .iter()
                .for_each(|e| events.push(e.to_owned()));
        };

        let elem = BytesEnd::new("EnviarLoteRpsEnvio");
        events.push(Event::End(elem));

        events
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::CanonicalizationAlgorithm;
    use crate::algorithms::DigestAlgorithm;
    use crate::algorithms::SignatureAlgorithm;
    use crate::utils;

    #[test]
    fn should_create_signed_lote_rps() {
        let signature = super::XmlSignature::new(
            CanonicalizationAlgorithm::NoOp,
            SignatureAlgorithm::Echo(String::from("the_signature")),
            DigestAlgorithm::Echo(String::from("the_digest")),
            String::from("the_certificate"),
        );

        let yaml: serde_yaml::Mapping = serde_yaml::from_str(
            "
cnpj: cnpj_prestador
inscricao_municipal: inscricao_municipal_prestador
codigo_municipio: codigo_municipio_prestador
notas_fiscais:
  - id: 1234
    competencia: data_emissao
    natureza_operacao: natureza_operacao
    regime_especial_tributacao: regime_especial_tributacao
    optante_simples_nacional: optante_simples_nacional
    incentivador_cultural: incentivador_cultural
    item_lista_servico: item_lista_servico
    codigo_tributacao_municipio: codigo_tributacao_municipio
    discriminacao: discriminacao
    valor_servicos: 1000.00
    aliquota_iss: 0.02
    cnpj: cnpj_tomador
    inscricao_municipal: inscricao_municipal_tomador
    razao_social: razao_social_tomador
    logradouro: logradouro_tomador
    numero: numero_tomador
    complemento: complemento_tomador
    bairro: bairro_tomador
    codigo_municipio: codigo_municipio_tomador
    uf: uf_tomador
    cep: cep_tomador
  - id: 5678
    competencia: data_emissao_2
    natureza_operacao: natureza_operacao_2
    regime_especial_tributacao: regime_especial_tributacao_2
    optante_simples_nacional: optante_simples_nacional_2
    incentivador_cultural: incentivador_cultural_2
    item_lista_servico: item_lista_servico_2
    codigo_tributacao_municipio: codigo_tributacao_municipio_2
    discriminacao: discriminacao_2
    valor_servicos: 800.00
    aliquota_iss: 0.03
    cnpj: cnpj_tomador_2
    inscricao_municipal: inscricao_municipal_tomador_2
    razao_social: razao_social_tomador_2
    logradouro: logradouro_tomador_2
    numero: numero_tomador_2
    complemento: complemento_tomador_2
    bairro: bairro_tomador_2
    codigo_municipio: codigo_municipio_tomador_2
    uf: uf_tomador_2
    cep: cep_tomador_2
",
        )
        .unwrap();

        let mut lote_rps = super::LoteRps::from_yaml(&yaml).unwrap();

        lote_rps.sign(signature);

        assert_eq!(
            utils::xml_events_to_xml_string(&lote_rps.enviar_lote_rps_envio_events()),
            String::from(
                r##"<EnviarLoteRpsEnvio xmlns="http://www.abrasf.org.br/nfse.xsd" versao="1.00"><LoteRps xmlns="http://www.abrasf.org.br/nfse.xsd" Id="lote" versao="1.00"><NumeroLote>1</NumeroLote><Cnpj>cnpj_prestador</Cnpj><InscricaoMunicipal>inscricao_municipal_prestador</InscricaoMunicipal><QuantidadeRps>2</QuantidadeRps><ListaRps><Rps versao="1.00"><InfRps Id="1234" versao="1.00"><IdentificacaoRps><Numero>1234</Numero><Serie>1</Serie><Tipo>1</Tipo></IdentificacaoRps><DataEmissao>data_emissao</DataEmissao><NaturezaOperacao>natureza_operacao</NaturezaOperacao><RegimeEspecialTributacao>regime_especial_tributacao</RegimeEspecialTributacao><OptanteSimplesNacional>optante_simples_nacional</OptanteSimplesNacional><IncentivadorCultural>incentivador_cultural</IncentivadorCultural><Status>1</Status><Servico><Valores><ValorServicos>1000.00</ValorServicos><IssRetido>1</IssRetido><ValorIss>20.00</ValorIss><ValorIssRetido>20.00</ValorIssRetido><BaseCalculo>1000.00</BaseCalculo><Aliquota>0.02</Aliquota><ValorLiquidoNfse>980.00</ValorLiquidoNfse></Valores><ItemListaServico>item_lista_servico</ItemListaServico><CodigoTributacaoMunicipio>codigo_tributacao_municipio</CodigoTributacaoMunicipio><Discriminacao>discriminacao</Discriminacao><CodigoMunicipio>codigo_municipio_prestador</CodigoMunicipio></Servico><Prestador><Cnpj>cnpj_prestador</Cnpj><InscricaoMunicipal>inscricao_municipal_prestador</InscricaoMunicipal></Prestador><Tomador><IdentificacaoTomador><CpfCnpj><Cnpj>cnpj_tomador</Cnpj></CpfCnpj><InscricaoMunicipal>inscricao_municipal_tomador</InscricaoMunicipal></IdentificacaoTomador><RazaoSocial>razao_social_tomador</RazaoSocial><Endereco><Endereco>logradouro_tomador</Endereco><Numero>numero_tomador</Numero><Complemento>complemento_tomador</Complemento><Bairro>bairro_tomador</Bairro><CodigoMunicipio>codigo_municipio_tomador</CodigoMunicipio><Uf>uf_tomador</Uf><Cep>cep_tomador</Cep></Endereco></Tomador></InfRps></Rps><Rps versao="1.00"><InfRps Id="5678" versao="1.00"><IdentificacaoRps><Numero>5678</Numero><Serie>1</Serie><Tipo>1</Tipo></IdentificacaoRps><DataEmissao>data_emissao_2</DataEmissao><NaturezaOperacao>natureza_operacao_2</NaturezaOperacao><RegimeEspecialTributacao>regime_especial_tributacao_2</RegimeEspecialTributacao><OptanteSimplesNacional>optante_simples_nacional_2</OptanteSimplesNacional><IncentivadorCultural>incentivador_cultural_2</IncentivadorCultural><Status>1</Status><Servico><Valores><ValorServicos>800.00</ValorServicos><IssRetido>1</IssRetido><ValorIss>24.00</ValorIss><ValorIssRetido>24.00</ValorIssRetido><BaseCalculo>800.00</BaseCalculo><Aliquota>0.03</Aliquota><ValorLiquidoNfse>776.00</ValorLiquidoNfse></Valores><ItemListaServico>item_lista_servico_2</ItemListaServico><CodigoTributacaoMunicipio>codigo_tributacao_municipio_2</CodigoTributacaoMunicipio><Discriminacao>discriminacao_2</Discriminacao><CodigoMunicipio>codigo_municipio_prestador</CodigoMunicipio></Servico><Prestador><Cnpj>cnpj_prestador</Cnpj><InscricaoMunicipal>inscricao_municipal_prestador</InscricaoMunicipal></Prestador><Tomador><IdentificacaoTomador><CpfCnpj><Cnpj>cnpj_tomador_2</Cnpj></CpfCnpj><InscricaoMunicipal>inscricao_municipal_tomador_2</InscricaoMunicipal></IdentificacaoTomador><RazaoSocial>razao_social_tomador_2</RazaoSocial><Endereco><Endereco>logradouro_tomador_2</Endereco><Numero>numero_tomador_2</Numero><Complemento>complemento_tomador_2</Complemento><Bairro>bairro_tomador_2</Bairro><CodigoMunicipio>codigo_municipio_tomador_2</CodigoMunicipio><Uf>uf_tomador_2</Uf><Cep>cep_tomador_2</Cep></Endereco></Tomador></InfRps></Rps></ListaRps></LoteRps><Signature xmlns="http://www.w3.org/2000/09/xmldsig#"><SignedInfo><CanonicalizationMethod Algorithm="noop-c14n"></CanonicalizationMethod><SignatureMethod Algorithm="echo-signature"></SignatureMethod><Reference URI="#lote"><Transforms><Transform Algorithm="noop-c14n"></Transform></Transforms><DigestMethod Algorithm="echo-digest"></DigestMethod><DigestValue>the_digest</DigestValue></Reference></SignedInfo><SignatureValue>the_signature</SignatureValue><KeyInfo><X509Data><X509Certificate>the_certificate</X509Certificate></X509Data></KeyInfo></Signature></EnviarLoteRpsEnvio>"##
            )
        );
    }
}
