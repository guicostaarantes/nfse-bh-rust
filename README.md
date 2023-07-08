# NFSE BH RUST

Aplicação para automatizar a emissão de notas fiscais de serviço eletrônica pela prefeitura de Belo Horizonte MG.

## COMO USAR

Instale o OpenSSL.

Supondo que seu certificado digital esteja no diretório e seja nomeado certificado.pfx, execute os seguintes comandos:

`openssl pkcs12 -in certificado.pfx -out certificado.pem -nodes -legacy`

`openssl pkcs12 -in certificado.pfx -clcerts -nokeys -legacy | openssl x509 -out certificado.cer`

`openssl pkcs12 -in certificado.pfx -nocerts -nodes -legacy | openssl pkcs8 -nocrypt -out certificado.key`

Você terá que digitar a senha do certificado cada vez que rodar um desses comandos. Serão criados os arquivos cerificado.pem, certificado.cer e certificado.key. Esses arquivos não são criptografados como o .pfx, portanto seja cuidadoso com eles e não os exponha a agentes maliciosos.

Copie o arquivo example.yml e salve a cópia como input.yml. Preencha os campos com seus dados.

Para rodar o programa, baixe a linguagem Rust e execute `cargo run`. Se preferir, compile o programa usando `cargo build` e assim poderá usá-lo sem ter a linguagem Rust instalada.
