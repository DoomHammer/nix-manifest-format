use super::format_manifest;


#[test]
fn check_single_entry()
{
    let input = "[ { meta = { sdfjdslfkdsjflkdsjfldskjflkdsjf }; } ]";
    let output = "[\n  {\n    meta = {};\n  }\n]\n";

    assert_eq!(format_manifest(String::from(input)), String::from(output));
}

#[test]
fn order_packages()
{
    let input = "[ { name = b; prop = value; } {name = c;  prop = value; } { name = a; prop = value; } ]";
    let output = r###"[
  {
    name = a;
    prop = value;
  }
  {
    name = b;
    prop = value;
  }
  {
    name = c;
    prop = value;
  }
]
"###;

    assert_eq!(format_manifest(String::from(input)), String::from(output));
}

#[test]
fn check_many_entries()
{
    let input = "[ { a = ble; c = ble; d = ble; e = ble; meta = { ble }; } ]";
    let output = r###"[
  {
    a = ble;
    c = ble;
    d = ble;
    e = ble;
    meta = {};
  }
]
"###;

    assert_eq!(format_manifest(String::from(input)), String::from(output));
}

#[test]
fn check_production_data()
{
    let input = r###"[ { meta = { available = true; branch = "1.12"; description = "The Go Programming language"; homepage = "http://golang.org/"; license = { fullName = "BSD 3-clause \"New\" or \"Revised\" License"; shortName = "bsd3"; spdxId = "BSD-3-Clause"; url = "http://spdx.org/licenses/BSD-3-Clause.html"; }; maintainers = [ { email = "charles@cstrahan.com"; github = "cstrahan"; name = "Charles Strahan"; } { email = "orivej@gmx.fr"; github = "orivej"; name = "Orivej Desh"; } { email = "xaviosx@gmail.com"; github = "velovix"; name = "Tyler Compton"; } { email = "joerg@thalheim.io"; github = "mic92"; keys = [ { fingerprint = "3DEE 1C55 6E1C 3DC5 54F5  875A 003F 2096 411B 5F92"; longkeyid = "rsa4096/0x003F2096411B5F92"; } ]; name = "Jörg Thalheim"; } ]; name = "go-1.12.1"; outputsToInstall = [ "out" ]; platforms = [ { kernel = { _type = "kernel"; execFormat = { _type = "exec-format"; name = "elf"; }; families = { }; name = "linux"; }; } { kernel = { families = { darwin = { _type = "exec-format"; name = "darwin"; }; }; }; } ]; position = "/nix/store/7zb574bzv3n17h4pp8vilad7mm6cfjqr-nixpkgs-19.09pre180124.920d066ded1/nixpkgs/pkgs/development/compilers/go/1.12.nix:234"; }; name = "go-1.12.1"; out = { outPath = "/nix/store/749qksf79hvn0aprcznd9bwfv550qwh3-go-1.12.1"; }; outPath = "/nix/store/749qksf79hvn0aprcznd9bwfv550qwh3-go-1.12.1"; outputs = [ "out" ]; system = "x86_64-linux"; type = "derivation"; } ]"###;
    let output = r###"[
  {
    meta = {};
    name = "go-1.12.1";
    out = { outPath = "/nix/store/749qksf79hvn0aprcznd9bwfv550qwh3-go-1.12.1"; };
    outPath = "/nix/store/749qksf79hvn0aprcznd9bwfv550qwh3-go-1.12.1";
    outputs = [ "out" ];
    system = "x86_64-linux";
    type = "derivation";
  }
]
"###;

    assert_eq!(format_manifest(String::from(input)), String::from(output));
}
