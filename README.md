# kubesecret

Secrets on Kubernetes are cumbersome to view with the vanilla tooling; this provides a way to quickly see the content of your secrets.


```shell
$ kubesecret hushhush
GENERAL: Kenobi
POSTGRES_CONNECTION_STRING: postgres://user:pass@example.com/database
$ kubesecret hushhush postgres connection string
postgres://user:pass@example.com/database
```

_given the following secret:_
```yaml
apiVersion: v1
kind: Secret
type: Opaque
metadata:
  name: hushhush
data:
  GENERAL: S2Vub2Jp
  POSTGRES_CONNECTION_STRING: cG9zdGdyZXM6Ly91c2VyOnBhc3NAZXhhbXBsZS5jb20vZGF0YWJhc2U=
```

## Installation

Grab the latest release from [the list](https://github.com/OutdatedVersion/kubesecret/releases/latest) and add it to your path.

## Wish List

- Support other [secret types](https://kubernetes.io/docs/concepts/configuration/secret/#secret-types)
- Support more than the default namespace
- Support a custom delimiter shortcut for the keyed argument
- Directly communicate with Kubernetes API

## Alternatives

- [k8sec](https://github.com/dtan4/k8sec)
- [ksec](https://github.com/mmailhos/ksec) 
