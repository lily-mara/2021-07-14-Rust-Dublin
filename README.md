# Observing Rust Services

## Running the demo

Dependencies:

- docker
- docker-compose

First start up the services:

```
$ ./run
```

This will print a bunch of output and probably spin up all of your CPUs while
the services compile. Once services are running, use this cURL command to test
it out:

```
$ curl --request POST \
  --url http://localhost:8080/calculate \
  --header 'Content-Type: application/json' \
  --data '"3 4 + 2 - 3 * 3 /"'
```

You can then navigate to the Jaeger web UI at this address:
[localhost:16686](http://localhost:16686).
