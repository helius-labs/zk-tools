FROM 558215002830.dkr.ecr.eu-north-1.amazonaws.com/zolana-prover:bench-cr-c7a-20261027-081750
COPY cr-current pm-runner /usr/local/bin/
COPY manifest.json /cr/manifest.json
ENTRYPOINT ["/usr/local/bin/pm-runner"]
