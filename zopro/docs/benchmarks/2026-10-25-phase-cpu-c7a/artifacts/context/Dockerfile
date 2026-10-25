FROM 558215002830.dkr.ecr.eu-north-1.amazonaws.com/zolana-prover:bench-latest-c7a-profile-20261025
COPY . /experiment/
RUN chmod 755 /experiment/run.sh /experiment/zopro-accepted /experiment/zopro-diagnostic /experiment/zopro-xtask /experiment/serve-results
ENTRYPOINT ["/experiment/run.sh"]
