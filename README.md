# Dottie

This is my dot files, some of them are modified from others.

## Packages/Dependencies

- A [Nerd Font](https://github.com/ryanoasis/nerd-fonts)
- [Z-Shell](https://www.zsh.org/) with [oh-my-zsh](https://ohmyz.sh/) and [zsh-completions](https://github.com/zsh-users/zsh-completions)
- [Starship](https://starship.rs/) shell prompt
- [Tmux](https://github.com/tmux/tmux/wiki) terminal multiplexer
- [GitUI](https://github.com/extrawurst/gitui) git terminal user interface

## Install

**WARNING**: Backup your dot files before installing.

```sh
# change-dir into this repo
cd dottie
# hard link everything into their position
ln ./* ~/
```

## References

- `.tmux.conf`
  - Theming config learned from: [tony/tmux-config](https://github.com/tony/tmux-config/blob/c1d6a4c6781221462376f07c78ec6d9cd4e949a3/.tmux.conf)
  - Theme color picked from: [dracula/tmux](https://github.com/dracula/tmux/blob/c2b1d67cbda5c44ea8ee25d2ab307063e6959d0f/screenshot.png)
- `.config/starship.toml`
  - Based on: [nerd-font preset](https://starship.rs/presets/nerd-font)
- `.config/gitui/key_bindings.ron`
  - Copied from: [extrawurst/gitui](https://github.com/extrawurst/gitui/blob/64a1e3866ea6f1b63f4f308972774622c57896b8/KEY_CONFIG.md)
