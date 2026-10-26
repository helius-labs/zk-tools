FROM 558215002830.dkr.ecr.eu-north-1.amazonaws.com/zolana-prover@sha256:bd04f1c9512b73de409578a9f42d0b2eb420f564e386ead60fee49984635e255
COPY . /experiment/
RUN chmod 755 /experiment/run.sh /experiment/zopro-accepted /experiment/zopro-candidate /experiment/zopro-xtask /experiment/review /experiment/serve-results
ENTRYPOINT ["/experiment/run.sh"]
