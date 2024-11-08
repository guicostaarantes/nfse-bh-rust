use quick_xml::events::BytesEnd;
use quick_xml::events::BytesStart;
use quick_xml::events::BytesText;
use quick_xml::events::Event;

use crate::signature::XmlSignature;
use crate::utils;

pub struct Rps {
    id: String,
    pub nome_arquivo: String,
    data_emissao: String,
    natureza_operacao: String,
    regime_especial_tributacao: String,
    optante_simples_nacional: String,
    incentivador_cultural: String,
    item_lista_servico: String,
    codigo_tributacao_municipio: String,
    discriminacao: String,
    codigo_municipio: String,
    valor_servicos: String,
    aliquota_iss: Option<String>,
    valor_iss: Option<String>,
    valor_liquido: String,
    cnpj_prestador: String,
    inscricao_municipal_prestador: String,
    cnpj_tomador: Option<String>,
    inscricao_municipal_tomador: Option<String>,
    razao_social_tomador: String,
    logradouro_tomador: String,
    numero_tomador: String,
    complemento_tomador: Option<String>,
    bairro_tomador: String,
    codigo_municipio_tomador: String,
    uf_tomador: String,
    cep_tomador: Option<String>,
    signature: Option<XmlSignature>,
}

impl Rps {
    pub fn new(
        id: String,
        nome_arquivo: String,
        data_emissao: String,
        natureza_operacao: String,
        regime_especial_tributacao: String,
        optante_simples_nacional: String,
        incentivador_cultural: String,
        item_lista_servico: String,
        codigo_tributacao_municipio: String,
        discriminacao: String,
        codigo_municipio: String,
        valor_servicos: String,
        aliquota_iss: Option<String>,
        cnpj_prestador: String,
        inscricao_municipal_prestador: String,
        cnpj_tomador: Option<String>,
        inscricao_municipal_tomador: Option<String>,
        razao_social_tomador: String,
        logradouro_tomador: String,
        numero_tomador: String,
        complemento_tomador: Option<String>,
        bairro_tomador: String,
        codigo_municipio_tomador: String,
        uf_tomador: String,
        cep_tomador: Option<String>,
    ) -> Self {
        let (valor_servicos, valor_iss, valor_liquido) = match &aliquota_iss {
            Some(aliquota) => {
                let valor_total = valor_servicos.parse::<f64>().unwrap();
                let aliquota = aliquota.parse::<f64>().unwrap();
                let valor_iss = (100.0 * valor_total * aliquota).round() / 100.0;
                let valor_liquido = valor_total - valor_iss;
                let valor_total = format!("{:.2}", valor_total);
                let valor_iss = format!("{:.2}", valor_iss);
                let valor_liquido = format!("{:.2}", valor_liquido);
                (valor_total, Some(valor_iss), valor_liquido)
            }
            None => {
                let valor_total = valor_servicos.parse::<f64>().unwrap();
                let valor_liquido = valor_total - 0.0;
                let valor_total = format!("{:.2}", valor_total);
                let valor_liquido = format!("{:.2}", valor_liquido);
                (valor_total, None, valor_liquido)
            }
        };

        Self {
            id,
            nome_arquivo,
            data_emissao,
            natureza_operacao,
            regime_especial_tributacao,
            optante_simples_nacional,
            incentivador_cultural,
            item_lista_servico,
            codigo_tributacao_municipio,
            discriminacao,
            codigo_municipio,
            valor_servicos,
            aliquota_iss,
            valor_iss,
            valor_liquido,
            cnpj_prestador,
            inscricao_municipal_prestador,
            cnpj_tomador,
            inscricao_municipal_tomador,
            razao_social_tomador,
            logradouro_tomador,
            numero_tomador,
            complemento_tomador,
            bairro_tomador,
            codigo_municipio_tomador,
            uf_tomador,
            cep_tomador,
            signature: None,
        }
    }
}

impl Rps {
    pub fn from_yaml(
        yaml: serde_yaml::Value,
        cnpj: String,
        inscricao_municipal: String,
        codigo_municipio: String,
    ) -> Result<Self, String> {
        let yaml = match yaml {
            serde_yaml::Value::Mapping(it) => Ok(it),
            _ => Err("bad yaml input"),
        }?;

        let id = match yaml.get("id") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                serde_yaml::Value::Number(it) => Ok(format!("{}", it)),
                _ => Err("bad yaml input: id"),
            },
            None => Ok(utils::generate_random_rps()),
        }?;

        let nome_arquivo = match yaml.get("nome_arquivo") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                _ => Err("bad yaml input: nome_arquivo"),
            },
            None => Err("bad yaml input: nome_arquivo"),
        }?;

        let data_emissao = match yaml.get("competencia") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => match it {
                    // TODO
                    // "start_of_current_month" =>
                    // "start_of_last_month" =>
                    _ => Ok(it.clone()),
                },
                _ => Err("bad yaml input: competencia"),
            },
            None => Err("bad yaml input: competencia"),
        }?;

        let natureza_operacao = match yaml.get("natureza_operacao") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                serde_yaml::Value::Number(it) => Ok(format!("{}", it)),
                _ => Err("bad yaml input: natureza_operacao"),
            },
            None => Err("bad yaml input: natureza_operacao"),
        }?;

        let regime_especial_tributacao = match yaml.get("regime_especial_tributacao") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                serde_yaml::Value::Number(it) => Ok(format!("{}", it)),
                _ => Err("bad yaml input: regime_especial_tributacao"),
            },
            None => Err("bad yaml input: regime_especial_tributacao"),
        }?;

        let optante_simples_nacional = match yaml.get("optante_simples_nacional") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                serde_yaml::Value::Number(it) => Ok(format!("{}", it)),
                _ => Err("bad yaml input: optante_simples_nacional"),
            },
            None => Err("bad yaml input: optante_simples_nacional"),
        }?;

        let incentivador_cultural = match yaml.get("incentivador_cultural") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                serde_yaml::Value::Number(it) => Ok(format!("{}", it)),
                _ => Err("bad yaml input: incentivador_cultural"),
            },
            None => Err("bad yaml input: incentivador_cultural"),
        }?;

        let item_lista_servico = match yaml.get("item_lista_servico") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                serde_yaml::Value::Number(it) => Ok(format!("{}", it)),
                _ => Err("bad yaml input: item_lista_servico"),
            },
            None => Err("bad yaml input: item_lista_servico"),
        }?;

        let codigo_tributacao_municipio = match yaml.get("codigo_tributacao_municipio") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                serde_yaml::Value::Number(it) => Ok(format!("{}", it)),
                _ => Err("bad yaml input: codigo_tributacao_municipio"),
            },
            None => Err("bad yaml input: codigo_tributacao_municipio"),
        }?;

        let discriminacao = match yaml.get("discriminacao") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                serde_yaml::Value::Number(it) => Ok(format!("{}", it)),
                _ => Err("bad yaml input: discriminacao"),
            },
            None => Err("bad yaml input: discriminacao"),
        }?;

        let valor_servicos = match yaml.get("valor_servicos") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                serde_yaml::Value::Number(it) => Ok(format!("{}", it)),
                _ => Err("bad yaml input: valor_servicos"),
            },
            None => Err("bad yaml input: valor_servicos"),
        }?;

        let aliquota_iss = match yaml.get("aliquota_iss") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(Some(it.clone())),
                serde_yaml::Value::Number(it) => Ok(Some(format!("{}", it))),
                _ => Err("bad yaml input: aliquota_iss"),
            },
            None => Ok(None),
        }?;

        let cnpj_tomador = match yaml.get("cnpj") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(Some(it.clone())),
                serde_yaml::Value::Number(it) => Ok(Some(format!("{}", it))),
                _ => Err("bad yaml input: cnpj"),
            },
            None => Ok(None),
        }?;

        let inscricao_municipal_tomador = match yaml.get("inscricao_municipal") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(Some(it.clone())),
                serde_yaml::Value::Number(it) => Ok(Some(format!("{}", it))),
                _ => Err("bad yaml input: inscricao_municipal"),
            },
            None => Ok(None),
        }?;

        let razao_social_tomador = match yaml.get("razao_social") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                serde_yaml::Value::Number(it) => Ok(format!("{}", it)),
                _ => Err("bad yaml input: razao_social"),
            },
            None => Err("bad yaml input: razao_social"),
        }?;

        let logradouro_tomador = match yaml.get("logradouro") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                serde_yaml::Value::Number(it) => Ok(format!("{}", it)),
                _ => Err("bad yaml input: logradouro"),
            },
            None => Err("bad yaml input: logradouro"),
        }?;

        let numero_tomador = match yaml.get("numero") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                serde_yaml::Value::Number(it) => Ok(format!("{}", it)),
                _ => Err("bad yaml input: numero"),
            },
            None => Err("bad yaml input: numero"),
        }?;

        let complemento_tomador = match yaml.get("complemento") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(Some(it.clone())),
                serde_yaml::Value::Number(it) => Ok(Some(format!("{}", it))),
                _ => Err("bad yaml input: complemento"),
            },
            None => Ok(None),
        }?;

        let bairro_tomador = match yaml.get("bairro") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                serde_yaml::Value::Number(it) => Ok(format!("{}", it)),
                _ => Err("bad yaml input: bairro"),
            },
            None => Err("bad yaml input: bairro"),
        }?;

        let codigo_municipio_tomador = match yaml.get("codigo_municipio") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                serde_yaml::Value::Number(it) => Ok(format!("{}", it)),
                _ => Err("bad yaml input: codigo_municipio"),
            },
            None => Err("bad yaml input: codigo_municipio"),
        }?;

        let uf_tomador = match yaml.get("uf") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(it.clone()),
                serde_yaml::Value::Number(it) => Ok(format!("{}", it)),
                _ => Err("bad yaml input: uf"),
            },
            None => Err("bad yaml input: uf"),
        }?;

        let cep_tomador = match yaml.get("cep") {
            Some(it) => match it {
                serde_yaml::Value::String(it) => Ok(Some(it.clone())),
                serde_yaml::Value::Number(it) => Ok(Some(format!("{}", it))),
                _ => Err("bad yaml input: cep"),
            },
            None => Ok(None),
        }?;

        Ok(Self::new(
            id,
            nome_arquivo,
            data_emissao,
            natureza_operacao,
            regime_especial_tributacao,
            optante_simples_nacional,
            incentivador_cultural,
            item_lista_servico,
            codigo_tributacao_municipio,
            discriminacao,
            codigo_municipio,
            valor_servicos,
            aliquota_iss,
            cnpj,
            inscricao_municipal,
            cnpj_tomador,
            inscricao_municipal_tomador,
            razao_social_tomador,
            logradouro_tomador,
            numero_tomador,
            complemento_tomador,
            bairro_tomador,
            codigo_municipio_tomador,
            uf_tomador,
            cep_tomador,
        ))
    }
}

impl Rps {
    pub fn sign(&mut self, mut signature: XmlSignature) {
        signature.load(
            format!("#{}", self.id),
            utils::xml_events_to_xml_string(&self.inf_rps_xml_events(true)),
        );

        signature.sign();

        self.signature = Some(signature);
    }
}

impl Rps {
    fn inf_rps_xml_events(&self, xmlns: bool) -> Vec<Event> {
        let mut events = Vec::new();

        let mut elem = BytesStart::new("InfRps");
        if xmlns {
            elem.push_attribute(("xmlns", "http://www.abrasf.org.br/nfse.xsd"));
        }
        elem.push_attribute(("Id", self.id.as_str()));
        elem.push_attribute(("versao", "1.00"));
        events.push(Event::Start(elem));

        let elem = BytesStart::new("IdentificacaoRps");
        events.push(Event::Start(elem));

        let elem = BytesStart::new("Numero");
        events.push(Event::Start(elem));

        let elem = BytesText::new(&self.id);
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("Numero");
        events.push(Event::End(elem));

        let elem = BytesStart::new("Serie");
        events.push(Event::Start(elem));

        let elem = BytesText::new("1");
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("Serie");
        events.push(Event::End(elem));

        let elem = BytesStart::new("Tipo");
        events.push(Event::Start(elem));

        let elem = BytesText::new("1");
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("Tipo");
        events.push(Event::End(elem));

        let elem = BytesEnd::new("IdentificacaoRps");
        events.push(Event::End(elem));

        let elem = BytesStart::new("DataEmissao");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.data_emissao.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("DataEmissao");
        events.push(Event::End(elem));

        let elem = BytesStart::new("NaturezaOperacao");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.natureza_operacao.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("NaturezaOperacao");
        events.push(Event::End(elem));

        let elem = BytesStart::new("RegimeEspecialTributacao");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.regime_especial_tributacao.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("RegimeEspecialTributacao");
        events.push(Event::End(elem));

        let elem = BytesStart::new("OptanteSimplesNacional");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.optante_simples_nacional.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("OptanteSimplesNacional");
        events.push(Event::End(elem));

        let elem = BytesStart::new("IncentivadorCultural");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.incentivador_cultural.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("IncentivadorCultural");
        events.push(Event::End(elem));

        let elem = BytesStart::new("Status");
        events.push(Event::Start(elem));

        let elem = BytesText::new("1");
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("Status");
        events.push(Event::End(elem));

        let elem = BytesStart::new("Servico");
        events.push(Event::Start(elem));

        let elem = BytesStart::new("Valores");
        events.push(Event::Start(elem));

        let elem = BytesStart::new("ValorServicos");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.valor_servicos.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("ValorServicos");
        events.push(Event::End(elem));

        let elem = BytesStart::new("IssRetido");
        events.push(Event::Start(elem));

        let elem = BytesText::new(match self.aliquota_iss {
            Some(_) => "1",
            None => "2",
        });
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("IssRetido");
        events.push(Event::End(elem));

        if let Some(valor_iss) = &self.valor_iss {
            let elem = BytesStart::new("ValorIss");
            events.push(Event::Start(elem));

            let elem = BytesText::new(valor_iss.as_str());
            events.push(Event::Text(elem));

            let elem = BytesEnd::new("ValorIss");
            events.push(Event::End(elem));

            let elem = BytesStart::new("ValorIssRetido");
            events.push(Event::Start(elem));

            let elem = BytesText::new(valor_iss.as_str());
            events.push(Event::Text(elem));

            let elem = BytesEnd::new("ValorIssRetido");
            events.push(Event::End(elem));
        }

        let elem = BytesStart::new("BaseCalculo");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.valor_servicos.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("BaseCalculo");
        events.push(Event::End(elem));

        if let Some(aliquota_iss) = &self.aliquota_iss {
            let elem = BytesStart::new("Aliquota");
            events.push(Event::Start(elem));

            let elem = BytesText::new(aliquota_iss.as_str());
            events.push(Event::Text(elem));

            let elem = BytesEnd::new("Aliquota");
            events.push(Event::End(elem));
        }

        let elem = BytesStart::new("ValorLiquidoNfse");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.valor_liquido.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("ValorLiquidoNfse");
        events.push(Event::End(elem));

        let elem = BytesEnd::new("Valores");
        events.push(Event::End(elem));

        let elem = BytesStart::new("ItemListaServico");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.item_lista_servico.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("ItemListaServico");
        events.push(Event::End(elem));

        let elem = BytesStart::new("CodigoTributacaoMunicipio");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.codigo_tributacao_municipio.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("CodigoTributacaoMunicipio");
        events.push(Event::End(elem));

        let elem = BytesStart::new("Discriminacao");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.discriminacao.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("Discriminacao");
        events.push(Event::End(elem));

        let elem = BytesStart::new("CodigoMunicipio");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.codigo_municipio.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("CodigoMunicipio");
        events.push(Event::End(elem));

        let elem = BytesEnd::new("Servico");
        events.push(Event::End(elem));

        let elem = BytesStart::new("Prestador");
        events.push(Event::Start(elem));

        let elem = BytesStart::new("Cnpj");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.cnpj_prestador.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("Cnpj");
        events.push(Event::End(elem));

        let elem = BytesStart::new("InscricaoMunicipal");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.inscricao_municipal_prestador.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("InscricaoMunicipal");
        events.push(Event::End(elem));

        let elem = BytesEnd::new("Prestador");
        events.push(Event::End(elem));

        let elem = BytesStart::new("Tomador");
        events.push(Event::Start(elem));

        if let Some(cnpj_tomador) = &self.cnpj_tomador {
            let elem = BytesStart::new("IdentificacaoTomador");
            events.push(Event::Start(elem));

            let elem = BytesStart::new("CpfCnpj");
            events.push(Event::Start(elem));

            let elem = BytesStart::new("Cnpj");
            events.push(Event::Start(elem));

            let elem = BytesText::new(cnpj_tomador.as_str());
            events.push(Event::Text(elem));

            let elem = BytesEnd::new("Cnpj");
            events.push(Event::End(elem));

            let elem = BytesEnd::new("CpfCnpj");
            events.push(Event::End(elem));

            if let Some(inscricao_municipal_tomador) = &self.inscricao_municipal_tomador {
                let elem = BytesStart::new("InscricaoMunicipal");
                events.push(Event::Start(elem));

                let elem = BytesText::new(inscricao_municipal_tomador.as_str());
                events.push(Event::Text(elem));

                let elem = BytesEnd::new("InscricaoMunicipal");
                events.push(Event::End(elem));
            }

            let elem = BytesEnd::new("IdentificacaoTomador");
            events.push(Event::End(elem));
        }

        let elem = BytesStart::new("RazaoSocial");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.razao_social_tomador.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("RazaoSocial");
        events.push(Event::End(elem));

        let elem = BytesStart::new("Endereco");
        events.push(Event::Start(elem));

        let elem = BytesStart::new("Endereco");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.logradouro_tomador.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("Endereco");
        events.push(Event::End(elem));

        let elem = BytesStart::new("Numero");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.numero_tomador.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("Numero");
        events.push(Event::End(elem));

        if let Some(complemento_tomador) = &self.complemento_tomador {
            let elem = BytesStart::new("Complemento");
            events.push(Event::Start(elem));

            let elem = BytesText::new(complemento_tomador.as_str());
            events.push(Event::Text(elem));

            let elem = BytesEnd::new("Complemento");
            events.push(Event::End(elem));
        }

        let elem = BytesStart::new("Bairro");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.bairro_tomador.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("Bairro");
        events.push(Event::End(elem));

        let elem = BytesStart::new("CodigoMunicipio");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.codigo_municipio_tomador.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("CodigoMunicipio");
        events.push(Event::End(elem));

        let elem = BytesStart::new("Uf");
        events.push(Event::Start(elem));

        let elem = BytesText::new(self.uf_tomador.as_str());
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("Uf");
        events.push(Event::End(elem));

        if let Some(cep_tomador) = &self.cep_tomador {
            let elem = BytesStart::new("Cep");
            events.push(Event::Start(elem));

            let elem = BytesText::new(cep_tomador.as_str());
            events.push(Event::Text(elem));

            let elem = BytesEnd::new("Cep");
            events.push(Event::End(elem));
        }

        let elem = BytesEnd::new("Endereco");
        events.push(Event::End(elem));

        let elem = BytesEnd::new("Tomador");
        events.push(Event::End(elem));

        let elem = BytesEnd::new("InfRps");
        events.push(Event::End(elem));

        events
    }
}

impl Rps {
    pub fn rps_xml_events(&self, xmlns: bool) -> Vec<Event> {
        let mut events = Vec::new();

        let mut elem = BytesStart::new("Rps");
        if xmlns {
            elem.push_attribute(("xmlns", "http://www.abrasf.org.br/nfse.xsd"));
        }
        elem.push_attribute(("versao", "1.00"));
        events.push(Event::Start(elem));

        self.inf_rps_xml_events(false)
            .iter()
            .for_each(|e| events.push(e.to_owned()));

        if let Some(signature) = &self.signature {
            signature
                .signature_xml_events()
                .iter()
                .for_each(|e| events.push(e.to_owned()));
        };

        let elem = BytesEnd::new("Rps");
        events.push(Event::End(elem));

        events
    }
}

impl Rps {
    pub fn uniquely_identify(&self) -> String {
        format!(
            "{}|{}|{}",
            self.razao_social_tomador.clone(),
            self.discriminacao.clone(),
            self.valor_servicos.clone()
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::algorithms::CanonicalizationAlgorithm;
    use crate::algorithms::DigestAlgorithm;
    use crate::algorithms::SignatureAlgorithm;
    use crate::utils;

    #[test]
    fn should_create_signed_rps() {
        let signature = super::XmlSignature::new(
            CanonicalizationAlgorithm::NoOp,
            SignatureAlgorithm::Echo(String::from("the_signature")),
            DigestAlgorithm::Echo(String::from("the_digest")),
            String::from("the_certificate"),
        );

        let yaml = serde_yaml::from_str(
            "
id: 1234
nome_arquivo: ACME_1234
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
",
        )
        .unwrap();

        let mut rps = super::Rps::from_yaml(
            yaml,
            String::from("cnpj_prestador"),
            String::from("inscricao_municipal_prestador"),
            String::from("codigo_municipio_prestador"),
        )
        .unwrap();

        rps.sign(signature);

        assert_eq!(
            utils::xml_events_to_xml_string(&rps.rps_xml_events(true)),
            String::from(
                r##"<Rps xmlns="http://www.abrasf.org.br/nfse.xsd" versao="1.00"><InfRps Id="1234" versao="1.00"><IdentificacaoRps><Numero>1234</Numero><Serie>1</Serie><Tipo>1</Tipo></IdentificacaoRps><DataEmissao>data_emissao</DataEmissao><NaturezaOperacao>natureza_operacao</NaturezaOperacao><RegimeEspecialTributacao>regime_especial_tributacao</RegimeEspecialTributacao><OptanteSimplesNacional>optante_simples_nacional</OptanteSimplesNacional><IncentivadorCultural>incentivador_cultural</IncentivadorCultural><Status>1</Status><Servico><Valores><ValorServicos>1000.00</ValorServicos><IssRetido>1</IssRetido><ValorIss>20.00</ValorIss><ValorIssRetido>20.00</ValorIssRetido><BaseCalculo>1000.00</BaseCalculo><Aliquota>0.02</Aliquota><ValorLiquidoNfse>980.00</ValorLiquidoNfse></Valores><ItemListaServico>item_lista_servico</ItemListaServico><CodigoTributacaoMunicipio>codigo_tributacao_municipio</CodigoTributacaoMunicipio><Discriminacao>discriminacao</Discriminacao><CodigoMunicipio>codigo_municipio_prestador</CodigoMunicipio></Servico><Prestador><Cnpj>cnpj_prestador</Cnpj><InscricaoMunicipal>inscricao_municipal_prestador</InscricaoMunicipal></Prestador><Tomador><IdentificacaoTomador><CpfCnpj><Cnpj>cnpj_tomador</Cnpj></CpfCnpj><InscricaoMunicipal>inscricao_municipal_tomador</InscricaoMunicipal></IdentificacaoTomador><RazaoSocial>razao_social_tomador</RazaoSocial><Endereco><Endereco>logradouro_tomador</Endereco><Numero>numero_tomador</Numero><Complemento>complemento_tomador</Complemento><Bairro>bairro_tomador</Bairro><CodigoMunicipio>codigo_municipio_tomador</CodigoMunicipio><Uf>uf_tomador</Uf><Cep>cep_tomador</Cep></Endereco></Tomador></InfRps><Signature xmlns="http://www.w3.org/2000/09/xmldsig#"><SignedInfo><CanonicalizationMethod Algorithm="noop-c14n"></CanonicalizationMethod><SignatureMethod Algorithm="echo-signature"></SignatureMethod><Reference URI="#1234"><Transforms><Transform Algorithm="noop-c14n"></Transform></Transforms><DigestMethod Algorithm="echo-digest"></DigestMethod><DigestValue>the_digest</DigestValue></Reference></SignedInfo><SignatureValue>the_signature</SignatureValue><KeyInfo><X509Data><X509Certificate>the_certificate</X509Certificate></X509Data></KeyInfo></Signature></Rps>"##
            )
        );
    }
}
