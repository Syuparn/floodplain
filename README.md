# floodplain
sample Rust wallet/bank app for practice

# requirements

- [Kind](https://kind.sigs.k8s.io/)
- [Skaffold](https://skaffold.dev/)

# dependencies

- Language: Rust
- gRPC Framework: tonic
- Build/Deploy images: Skaffold
- Infrastructure: Kubernetes
- Application Runtime: Dapr

# usage

```bash
# create k8s cluster
$ kind create cluster
# deploy infra components (like Dapr) to the cluster
$ skaffold run -f skaffold.infra.yaml
# deploy apps to the cluster
$ skaffold run

# TODO: impl real wallet/bank app
$ kubectl port-forward svc/floodplain 50051:50051
Forwarding from 127.0.0.1:50051 -> 50001
Forwarding from [::1]:50051 -> 50001

$ $ grpcurl -plaintext -rpc-header 'dapr-app-id: floodplain' -proto proto/wallet/service.proto  localhost:50051 wallet.WalletService/Create
{
}
```
