#!/bin/bash
set -euo pipefail
exec > /var/log/zopro-phase-cpu-bootstrap.log 2>&1
systemd-run --unit=zopro-phase-cpu-ttl --on-active=90m /sbin/shutdown -h now
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq
apt-get install -y -qq docker.io curl unzip
curl --fail --silent --show-error --location https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip -o /var/tmp/zopro-phase-cpu-awscli.zip
unzip -q /var/tmp/zopro-phase-cpu-awscli.zip -d /var/tmp/zopro-phase-cpu-awscli
/var/tmp/zopro-phase-cpu-awscli/aws/install
systemctl enable --now docker
systemctl enable --now snap.amazon-ssm-agent.amazon-ssm-agent.service || snap start amazon-ssm-agent
mkdir -p /var/tmp/zopro-phase-cpu-c7a
uname -a > /var/tmp/zopro-phase-cpu-c7a/hardware.txt
lscpu >> /var/tmp/zopro-phase-cpu-c7a/hardware.txt
