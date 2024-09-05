# This script creates a local kind cluster and installs
# the flux-atlas components on it

set -ex

stat kind.sh > /dev/null

kind create cluster -n flux-atlas-testing

flux install

kubectl apply -f ./manifests

kubectl apply -f ../ui/tests/manifests