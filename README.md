## Health Check For Distroless Images
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
  build: amin-mir/distroless-hc
  environment:
    HOSTS: >
      http://server1:3030/healthcheck,
      http://server2:3031/healthcheck,
      http://server3:3032/healthcheck
    RETRIES: 4
  depends_on:
    server1:
      condition: service_healthy
```

Simply list the url of the services you need up and running before dependent services under HOSTS environment variable.
Then in your top-level services you can wait until `wait-services` container exits successfully:

```yml
depends_on:
  server1:
    condition: service_healthy
```

