use quick_xml::events::BytesEnd;
use quick_xml::events::BytesStart;
use quick_xml::events::BytesText;
use quick_xml::events::Event;

use crate::algorithms::CanonicalizationAlgorithm;
use crate::algorithms::DigestAlgorithm;
use crate::algorithms::SignatureAlgorithm;
use crate::utils;

#[derive(Clone)]
pub struct XmlSignature {
    canonicalization_method: CanonicalizationAlgorithm,
    signature_method: SignatureAlgorithm,
    digest_method: DigestAlgorithm,
    certificate: String,
    payload_uri: Option<String>,
    payload: Option<String>,
    digest_value: Option<String>,
    signature_value: Option<String>,
}

impl XmlSignature {
    pub fn new(
        canonicalization_method: CanonicalizationAlgorithm,
        signature_method: SignatureAlgorithm,
        digest_method: DigestAlgorithm,
        certificate: String,
    ) -> Self {
        Self {
            canonicalization_method,
            signature_method,
            digest_method,
            certificate,
            payload_uri: None,
            payload: None,
            digest_value: None,
            signature_value: None,
        }
    }
}

impl XmlSignature {
    pub fn load(&mut self, uri: String, payload: String) {
        self.payload_uri = Some(uri);
        self.payload = Some(payload);
    }
}

impl XmlSignature {
    pub fn sign(&mut self) {
        let payload = self.payload.clone().expect("cannot sign without payload");

        let payload = self.canonicalization_method.run(payload.to_owned());

        let digest_value = self.digest_method.run(payload);

        self.digest_value = Some(digest_value);

        let signed_info_xml_events = self.signed_info_xml_events(true);

        let signed_info_xml_string = utils::xml_events_to_xml_string(&signed_info_xml_events);

        let signature_value = self.signature_method.run(signed_info_xml_string);

        self.signature_value = Some(signature_value);
    }
}

impl XmlSignature {
    fn signed_info_xml_events(&self, xmlns: bool) -> Vec<Event> {
        let digest_value = self.digest_value.as_ref().expect("need to sign first");

        let mut events = Vec::new();

        let mut elem = BytesStart::new("SignedInfo");
        if xmlns {
            elem.push_attribute(("xmlns", "http://www.w3.org/2000/09/xmldsig#"));
        }
        events.push(Event::Start(elem));

        let mut elem = BytesStart::new("CanonicalizationMethod");
        elem.push_attribute(("Algorithm", self.canonicalization_method.as_str()));
        events.push(Event::Start(elem));

        let elem = BytesEnd::new("CanonicalizationMethod");
        events.push(Event::End(elem));

        let mut elem = BytesStart::new("SignatureMethod");
        elem.push_attribute(("Algorithm", self.signature_method.as_str()));
        events.push(Event::Start(elem));

        let elem = BytesEnd::new("SignatureMethod");
        events.push(Event::End(elem));

        let mut elem = BytesStart::new("Reference");
        elem.push_attribute(("URI", self.payload_uri.clone().unwrap().as_str()));
        events.push(Event::Start(elem));

        let elem = BytesStart::new("Transforms");
        events.push(Event::Start(elem));

        let mut elem = BytesStart::new("Transform");
        elem.push_attribute(("Algorithm", self.canonicalization_method.as_str()));
        events.push(Event::Start(elem));

        let elem = BytesEnd::new("Transform");
        events.push(Event::End(elem));

        let elem = BytesEnd::new("Transforms");
        events.push(Event::End(elem));

        let mut elem = BytesStart::new("DigestMethod");
        elem.push_attribute(("Algorithm", self.digest_method.as_str()));
        events.push(Event::Start(elem));

        let elem = BytesEnd::new("DigestMethod");
        events.push(Event::End(elem));

        let elem = BytesStart::new("DigestValue");
        events.push(Event::Start(elem));

        let elem = BytesText::new(digest_value);
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("DigestValue");
        events.push(Event::End(elem));

        let elem = BytesEnd::new("Reference");
        events.push(Event::End(elem));

        let elem = BytesEnd::new("SignedInfo");
        events.push(Event::End(elem));

        events
    }
}

impl XmlSignature {
    pub fn signature_xml_events(&self) -> Vec<Event> {
        let signature_value = self.signature_value.as_ref().expect("need to sign first");

        let mut events = Vec::new();

        let mut elem = BytesStart::new("Signature");
        elem.push_attribute(("xmlns", "http://www.w3.org/2000/09/xmldsig#"));
        events.push(Event::Start(elem));

        self.signed_info_xml_events(false)
            .iter()
            .for_each(|e| events.push(e.to_owned()));

        let elem = BytesStart::new("SignatureValue");
        events.push(Event::Start(elem));

        let elem = BytesText::new(signature_value);
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("SignatureValue");
        events.push(Event::End(elem));

        let elem = BytesStart::new("KeyInfo");
        events.push(Event::Start(elem));

        let elem = BytesStart::new("X509Data");
        events.push(Event::Start(elem));

        let elem = BytesStart::new("X509Certificate");
        events.push(Event::Start(elem));

        let elem = BytesText::new(&self.certificate);
        events.push(Event::Text(elem));

        let elem = BytesEnd::new("X509Certificate");
        events.push(Event::End(elem));

        let elem = BytesEnd::new("X509Data");
        events.push(Event::End(elem));

        let elem = BytesEnd::new("KeyInfo");
        events.push(Event::End(elem));

        let elem = BytesEnd::new("Signature");
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
    fn should_sign_payload() {
        let mut signature = super::XmlSignature::new(
            CanonicalizationAlgorithm::NoOp,
            SignatureAlgorithm::Echo(String::from("the_signature")),
            DigestAlgorithm::Echo(String::from("the_digest")),
            String::from("the_certificate"),
        );
        signature.load(
            String::from("#URI"),
            String::from("<Payload>To Sign</Payload>"),
        );
        signature.sign();
        assert_eq!(
            utils::xml_events_to_xml_string(&signature.signature_xml_events()),
            String::from(
                r##"<Signature xmlns="http://www.w3.org/2000/09/xmldsig#"><SignedInfo><CanonicalizationMethod Algorithm="noop-c14n"></CanonicalizationMethod><SignatureMethod Algorithm="echo-signature"></SignatureMethod><Reference URI="#URI"><Transforms><Transform Algorithm="noop-c14n"></Transform></Transforms><DigestMethod Algorithm="echo-digest"></DigestMethod><DigestValue>the_digest</DigestValue></Reference></SignedInfo><SignatureValue>the_signature</SignatureValue><KeyInfo><X509Data><X509Certificate>the_certificate</X509Certificate></X509Data></KeyInfo></Signature>"##
            )
        );
    }
}
