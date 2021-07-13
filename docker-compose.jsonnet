local rustMicroservice(service) = {
  image: 'rust:1.53',
  entrypoint: '/code/entrypoint',
  working_dir: '/code/%s' % service.name,
  volumes: [
    './:/code',
    '%s_target:/cargo-target' % service.name,
    '%s_home:/cargo-home' % service.name,
  ],
  ports: [
    '%s:80' % service.hostPort,
  ],
  environment: {
    CARGO_TARGET_DIR: '/cargo-target',
    CARGO_HOME: '/cargo-home',
    RUST_LOG: 'info',
    OTEL_EXPORTER_JAEGER_AGENT_HOST: 'jaeger',
    OTEL_EXPORTER_JAEGER_AGENT_PORT: '6831',
    OTEL_EXPORTER_SERVICE_NAME: service.name,
  },
};

local services = [
  { name: 'calculator', hostPort: '8080' },
  { name: 'add', hostPort: '8081' },
  { name: 'sub', hostPort: '8082' },
  { name: 'mul', hostPort: '8083' },
  { name: 'div', hostPort: '8084' },
];

local serviceNames = [i.name for i in services];

local serviceContainers = {
  [i.name]: rustMicroservice(i)
  for i in services
};

{
  version: '3.1',
  services: {
    jaeger: {
      image: 'jaegertracing/all-in-one:1.23',
      ports: [
        '5778:5778',
        '16686:16686',
      ],
    },
  } + serviceContainers,
  volumes: {
    ['%s_target' % i]: {}
    for i in serviceNames
  } + {
    ['%s_home' % i]: {}
    for i in serviceNames
  },
}
