FROM 558215002830.dkr.ecr.eu-north-1.amazonaws.com/zolana-prover:bench-latest-c7a-profile-20261025
COPY gs-accepted gs-candidate gs-adapter gs-runner gs-xtask gnark-verify /usr/local/bin/
COPY zopro-keys.lock /manifest/zopro-keys.lock
COPY basis /gs-basis
COPY manifest.json /gs/manifest.json
ENTRYPOINT ["/usr/local/bin/gs-runner"]
