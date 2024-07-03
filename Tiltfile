docker_build('flux-atlas', '.', build_args={'node_env': 'development'})

k8s_yaml(listdir('deploy/manifests'))

k8s_resource('flux-atlas', port_forwards=3000)
