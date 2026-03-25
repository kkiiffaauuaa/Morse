project(
  'morse', 'rust', 
  license: 'GPLv3',
  version: '1.1.0',
  meson_version: '>= 1.0.0',
  default_options: [ 'warning_level=2', 'werror=false', ],
)

i18n = import('i18n')
gnome = import('gnome')
fs = import('fs')
cargo = find_program('cargo', required: true)
gettext_package = meson.project_name()

base_id = 'io.github.teacond.Morse'
path_id = '/io/github/teacond/Morse'
app_id  = base_id
vcs_tag = run_command('git', 'rev-parse', '--short', 'HEAD', check: false).stdout().strip()
profile = get_option('profile')
datadir = join_paths(get_option('prefix'), get_option('datadir'))
localedir = join_paths(get_option('prefix'), get_option('localedir'))
bindir = join_paths(get_option('prefix'), get_option('bindir'))

dependency('gtk4', version: '>= 4.22')
dependency('libadwaita-1', version: '>= 1.9')

cargo_sources = files(
  'Cargo.toml',
  'Cargo.lock',
)

if profile == 'development'
  if fs.is_dir('.git')
    vcs_tag = run_command('git', 'rev-parse', '--short', 'HEAD', check: true).stdout().strip()
    if vcs_tag == ''
      version_suffix = '-devel'
    else
      version_suffix = '-@0@'.format(vcs_tag)
    endif
  else
    version_suffix = '-devel'
  endif
  app_id = '@0@.@1@'.format(base_id, 'Devel')
else
  version_suffix = ''
  app_id = base_id
endif

subdir('data')
subdir('src')
subdir('po')

meson.add_dist_script(
  'build-aux/dist-vendor.sh',
  meson.project_source_root(),
  meson.project_build_root() / 'meson-dist' / '@0@-@1@'.format(meson.project_name(), meson.project_version()),
)

summary({
    'prefix': get_option('prefix'),
    'libdir': get_option('libdir'),
    'datadir': get_option('datadir'),
    'bindir': get_option('bindir'),
  },
  section: 'Directories',
)

summary({
    'Profile': get_option('profile'),
  },
  section: 'Build options',
)
