FROM 558215002830.dkr.ecr.eu-north-1.amazonaws.com/zolana-prover:bench-latest-c7a-profile-20261025
COPY lb-accepted lb-candidate lb-adapter lb-runner lb-xtask gnark-verify /usr/local/bin/
COPY zopro-keys.lock /manifest/zopro-keys.lock
COPY basis /lb-basis
COPY manifest.json /lb/manifest.json
ENTRYPOINT ["/usr/local/bin/lb-runner"]
