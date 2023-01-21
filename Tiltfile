include("tilt/local_jaeger.tilt")

k8s_yaml(helm('helm'))
k8s_resource('axum-tower')
docker_build('axum-tower', '.')

k8s_resource(
  workload='axum-tower',
  port_forwards='9000:8080'
)
