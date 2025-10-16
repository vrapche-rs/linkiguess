#!/bin/sh

if [ ! -e "linkiguess-ca.crt" ] || [ ! -e "linkiguess-ca.key" ]; then
    # Create config for the leaf certificate
    cat > linkiguess.cnf << EOF
[req]
distinguished_name = req_distinguished_name
req_extensions = v3_req
prompt = no

[req_distinguished_name]
CN = linkiguess.localhost
O = linkiguess

[v3_req]
keyUsage = digitalSignature, keyEncipherment
extendedKeyUsage = serverAuth
subjectAltName = @alt_names
basicConstraints = CA:FALSE

[alt_names]
DNS.1 = linkiguess.localhost
DNS.2 = short.linkiguess.localhost
DNS.3 = traefik.admin.linkiguess.localhost
DNS.4 = grafana.admin.linkiguess.localhost
DNS.5 = elk.admin.linkiguess.localhost
IP.1 = 127.0.0.1
IP.2 = ::1
EOF

    # Create a separate config for the CA
    cat > ca.cnf << EOF
[req]
distinguished_name = req_distinguished_name
prompt = no

[req_distinguished_name]
CN = linkiguess CA
O = linkiguess

[v3_ca]
keyUsage = digitalSignature, keyCertSign, cRLSign
basicConstraints = CA:TRUE
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid:always,issuer
EOF

    # Generate CA with proper CA extensions
    openssl req -x509 -newkey rsa:4096 -sha256 -days 3650 -nodes \
      -keyout linkiguess-ca.key -out linkiguess-ca.crt \
      -subj "/CN=linkiguess CA/O=linkiguess" \
      -extensions v3_ca -config ca.cnf

    # Generate leaf certificate key and CSR
    openssl req -newkey rsa:2048 -nodes -keyout linkiguess.key \
        -out linkiguess.csr \
        -config linkiguess.cnf

    # Sign the leaf certificate WITH extensions
    openssl x509 -req -in linkiguess.csr -CA linkiguess-ca.crt -CAkey linkiguess-ca.key -CAcreateserial \
        -out linkiguess.crt -days 365 -sha256 \
        -extfile linkiguess.cnf -extensions v3_req

    echo "CA file has been generated, please trust it in your system"

    # Clean up
    rm -f ca.cnf linkiguess.cnf linkiguess.csr
fi

if [ ! -e "env/certs/linkiguess.crt" ]; then
    cp linkiguess.crt env/certs/linkiguess.crt
fi

if [ ! -e "env/certs/linkiguess.key" ]; then
    cp linkiguess.key env/certs/linkiguess.key
fi
