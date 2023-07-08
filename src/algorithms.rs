use base64ct::{Base64, Encoding};
use rsa::{
    signature::{SignatureEncoding, Signer},
    RsaPrivateKey,
};
use sha1::{Digest, Sha1};

#[derive(Clone)]
pub enum CanonicalizationAlgorithm {
    ExclusiveXMLCanonicalization,
    #[cfg(test)]
    NoOp,
}

impl CanonicalizationAlgorithm {
    pub fn as_str(&self) -> &str {
        match &self {
            CanonicalizationAlgorithm::ExclusiveXMLCanonicalization => {
                "http://www.w3.org/2001/10/xml-exc-c14n#"
            }
            #[cfg(test)]
            CanonicalizationAlgorithm::NoOp => "noop-c14n",
        }
    }
}

impl CanonicalizationAlgorithm {
    pub fn run(&self, payload: String) -> String {
        match &self {
            CanonicalizationAlgorithm::ExclusiveXMLCanonicalization => payload,
            #[cfg(test)]
            CanonicalizationAlgorithm::NoOp => payload,
        }
    }
}

#[derive(Clone)]
pub enum SignatureAlgorithm {
    RsaSha1(RsaPrivateKey),
    #[cfg(test)]
    Echo(String),
}

impl SignatureAlgorithm {
    pub fn as_str(&self) -> &str {
        match &self {
            SignatureAlgorithm::RsaSha1(_) => "http://www.w3.org/2000/09/xmldsig#rsa-sha1",
            #[cfg(test)]
            SignatureAlgorithm::Echo(_) => "echo-signature",
        }
    }
}

impl SignatureAlgorithm {
    pub fn run(&self, payload: String) -> String {
        match &self {
            SignatureAlgorithm::RsaSha1(private_key) => {
                let signing_key = rsa::pkcs1v15::SigningKey::<Sha1>::new(private_key.to_owned());
                Base64::encode_string(&signing_key.sign(payload.as_bytes()).to_bytes())
            }
            #[cfg(test)]
            SignatureAlgorithm::Echo(s) => s.to_owned(),
        }
    }
}

#[derive(Clone)]
pub enum DigestAlgorithm {
    Sha1,
    #[cfg(test)]
    Echo(String),
}

impl DigestAlgorithm {
    pub fn as_str(&self) -> &str {
        match &self {
            DigestAlgorithm::Sha1 => "http://www.w3.org/2000/09/xmldsig#sha1",
            #[cfg(test)]
            DigestAlgorithm::Echo(_) => "echo-digest",
        }
    }
}

impl DigestAlgorithm {
    pub fn run(&self, payload: String) -> String {
        match &self {
            DigestAlgorithm::Sha1 => Base64::encode_string(&Sha1::digest(payload)),
            #[cfg(test)]
            DigestAlgorithm::Echo(s) => s.to_owned(),
        }
    }
}
