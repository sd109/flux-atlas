# This script creates a local kind cluster and installs
# the flux-atlas components on it

set -ex

kind create cluster -n flux-atlas-testing

flux install

kubectl apply -f ../manifests

kubectl apply -f ../../ui/tests/manifests