# SPECIFICATION of RSP(Raw String Peeler)

## Motivation


```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: example-config
data:
  raw-yaml-string.yaml: "\"hello\": \"test\"\n\"foo\"\": 22\n"
  \ 
```

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: example-config
data:
  raw-toml-string.toml: "hello = \"test\"\n foo 
  \ = \"bar\"\n"
```

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: example-config
data:
  raw-json-string.json: "{\n
  \  \"hello\":\"test\",\n  \"foo\":\"bar\"\n
  }
```

Those inner yaml/toml/json is not good for human readers. So RSP is meant to be created to peel those inner file out of yaml. So the expected output would be like this.


```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: example-config
data:
  raw-yaml-string.yaml: |
    hello: test
    foo: bar
```

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: example-config
data:
  raw-toml-string.toml: |
    hello = "test"
    foo = "bar"
```

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: example-config
data:
  raw-json-string.json: |
    {
      "hello": "test",
      "foo": "bar"
    }
```


## APIs

RSP also provide nicer CLI based api

```sh
rsp peel test-configmap.yaml
```

Here is the common sub commands and the options

```
--help: show help of this cli

--version: show version
```

## Directory structure
Follow Rust CLI tool best practice.

## Testing

Please create some test cases including both normal input and unexpected input and place the code under ./tests

## CICD

Use github Actions, and do those kind of jobs to sustain the quality product. You only need to run those feature in ubuntu-latest for the time being. 

- build
- lint (including rustfmt, clippy)
- tests
- audit

## Development style

Firstly create a branch and the PR for each fix or feature enhancement. Then you can't merge this till human review approve it. You can also create an issue if you find a bug or points to be refactored.

## License

MIT
