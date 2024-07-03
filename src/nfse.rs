use quick_xml::events::Event;
use quick_xml::reader::Reader;

#[derive(PartialEq, Debug)]
pub struct Nfse {
    numero: String,
    codigo_verificacao: String,
    data_emissao: String,
    competencia: String,
    natureza_operacao: String,
    regime_especial_tributacao: String,
    optante_simples_nacional: String,
    incentivador_cultural: String,
    outras_informacoes: String,
    valor_servicos: String,
    aliquota_iss: String,
    valor_iss: String,
    codigo_tributacao_municipio: String,
    discriminacao: String,
    codigo_municipio: String,
    cnpj_prestador: String,
    inscricao_municipal_prestador: Option<String>,
    razao_social_prestador: String,
    logradouro_prestador: String,
    numero_prestador: String,
    complemento_prestador: Option<String>,
    bairro_prestador: String,
    codigo_municipio_prestador: String,
    uf_prestador: String,
    cep_prestador: String,
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
}

impl Nfse {
    pub fn from_xml_string(xml: &str) -> Result<Self, String> {
        let mut reader = Reader::from_str(&xml);

        let mut names = Vec::new();

        let mut numero: Option<String> = None;
        let mut codigo_verificacao: Option<String> = None;
        let mut data_emissao: Option<String> = None;
        let mut competencia: Option<String> = None;
        let mut natureza_operacao: Option<String> = None;
        let mut regime_especial_tributacao: Option<String> = None;
        let mut optante_simples_nacional: Option<String> = None;
        let mut incentivador_cultural: Option<String> = None;
        let mut outras_informacoes: Option<String> = None;
        let mut valor_servicos: Option<String> = None;
        let mut aliquota_iss: Option<String> = None;
        let mut valor_iss: Option<String> = None;
        let mut codigo_tributacao_municipio: Option<String> = None;
        let mut discriminacao: Option<String> = None;
        let mut codigo_municipio: Option<String> = None;
        let mut cnpj_prestador: Option<String> = None;
        let mut inscricao_municipal_prestador: Option<String> = None;
        let mut razao_social_prestador: Option<String> = None;
        let mut logradouro_prestador: Option<String> = None;
        let mut numero_prestador: Option<String> = None;
        let mut complemento_prestador: Option<String> = None;
        let mut bairro_prestador: Option<String> = None;
        let mut codigo_municipio_prestador: Option<String> = None;
        let mut uf_prestador: Option<String> = None;
        let mut cep_prestador: Option<String> = None;
        let mut cnpj_tomador: Option<String> = None;
        let mut inscricao_municipal_tomador: Option<String> = None;
        let mut razao_social_tomador: Option<String> = None;
        let mut logradouro_tomador: Option<String> = None;
        let mut numero_tomador: Option<String> = None;
        let mut complemento_tomador: Option<String> = None;
        let mut bairro_tomador: Option<String> = None;
        let mut codigo_municipio_tomador: Option<String> = None;
        let mut uf_tomador: Option<String> = None;
        let mut cep_tomador: Option<String> = None;

        loop {
            match reader.read_event() {
                Ok(Event::Start(e)) => {
                    names.push(e.clone());
                    Ok(())
                }
                Ok(Event::End(e)) => {
                    match names.pop() {
                        Some(pop) => {
                            if pop.name() != e.name() {
                                return Err(String::from("bad xml"));
                            };
                        }
                        None => {
                            return Err(String::from("bad xml"));
                        }
                    }
                    Ok(())
                }
                Ok(Event::Text(e)) => {
                    let mut names_iter = names.iter();
                    match names_iter.next() {
                        Some(elem) => match elem.name().as_ref() {
                            b"InfNfse" => match names_iter.next() {
                                Some(elem) => match elem.name().as_ref() {
                                    b"Numero" => {
                                        numero = Some(String::from_utf8(e.to_vec()).unwrap())
                                    }
                                    b"CodigoVerificacao" => {
                                        codigo_verificacao =
                                            Some(String::from_utf8(e.to_vec()).unwrap())
                                    }
                                    b"DataEmissao" => {
                                        data_emissao = Some(String::from_utf8(e.to_vec()).unwrap())
                                    }
                                    b"Competencia" => {
                                        competencia = Some(String::from_utf8(e.to_vec()).unwrap())
                                    }
                                    b"NaturezaOperacao" => {
                                        natureza_operacao =
                                            Some(String::from_utf8(e.to_vec()).unwrap())
                                    }
                                    b"RegimeEspecialTributacao" => {
                                        regime_especial_tributacao =
                                            Some(String::from_utf8(e.to_vec()).unwrap())
                                    }
                                    b"OptanteSimplesNacional" => {
                                        optante_simples_nacional =
                                            Some(String::from_utf8(e.to_vec()).unwrap())
                                    }
                                    b"IncentivadorCultural" => {
                                        incentivador_cultural =
                                            Some(String::from_utf8(e.to_vec()).unwrap())
                                    }
                                    b"OutrasInformacoes" => {
                                        outras_informacoes =
                                            Some(String::from_utf8(e.to_vec()).unwrap())
                                    }
                                    b"Servico" => match names_iter.next() {
                                        Some(elem) => match elem.name().as_ref() {
                                            b"CodigoTributacaoMunicipio" => {
                                                codigo_tributacao_municipio =
                                                    Some(String::from_utf8(e.to_vec()).unwrap())
                                            }
                                            b"Discriminacao" => {
                                                discriminacao =
                                                    Some(String::from_utf8(e.to_vec()).unwrap())
                                            }
                                            b"CodigoMunicipio" => {
                                                codigo_municipio =
                                                    Some(String::from_utf8(e.to_vec()).unwrap())
                                            }
                                            b"Valores" => match names_iter.next() {
                                                Some(elem) => match elem.name().as_ref() {
                                                    b"ValorServicos" => {
                                                        valor_servicos = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    b"Aliquota" => {
                                                        aliquota_iss = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    b"ValorIss" => {
                                                        valor_iss = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    _ => {}
                                                },
                                                None => {
                                                    return Err(String::from("bad xml"));
                                                }
                                            },
                                            _ => {}
                                        },
                                        None => {
                                            return Err(String::from("bad xml"));
                                        }
                                    },
                                    b"PrestadorServico" => match names_iter.next() {
                                        Some(elem) => match elem.name().as_ref() {
                                            b"RazaoSocial" => {
                                                razao_social_prestador =
                                                    Some(String::from_utf8(e.to_vec()).unwrap())
                                            }
                                            b"IdentificacaoPrestador" => match names_iter.next() {
                                                Some(elem) => match elem.name().as_ref() {
                                                    b"Cnpj" => {
                                                        cnpj_prestador = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    b"InscricaoMunicipal" => {
                                                        inscricao_municipal_prestador = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    _ => {}
                                                },
                                                None => {
                                                    return Err(String::from("bad xml"));
                                                }
                                            },
                                            b"Endereco" => match names_iter.next() {
                                                Some(elem) => match elem.name().as_ref() {
                                                    b"Endereco" => {
                                                        logradouro_prestador = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    b"Numero" => {
                                                        numero_prestador = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    b"Complemento" => {
                                                        complemento_prestador = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    b"Bairro" => {
                                                        bairro_prestador = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    b"CodigoMunicipio" => {
                                                        codigo_municipio_prestador = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    b"Uf" => {
                                                        uf_prestador = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    b"Cep" => {
                                                        cep_prestador = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    _ => {}
                                                },
                                                None => {
                                                    return Err(String::from("bad xml"));
                                                }
                                            },
                                            _ => {}
                                        },
                                        None => {
                                            return Err(String::from("bad xml"));
                                        }
                                    },
                                    b"TomadorServico" => match names_iter.next() {
                                        Some(elem) => match elem.name().as_ref() {
                                            b"RazaoSocial" => {
                                                razao_social_tomador =
                                                    Some(String::from_utf8(e.to_vec()).unwrap())
                                            }
                                            b"IdentificacaoTomador" => match names_iter.next() {
                                                Some(elem) => match elem.name().as_ref() {
                                                    b"CpfCnpj" => match names_iter.next() {
                                                        Some(elem) => match elem.name().as_ref() {
                                                            b"Cnpj" => {
                                                                cnpj_tomador = Some(
                                                                    String::from_utf8(e.to_vec())
                                                                        .unwrap(),
                                                                )
                                                            }
                                                            _ => {}
                                                        },
                                                        None => {
                                                            return Err(String::from("bad xml"));
                                                        }
                                                    },
                                                    b"InscricaoMunicipal" => {
                                                        inscricao_municipal_tomador = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    _ => {}
                                                },
                                                None => {
                                                    return Err(String::from("bad xml"));
                                                }
                                            },
                                            b"Endereco" => match names_iter.next() {
                                                Some(elem) => match elem.name().as_ref() {
                                                    b"Endereco" => {
                                                        logradouro_tomador = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    b"Numero" => {
                                                        numero_tomador = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    b"Complemento" => {
                                                        complemento_tomador = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    b"Bairro" => {
                                                        bairro_tomador = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    b"CodigoMunicipio" => {
                                                        codigo_municipio_tomador = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    b"Uf" => {
                                                        uf_tomador = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    b"Cep" => {
                                                        cep_tomador = Some(
                                                            String::from_utf8(e.to_vec()).unwrap(),
                                                        )
                                                    }
                                                    _ => {}
                                                },
                                                None => {
                                                    return Err(String::from("bad xml"));
                                                }
                                            },
                                            _ => {}
                                        },
                                        None => {
                                            return Err(String::from("bad xml"));
                                        }
                                    },
                                    _ => {}
                                },
                                None => {
                                    return Err(String::from("bad xml"));
                                }
                            },
                            e => {
                                return Err(format!(
                                    "unexpected xml {}",
                                    String::from_utf8(e.to_vec()).unwrap()
                                ));
                            }
                        },
                        None => {
                            return Err(String::from("bad xml"));
                        }
                    };
                    Ok(())
                }
                Ok(Event::Eof) => break,
                Ok(_) => Err(String::from("unexpected xml")),
                Err(e) => Err(format!("error in xml: {e}")),
            }?;
        }

        let numero = numero.ok_or(String::from("missing numero"))?;
        let codigo_verificacao =
            codigo_verificacao.ok_or(String::from("missing codigo_verificacao"))?;
        let data_emissao = data_emissao.ok_or(String::from("missing data_emissao"))?;
        let competencia = competencia.ok_or(String::from("missing competencia"))?;
        let natureza_operacao =
            natureza_operacao.ok_or(String::from("missing natureza_operacao"))?;
        let regime_especial_tributacao =
            regime_especial_tributacao.ok_or(String::from("missing regime_especial_tributacao"))?;
        let optante_simples_nacional =
            optante_simples_nacional.ok_or(String::from("missing optante_simples_nacional"))?;
        let incentivador_cultural =
            incentivador_cultural.ok_or(String::from("missing incentivador_cultural"))?;
        let outras_informacoes =
            outras_informacoes.ok_or(String::from("missing outras_informacoes"))?;
        let valor_servicos = valor_servicos.ok_or(String::from("missing valor_servicos"))?;
        let aliquota_iss = aliquota_iss.ok_or(String::from("missing aliquota_iss"))?;
        let valor_iss = valor_iss.ok_or(String::from("missing valor_iss"))?;
        let codigo_tributacao_municipio = codigo_tributacao_municipio
            .ok_or(String::from("missing codigo_tributacao_municipio"))?;
        let discriminacao = discriminacao.ok_or(String::from("missing discriminacao"))?;
        let codigo_municipio = codigo_municipio.ok_or(String::from("missing codigo_municipio"))?;
        let cnpj_prestador = cnpj_prestador.ok_or(String::from("missing cnpj_prestador"))?;
        let razao_social_prestador =
            razao_social_prestador.ok_or(String::from("missing razao_social_prestador"))?;
        let logradouro_prestador =
            logradouro_prestador.ok_or(String::from("missing logradouro_prestador"))?;
        let numero_prestador = numero_prestador.ok_or(String::from("missing numero_prestador"))?;
        let bairro_prestador = bairro_prestador.ok_or(String::from("missing bairro_prestador"))?;
        let codigo_municipio_prestador =
            codigo_municipio_prestador.ok_or(String::from("missing codigo_municipio_prestador"))?;
        let uf_prestador = uf_prestador.ok_or(String::from("missing uf_prestador"))?;
        let cep_prestador = cep_prestador.ok_or(String::from("missing cep_prestador"))?;
        let razao_social_tomador =
            razao_social_tomador.ok_or(String::from("missing razao_social_tomador"))?;
        let logradouro_tomador =
            logradouro_tomador.ok_or(String::from("missing logradouro_tomador"))?;
        let numero_tomador = numero_tomador.ok_or(String::from("missing numero_tomador"))?;
        let bairro_tomador = bairro_tomador.ok_or(String::from("missing bairro_tomador"))?;
        let codigo_municipio_tomador =
            codigo_municipio_tomador.ok_or(String::from("missing codigo_municipio_tomador"))?;
        let uf_tomador = uf_tomador.ok_or(String::from("missing uf_tomador"))?;

        Ok(Self {
            numero,
            codigo_verificacao,
            data_emissao,
            competencia,
            natureza_operacao,
            regime_especial_tributacao,
            optante_simples_nacional,
            incentivador_cultural,
            outras_informacoes,
            valor_servicos,
            aliquota_iss,
            valor_iss,
            codigo_tributacao_municipio,
            discriminacao,
            codigo_municipio,
            cnpj_prestador,
            inscricao_municipal_prestador,
            razao_social_prestador,
            logradouro_prestador,
            numero_prestador,
            complemento_prestador,
            bairro_prestador,
            codigo_municipio_prestador,
            uf_prestador,
            cep_prestador,
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
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::nfse::Nfse;

    #[test]
    fn should_create_nfse_from_xml() {
        let example = r#"<InfNfse Id="nfse"><Numero>12345</Numero><CodigoVerificacao>67890</CodigoVerificacao><DataEmissao>2020-01-01T01:02:03</DataEmissao><NaturezaOperacao>3</NaturezaOperacao><RegimeEspecialTributacao>6</RegimeEspecialTributacao><OptanteSimplesNacional>1</OptanteSimplesNacional><IncentivadorCultural>2</IncentivadorCultural><Competencia>2020-01-01T00:00:00</Competencia><OutrasInformacoes>NFS-e gerada em ambiente de teste. NÃO TEM VALOR JURÍDICO NEM FISCAL.</OutrasInformacoes><Servico><Valores><ValorServicos>95.31</ValorServicos><IssRetido>2</IssRetido><ValorIss>0.00</ValorIss><BaseCalculo>95.31</BaseCalculo><Aliquota>0.0217</Aliquota><ValorLiquidoNfse>95.31</ValorLiquidoNfse></Valores><ItemListaServico>1.04</ItemListaServico><CodigoTributacaoMunicipio>10400188</CodigoTributacaoMunicipio><Discriminacao>Consultoria em desenvolvimento de software</Discriminacao><CodigoMunicipio>3106200</CodigoMunicipio></Servico><PrestadorServico><IdentificacaoPrestador><Cnpj>12345678000190</Cnpj><InscricaoMunicipal>12345670018</InscricaoMunicipal></IdentificacaoPrestador><RazaoSocial>NOME DA EMPRESA</RazaoSocial><NomeFantasia>NOME FANTASIA</NomeFantasia><Endereco><Endereco>RUA DO PRESTADOR</Endereco><Numero>12</Numero><Complemento>SALA 01</Complemento><Bairro>Bairro Um</Bairro><CodigoMunicipio>3106200</CodigoMunicipio><Uf>MG</Uf><Cep>34567890</Cep></Endereco></PrestadorServico><TomadorServico><IdentificacaoTomador><CpfCnpj><Cnpj>12345678000290</Cnpj></CpfCnpj><InscricaoMunicipal>12345670019</InscricaoMunicipal></IdentificacaoTomador><RazaoSocial>NOME DO TOMADOR</RazaoSocial><Endereco><Endereco>RUA DO TOMADOR</Endereco><Numero>34</Numero><Complemento>SALA 02</Complemento><Bairro>Bairro Dois</Bairro><CodigoMunicipio>3106200</CodigoMunicipio><Uf>MG</Uf><Cep>34567891</Cep></Endereco></TomadorServico><OrgaoGerador><CodigoMunicipio>3106200</CodigoMunicipio><Uf>MG</Uf></OrgaoGerador></InfNfse>"#;
        let nfse = Nfse::from_xml_string(example).unwrap();
        assert_eq!(
            nfse,
            Nfse {
                numero: String::from("12345"),
                codigo_verificacao: String::from("67890"),
                data_emissao: String::from("2020-01-01T01:02:03"),
                competencia: String::from("2020-01-01T00:00:00"),
                natureza_operacao: String::from("3"),
                regime_especial_tributacao: String::from("6"),
                optante_simples_nacional: String::from("1"),
                incentivador_cultural: String::from("2"),
                outras_informacoes: String::from(
                    "NFS-e gerada em ambiente de teste. NÃO TEM VALOR JURÍDICO NEM FISCAL."
                ),
                valor_servicos: String::from("95.31"),
                aliquota_iss: String::from("0.0217"),
                valor_iss: String::from("0.00"),
                codigo_tributacao_municipio: String::from("10400188"),
                discriminacao: String::from("Consultoria em desenvolvimento de software"),
                codigo_municipio: String::from("3106200"),
                cnpj_prestador: String::from("12345678000190"),
                inscricao_municipal_prestador: Some(String::from("12345670018")),
                razao_social_prestador: String::from("NOME DA EMPRESA"),
                logradouro_prestador: String::from("RUA DO PRESTADOR"),
                numero_prestador: String::from("12"),
                complemento_prestador: Some(String::from("SALA 01")),
                bairro_prestador: String::from("Bairro Um"),
                codigo_municipio_prestador: String::from("3106200"),
                uf_prestador: String::from("MG"),
                cep_prestador: String::from("34567890"),
                cnpj_tomador: Some(String::from("12345678000290")),
                inscricao_municipal_tomador: Some(String::from("12345670019")),
                razao_social_tomador: String::from("NOME DO TOMADOR"),
                logradouro_tomador: String::from("RUA DO TOMADOR"),
                numero_tomador: String::from("34"),
                complemento_tomador: Some(String::from("SALA 02")),
                bairro_tomador: String::from("Bairro Dois"),
                codigo_municipio_tomador: String::from("3106200"),
                uf_tomador: String::from("MG"),
                cep_tomador: Some(String::from("34567891")),
            }
        );
    }
}
