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
- Database: PostgreSQL
- ORM: diesel

# usage

```bash
# create k8s cluster
$ kind create cluster
# deploy infra components (like Dapr and DBs) to the cluster
$ skaffold run -f skaffold.infra.yaml
# deploy apps to the cluster
$ skaffold run

# setup tables
$ kubectl port-forward svc/postgresql 5432:5432
Forwarding from 127.0.0.1:5432 -> 5432
Forwarding from [::1]:5432 -> 5432
$ DATABASE_URL="postgres://wallet:walletpass@localhost/postgres" diesel migration run
Running migration 2021-11-24-051017_create_wallets

# request gRPC to create new wallet
$ kubectl port-forward svc/floodplain-dapr 50001:50001
Forwarding from 127.0.0.1:50001 -> 50001
Forwarding from [::1]:50001 -> 50001
$ grpcurl -plaintext -emit-defaults -rpc-header 'dapr-app-id: floodplain' -proto proto/wallet/service.proto localhost:50001 wallet.WalletService/Create
{
  "wallet": {
    "id": "wallet-01FRTD44X460BB22D65S49CPK0",
    "deposit": "0",
    "currency": "JPY"
  }
}
```
