#!/bin/bash
set -eux
exec > /var/log/zopro-bench-bootstrap.log 2>&1
systemd-run --unit=zopro-bench-ttl --on-active=90m /sbin/shutdown -h now
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq
apt-get install -y -qq docker.io curl unzip
curl -fsSL https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip -o /var/tmp/awscliv2.zip
unzip -q /var/tmp/awscliv2.zip -d /var/tmp/awscli-install
/var/tmp/awscli-install/aws/install
rm -rf /var/tmp/awscli-install /var/tmp/awscliv2.zip
systemctl enable --now docker
systemctl enable --now snap.amazon-ssm-agent.amazon-ssm-agent.service || snap start amazon-ssm-agent
mkdir -p /var/tmp/zopro-cr-c7a-20261027-081750
uname -a > /var/tmp/zopro-cr-c7a-20261027-081750/hardware.txt
lscpu >> /var/tmp/zopro-cr-c7a-20261027-081750/hardware.txt
