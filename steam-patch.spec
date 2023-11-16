Name:           steam-patch
Version:        {{{ git_dir_version }}}
Release:        1%{?dist}
Summary:        Steam Patch for TDP and GPU clock control

License:        GPL3
URL:            https://github.com/KyleGospo/steam-patch

VCS:            {{{ git_dir_vcs }}}
Source:         {{{ git_dir_pack }}}

BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  systemd-rpm-macros

%description
Steam Patch for ASUS ROG ALLY

%prep
{{{ git_dir_setup_macro }}}

%build
cargo build -r

%install
mkdir -p %{buildroot}/%{_bindir}
cp %{_builddir}/steam-patch/target/release/steam-patch %{buildroot}/%{_bindir}/steam-patch

mkdir -p %{buildroot}/%{_unitdir}

install -m 644 steam-patch@.service %{buildroot}/%{_unitdir}/steam-patch@.service
install -m 644 restart-steam-patch-on-boot.service %{buildroot}/%{_unitdir}/restart-steam-patch-on-boot.service

%post
%systemd_post restart-steam-patch-on-boot.service

%preun
%systemd_preun restart-steam-patch-on-boot.service

%postun
%systemd_postun_with_restart restart-steam-patch-on-boot.service

%files
%{_bindir}/steam-patch
%{_unitdir}/steam-patch@.service
%{_unitdir}/restart-steam-patch-on-boot.service

%changelog
{{{ git_dir_changelog }}}
