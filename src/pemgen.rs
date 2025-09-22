use rcgen::{
    BasicConstraints, Certificate, CertificateParams, DistinguishedName, DnType, IsCa, Issuer,
    KeyPair, KeyUsagePurpose, SanType,
};
use std::{fs, thread::scope};
use time::OffsetDateTime;

fn create_ca() -> (Certificate, Issuer<'static, KeyPair>) {
    let mut ca_cert_params = CertificateParams::default();

    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, "Azo Custom Cert");
    dn.push(DnType::CountryName, "CN");
    dn.push(DnType::LocalityName, "Shanghai");
    dn.push(DnType::OrganizationName, "azazo");
    dn.push(DnType::OrganizationalUnitName, "common");
    dn.push(DnType::StateOrProvinceName, "Shanghai");
    ca_cert_params.distinguished_name = dn;

    ca_cert_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);

    ca_cert_params.key_usages = [
        KeyUsagePurpose::CrlSign,
        KeyUsagePurpose::KeyCertSign,
        KeyUsagePurpose::DigitalSignature,
        KeyUsagePurpose::KeyEncipherment,
    ]
    .into();

    let now = OffsetDateTime::now_utc();
    ca_cert_params.not_after = now.checked_add(time::Duration::days(365 * 10)).unwrap();
    ca_cert_params.not_before = now;

    let key_pair = KeyPair::generate().unwrap();
    let ca_cert = ca_cert_params.self_signed(&key_pair).unwrap();

    fs::write("ca_cert.crt", ca_cert.pem()).unwrap();
    fs::write("ca_cert_secret.pem", key_pair.serialize_pem()).unwrap();
    fs::write("ca_cert_public.pem", key_pair.public_key_pem()).unwrap();

    (ca_cert, Issuer::new(ca_cert_params, key_pair))
}

fn create_server_cert(issuer: &Issuer<'_, KeyPair>) {
    let mut server_cert_params = CertificateParams::default();
    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, "Azo Test Server");
    dn.push(DnType::CountryName, "CN");
    dn.push(DnType::LocalityName, "Shanghai");
    dn.push(DnType::OrganizationName, "azazo");
    dn.push(DnType::OrganizationalUnitName, "common");
    dn.push(DnType::StateOrProvinceName, "Shanghai");
    server_cert_params.distinguished_name = dn;

    server_cert_params.is_ca = IsCa::ExplicitNoCa;
    server_cert_params.key_usages = [
        KeyUsagePurpose::KeyEncipherment,
        KeyUsagePurpose::DigitalSignature,
    ]
    .into();
    let now = OffsetDateTime::now_utc();
    server_cert_params.not_before = now;
    server_cert_params.not_after = now.checked_add(time::Duration::days(365 * 10)).unwrap();

    server_cert_params.subject_alt_names =
        [SanType::DnsName("localhost".try_into().unwrap())].into();
    let key_pair = KeyPair::generate().unwrap();
    let server_cert = server_cert_params.signed_by(&key_pair, issuer).unwrap();

    fs::write("server_cert.crt", server_cert.pem()).unwrap();
    fs::write("server_cert_secret.pem", key_pair.serialize_pem()).unwrap();
    fs::write("server_cert_public.pem", key_pair.public_key_pem()).unwrap();
}

fn main() {
    let (_cert, issuer) = create_ca();
    create_server_cert(&issuer);
}
