# Config

This module writes a config into k8sgpt-dev.
This file is used to control the actions performed by the k8sgpt-dev on the repositories checked out.

e.g.
```
repository:
- name: k8sgpt
  command:
   start: go run main.go serve
- name: k8sgpt-operator
  env: LOCAL_HOST=true
  command:
   start: go run main.go
```