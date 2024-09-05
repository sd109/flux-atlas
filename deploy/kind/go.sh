# This script creates a local kind cluster with an Nginx Ingress controller
# and then installs the flux-atlas components on the cluster.
# Once deployment is complete the flux-atlas UI will be available on
# localhost (port 80) on the host machine (i.e. outside the kind cluster).

set -ex

stat ./cluster-config.yml > /dev/null

# Based on https://kind.sigs.k8s.io/docs/user/ingress/
kind create cluster --config ./cluster-config.yml -n flux-atlas-testing

kubectl apply --wait -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/kind/deploy.yaml

sleep 5

kubectl wait --namespace ingress-nginx \
  --for=condition=ready pod \
  --selector=app.kubernetes.io/component=controller \
  --timeout=90s

flux install

kubectl apply -f ../manifests

# kubectl apply -f ../../ui/tests/manifests