FROM 558215002830.dkr.ecr.eu-north-1.amazonaws.com/zolana-prover:bench-latest-c7a-profile-20261025
COPY ready-accepted ready-diagnostic ready-adapter ready-runner ready-xtask gnark-verify /usr/local/bin/
COPY zopro-keys.lock /manifest/zopro-keys.lock
COPY basis /ready-basis
COPY manifest.json /ready/manifest.json
ENTRYPOINT ["/usr/local/bin/ready-runner"]
