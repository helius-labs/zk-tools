FROM sha256:a24195e830e9becaa887ece0898c7c07b628edba3f365f439988e885c78bd1b5
COPY cr-previous cr-current aws-runner cr-xtask /usr/local/bin/
COPY basis/ /cr-basis/
COPY manifest.json /cr/manifest.json
ENTRYPOINT ["/usr/local/bin/aws-runner"]
