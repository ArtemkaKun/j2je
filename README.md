# Tool that converts JSON into a `jsonencode` Terraform HCL code.

## Conversion example

### Input

```json
{"datasource":{"type":"prometheus","uid":"${grafana_data_source.prometheus_datasource_global.uid}"},"editorMode":"code","expr":"sum by (node) (aksnth_spot_node_eviction_imminent)","instant":false,"intervalMs":1000,"maxDataPoints":43200,"range":true,"refId":"A"}
```

### Output

```hcl
jsonencode({
    "datasource" = {
      "type" = "prometheus"
      "uid"  = grafana_data_source.prometheus_datasource_global.uid
    }
    "editorMode"    = "code"
    "expr"          = "sum by (node) (aksnth_spot_node_eviction_imminent)"
    "instant"       = false
    "intervalMs"    = 1000
    "maxDataPoints" = 43200
    "range"         = true
    "refId"         = "A"
})
```

## How to run

1. Install Rust.
2. Clone this repo and run `cargo run` in the cloned directory.