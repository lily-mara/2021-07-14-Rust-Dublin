local rustMicroservice(name) = {
  image: 'rust:1.53',
  entrypoint: '/code/entrypoint',
  working_dir: '/code/%s' % name,
  volumes: [
    './:/code',
    '%s_target:/cargo-target' % name,
    '%s_home:/cargo-home' % name,
  ],
  environment: {
    CARGO_TARGET_DIR: '/cargo-target',
    CARGO_HOME: '/cargo-home',
    OTEL_EXPORTER_JAEGER_AGENT_HOST: 'jaeger',
    OTEL_EXPORTER_JAEGER_AGENT_PORT: '6831',
  },
};

local serviceNames = ['calculator', 'add', 'sub', 'mul', 'div'];

local serviceContainers = {
  [i]: rustMicroservice(i)
  for i in serviceNames
} + { calculator+: { ports: ['8080:80'] } };

{
  version: '3.1',
  services: {
    jaeger: {
      image: 'jaegertracing/all-in-one:1.23',
      ports: [
        '6831:6831/udp',
        '6832:6832/udp',
        '5778:5778',
        '16686:16686',
        '14268:14268',
        '14250:14250',
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
