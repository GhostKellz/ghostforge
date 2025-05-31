pub fn run() {
    println!("AUR submission is not supported directly. To submit, use:");
    println!("  git clone ssh://aur@aur.archlinux.org/<your-pkg>.git");
    println!("  cp PKGBUILD <your-pkg>/");
    println!("  cd <your-pkg> && git add PKGBUILD && git commit -m 'update' && git push");
}