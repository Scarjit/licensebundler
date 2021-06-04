use chrono::{DateTime, Datelike, Utc};

static MARKDOWN_0BSD: &str = include_str!("./markdown/0bsd.md");
static MARKDOWN_APACHE_V20: &str = include_str!("./markdown/apache-v2.0.md");
static MARKDOWN_APACHE_V20_LLVM: &str = include_str!("./markdown/apache-v2.0-llvm.md");
static MARKDOWN_ARTISTIC_V20: &str = include_str!("./markdown/artistic-v2.0.md");
static MARKDOWN_BSD_2: &str = include_str!("./markdown/bsd-2.md");
static MARKDOWN_BSD_3: &str = include_str!("./markdown/bsd-3.md");
static MARKDOWN_EPL_V10: &str = include_str!("./markdown/epl-v1.0.md");
static MARKDOWN_GNU_AGPL_V30: &str = include_str!("./markdown/gnu-agpl-v3.0.md");
static MARKDOWN_GNU_FDL_V13: &str = include_str!("./markdown/gnu-fdl-v1.3.md");
static MARKDOWN_GNU_GPL_V10: &str = include_str!("./markdown/gnu-gpl-v1.0.md");
static MARKDOWN_GNU_GPL_V20: &str = include_str!("./markdown/gnu-gpl-v2.0.md");
static MARKDOWN_GNU_GPL_V30: &str = include_str!("./markdown/gnu-gpl-v3.0.md");
static MARKDOWN_GNU_LGPL_V21: &str = include_str!("./markdown/gnu-lgpl-v2.1.md");
static MARKDOWN_GNU_LGPL_V30: &str = include_str!("./markdown/gnu-lgpl-v3.0.md");
static MARKDOWN_MIT: &str = include_str!("./markdown/mit.md");
static MARKDOWN_MPL_V20: &str = include_str!("./markdown/mpl-v2.0.md");
static MARKDOWN_UNLICENSE: &str = include_str!("./markdown/unlicense.md");
static MARKDOWN_ZLIB: &str = include_str!("./markdown/zlib.md");
static MARKDOWN_ISC: &str = include_str!("./markdown/isc.md");
static MARKDOWN_BSL_10: &str = include_str!("./markdown/bsl-1.0.md");

pub fn lookup_md(license_name: &str, authors: &Option<String>) -> Vec<String> {
    let license_name = license_name.to_lowercase();
    //log::info!("{}", license_name);
    let now: DateTime<Utc> = Utc::now();
    let current_year = now.date().year();

    let licenses = license_name
        .split(" or ")
        .flat_map(|s| s.split(" and "))
        .collect::<Vec<&str>>();

    let mut md_licenses = vec![];
    for license in licenses {
        let ltext = match license {
            "0bsd" => MARKDOWN_0BSD,
            "apache-v2.0" | "apache-2.0" => MARKDOWN_APACHE_V20,
            "apache-v2.0-llvm" | "apache-2.0-llvm" | "apache-2.0 with llvm-exception" => {
                MARKDOWN_APACHE_V20_LLVM
            }
            "artistic-v2.0" => MARKDOWN_ARTISTIC_V20,
            "bsd-2" | "bsd-2-clause" => MARKDOWN_BSD_2,
            "bsd-3" | "bsd-3-clause" => MARKDOWN_BSD_3,
            "epl-v1.0" => MARKDOWN_EPL_V10,
            "gnu-agpl-v3.0" => MARKDOWN_GNU_AGPL_V30,
            "gnu-fdl-v1.3" => MARKDOWN_GNU_FDL_V13,
            "gnu-gpl-v1.0" => MARKDOWN_GNU_GPL_V10,
            "gnu-gpl-v2.0" => MARKDOWN_GNU_GPL_V20,
            "gnu-gpl-v3.0" => MARKDOWN_GNU_GPL_V30,
            "gnu-lgpl-v2.1" => MARKDOWN_GNU_LGPL_V21,
            "gnu-lgpl-v3.0" => MARKDOWN_GNU_LGPL_V30,
            "mit" => MARKDOWN_MIT,
            "mpl-v2.0" | "mpl-2.0" => MARKDOWN_MPL_V20,
            "unlicense" => MARKDOWN_UNLICENSE,
            "zlib" => MARKDOWN_ZLIB,
            "isc" => MARKDOWN_ISC,
            "bsl-1.0" => MARKDOWN_BSL_10,
            v => {
                log::warn!("Couldn't find markdown-license for: {:?}", v);
                ""
            }
        };

        let mut license_text = ltext
            .to_string()
            .replace("`<year>`", &format!("{}", current_year));
        match authors.clone() {
            None => {}
            Some(a) => {
                license_text = license_text.replace("`<copyright holders>`", &format!("{:?}", a))
            }
        }
        md_licenses.push(license_text);
    }

    md_licenses
}
