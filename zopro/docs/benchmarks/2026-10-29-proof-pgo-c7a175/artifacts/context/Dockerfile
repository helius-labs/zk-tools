FROM 558215002830.dkr.ecr.eu-north-1.amazonaws.com/zolana-prover:bench-pmu-c7a-20261027-123202
COPY cr-previous cr-current aws-runner cr-xtask /usr/local/bin/
COPY basis/ /cr-basis/
COPY manifest.json /cr/manifest.json
ENTRYPOINT ["/usr/local/bin/aws-runner"]
COPY preflight-complete.json /cr/preflight-complete.json
