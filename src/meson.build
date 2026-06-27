cargo_env = [
  'CARGO_HOME=' + meson.project_build_root() / 'cargo-home',
  'PKGNAME=' + meson.project_name(),
  'VERSION=' + '@0@@1@'.format(meson.project_version(), version_suffix),
  'APP_ID=' + app_id,
  'PATH_ID=' + path_id,
  'PROFILE=' + profile,
  'PREFIX=' + get_option('prefix'),
  'LOCALEDIR=' + get_option('localedir'),
  'DATADIR=' + get_option('datadir')
]

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
