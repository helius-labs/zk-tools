FROM 558215002830.dkr.ecr.eu-north-1.amazonaws.com/zolana-prover:bench-latest-c7a-profile-20261025
COPY cr-previous cr-current cr-adapter cr-runner cr-xtask gnark-verify /usr/local/bin/
COPY zopro-keys.lock /manifest/zopro-keys.lock
COPY basis /cr-basis
COPY manifest.json /cr/manifest.json
ENTRYPOINT ["/usr/local/bin/cr-runner"]
