conf = configuration_data()
conf.set_quoted('PKGNAME', meson.project_name())
conf.set_quoted('VERSION', '@0@@1@'.format(meson.project_version(), version_suffix))
conf.set_quoted('APP_ID', app_id)
conf.set_quoted('PATH_ID', path_id)
conf.set_quoted('PROFILE', profile)
conf.set_quoted('PREFIX', get_option('prefix'))
conf.set_quoted('LOCALEDIR', get_option('localedir'))
conf.set_quoted('DATADIR', get_option('datadir'))

configure_file(
  input: 'config.rs.in',
  output: 'config.rs',
  configuration: conf
)

run_command(
  'cp',
  meson.project_build_root() / 'src' / 'config.rs',
  meson.project_source_root() / 'src' / 'config.rs',
  check: true
)

if host_machine.system() == 'windows'
  target_name = meson.project_name() + '.exe'
else
  target_name = meson.project_name()
endif

cargo_options = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_options += [ '--target-dir', meson.project_build_root() / 'src' ]

if profile == 'default'
  cargo_options += [ '--release' ]
  rust_target = 'release'
  message('Building in release mode')
else
  rust_target = 'debug'
  message('Building in debug mode')
endif

cargo_env = [ 'CARGO_HOME=' + meson.project_build_root() / 'cargo-home' ]

cargo_compile = custom_target(
  'cargo-compile',
  build_by_default: true,
  build_always_stale: true,
  output: 'compiledfile',
  console: true,
  env: cargo_env,
  command: [cargo, 'build', cargo_options]
)

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  output: target_name,
  depends: cargo_compile,
  install: true,
  install_dir: bindir,
  command: [
    'cp', 
    meson.project_build_root() / 'src' / rust_target / target_name,
    '@OUTPUT@',
  ]
)
