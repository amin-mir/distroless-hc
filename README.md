## Health Check For Distroless Images

I've also published a story on Medium explaining the motivation for creating this image which you can have a look [here](https://medium.com/@aminmir326/health-checks-for-distroless-containers-a2180c4c4fcf).

This project provides a lightweight solution for health checking Docker containers built from distroless base images or images lacking essential utilities
like `wget` and `curl`. 

If you try to do a typical health check for a distroless container as shown below:

```yml
healthcheck:
  test: ["CMD-SHELL", "curl -f http://localhost:80/ || exit 1"]
  interval: 30s
  timeout: 10s
  retries: 3
```

You will get the following error when you inspect the container logs (`docker container inspect --format='{{json .State}}' <CONTAINER_ID> | jq`)

```
"Health": {
  "Status": "unhealthy",
  "FailingStreak": 8,
  "Log": [
    {
      "Start": "2023-12-20T19:48:31.029268007Z",
      "End": "2023-12-20T19:48:31.048005757Z",
      "ExitCode": -1,
      "Output": "OCI runtime exec failed: exec failed: unable to start container process: exec: \"/bin/sh\": stat /bin/sh: no such file or directory: unknown"
    },
    // ...
    {
      "Start": "2023-12-20T19:50:31.127219507Z",
      "End": "2023-12-20T19:50:31.150400091Z",
      "ExitCode": -1,
      "Output": "OCI runtime exec failed: exec failed: unable to start container process: exec: \"/bin/sh\": stat /bin/sh: no such file or directory: unknown"
    }
  ]
}
```

Distroless base images are known for their minimalism, making them secure and lightweight, but they can pose challenges when implementing health checks or debugging. 
With `distroless-hc`, you can eliminate the flakiness in your tests caused by the startup times of services within your Docker Compose setup. 

To solve this, you can use distroless-hc image and add a `wait-services` container in your docker-compose.yml file like shown below:

```
wait-services:
  build: malooooch/distroless-hc
  environment:
    HOSTS: >
      http://server1:3030/healthcheck,
      http://server2:3031/healthcheck,
      http://server3:3032/healthcheck
    RETRIES: 4
  depends_on:
    - server1
    - server2
    - server3
```

Simply list the url of the services you need up and running before dependent services under HOSTS environment variable.
Then in your top-level services you can wait until `wait-services` container exits successfully:

```yml
depends_on:
  wait-services:
    condition: service_completed_successfully
```

### Configuration
You can configure the health checker with the following environment variables:

* `HOSTS`: A comma-separated list of URLs.
* `TIMEOUT`: The timeout duration for each HTTP GET request e.g. `500ms`, `1s`.
* `RETRIES`: The maximum number of HTTP requests sent to each URL before it is marked as unhealthy.
* `INTERVAL`: The time interval in milliseconds between consecutive HTTP requests to each URL e.g. `2s`.

### Image Link
Check out the image on [dockerhub](https://hub.docker.com/r/malooooch/distroless-hc).

```
docker pull malooooch/distroless-hc
```
