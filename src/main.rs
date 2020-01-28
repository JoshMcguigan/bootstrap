use alchemist::{
    ingredients::{
        cargo_crate, git, gsettings, link, mkdir, package, prompt, ssh_keygen, LinkType, 
        SshKeyType, rustup_component, line_in_file
    },
    Recipe, ServerConfig,
};

fn main() {
    let localhost = ServerConfig::Local;

    // TODO think about how paths should be normalized
    Recipe::new("Provision localhost", vec![localhost]).run(vec![
        package("vim-enhanced"),
        package("make"),
        package("ripgrep"),
        gsettings(
            "org.gnome.desktop.input-sources",
            "xkb-options",
            "['ctrl:nocaps']",
        ),
        gsettings("org.gnome.desktop.interface", "text-scaling-factor", "1.5"),
        ssh_keygen(SshKeyType::Ed25519)
            .if_modified(prompt("Manually add public ssh key to GitHub")),
        rustup_component("rustfmt"),
        rustup_component("clippy"),
        rustup_component("rls"),
        rustup_component("rust-analysis"),
        rustup_component("rust-src"),
        mkdir("~/.cargo"),
        link("~/workspace/bootstrap/.cargo/config", "~/.cargo/config",
            LinkType::Symbolic),
        link("~/workspace/bootstrap/.gitignore_global", "~/.gitignore_global",
            LinkType::Symbolic),
        cargo_crate("watchexec"),
        package("nodejs"),
        git("https://github.com/JoshMcguigan/dotvim.git", "~/.vim"),
        cargo_crate("pista"),
        line_in_file("source ~/workspace/bootstrap/pista-bashrc", "~/.bashrc"),
        line_in_file("source ~/workspace/bootstrap/fzf-bashrc", "~/.bashrc"),
        prompt(
            "Manually set hostname with \
             'hostnamectl set-hostname $DESIRED_HOSTNAME'",
        ),
        prompt("Manually install uBlock Origin plugin for Firefox"),
        prompt("Manually configure the terminal theme to light mode"),
        prompt("Manually configure the terminal to disable the bell"),
        prompt("Manually run :CocInstall coc-rls within vim"),
    ]);
}
