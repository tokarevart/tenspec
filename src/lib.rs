use geoconv::*;
use std::fs;
use std::process::{Command, Stdio};
use serde::{Deserialize, Serialize};

mod generic_tess {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct Tessellation {
        pub n: String,
        pub domain: Option<String>,
        pub morpho: Option<String>,
        pub morphooptiini: Option<String>,
        pub reg: Option<String>,
        pub fmax: Option<String>,
        pub sel: Option<String>,
        pub mloop: Option<String>,
        pub output: Option<String>,
        pub format: Option<String>,
    }

    impl Tessellation {
        pub fn new(n: &str) -> Self {
            Self {
                n: n.into(),
                domain: None,
                morpho: None,
                morphooptiini: None,
                reg: None,
                fmax: None,
                sel: None,
                mloop: None,
                output: None,
                format: None,
            }
        }

        pub fn domain(&mut self, v: &str) -> &mut Self  {
            self.domain = Some(v.into());
            self
        }

        pub fn morpho(&mut self, v: &str) -> &mut Self  {
            self.morpho = Some(v.into());
            self
        }

        pub fn morphooptiini(&mut self, v: &str) -> &mut Self  {
            self.morphooptiini = Some(v.into());
            self
        }

        pub fn reg(&mut self, v: &str) -> &mut Self {
            if v != "0" && v != "1" {
                panic!();
            }
            self.reg = Some(v.into());
            self
        }

        pub fn fmax(&mut self, v: &str) -> &mut Self {
            self.fmax = Some(v.into());
            self
        }

        pub fn sel(&mut self, v: &str) -> &mut Self {
            self.sel = Some(v.into());
            self
        }

        pub fn mloop(&mut self, v: &str) -> &mut Self {
            self.mloop = Some(v.into());
            self
        }

        pub fn output(&mut self, v: &str) -> &mut Self  {
            self.output = Some(v.into());
            self
        }

        pub fn format(&mut self, v: &str) -> &mut Self {
            self.format = Some(v.into());
            self
        }

        pub fn run(&self) {
            let mut args = vec!["-T".to_owned()];
            let mut ext_args = |x: &str, y: &Option<String>| {
                if let Some(v) = y {
                    args.extend_from_slice(&[x.to_owned(), v.to_owned()]);
                }
            };
            ext_args("-n".into(), &Some(self.n.clone()));
            ext_args("-domain".into(), &self.domain);
            ext_args("-morpho".into(), &self.morpho);
            ext_args("-morphooptiini".into(), &self.morphooptiini);
            ext_args("-reg".into(), &self.reg);
            ext_args("-fmax".into(), &self.fmax);
            ext_args("-sel".into(), &self.sel);
            ext_args("-mloop".into(), &self.mloop);
            ext_args("-o".into(), &self.output);
            ext_args("-format".into(), &self.format);
            Command::new("neper").args(["--rcfile", "none"].iter())
                                    .args(args)
                                    .stdout(Stdio::inherit())
                                    .stderr(Stdio::inherit())
                                    .output()
                                    .unwrap();
        }
    }
}
use generic_tess::*;

#[derive(Debug, Clone)]
pub struct Tess{
    tess: Tessellation, 
    dims: SpecDims, 
    n: String,
}

impl Tess {
    pub fn new(config: Config) -> Self {
        let Config{ dims, n } = config;
        let mut tess = Tessellation::new(&n);
        let halfx = dims.r1;
        let halfy = dims.r2;
        let halfz = dims.l1;
        let domain = format!(
            "cube({},{},{}):translate({},{},{})", 
            2.0 * halfx, 2.0 * halfy, 2.0 * halfz,
            -halfx, -halfy, -halfz,
        );
        tess.morpho("graingrowth".into())
            .domain(&domain)
            .output("tenspec-data/tenspec-tess".into())
            .format("tess".into());
        Self{ tess, dims, n }
    }

    pub fn run(&self) {
        Config{ dims: self.dims, n: self.n.clone() }.serialize_to_file();
        self.tess.run()
    }
}

#[derive(Debug, Clone)]
pub struct Reg(Tessellation);

impl Reg {
    pub fn new() -> Self {
        let Config{ dims, n } = Config::deserialize_from_file();
        let mut tess = Tessellation::new(&n);
        let halfx = dims.r1;
        let halfy = dims.r2;
        let halfz = dims.l1;
        let domain = format!(
            "cube({},{},{}):translate({},{},{})", 
            2.0 * halfx, 2.0 * halfy, 2.0 * halfz,
            -halfx, -halfy, -halfz,
        );
        tess.reg("1".into())
            .morphooptiini("coo:file(tenspec-data/tenspec-tess.tess)".into())
            .domain(&domain)
            .output("tenspec-data/tenspec-tess-reg".into())
            .format("geo".into());
        Self(tess)
    }

    pub fn fmax(&mut self, v: &str) -> &mut Self {
        self.0.fmax(v);
        self
    }

    pub fn sel(&mut self, v: &str) -> &mut Self {
        self.0.sel(v);
        self
    }

    pub fn mloop(&mut self, v: &str) -> &mut Self {
        self.0.mloop(v);
        self
    }

    pub fn run(&self) {
        self.0.run();
        Self::convert_geo();
    }

    fn convert_geo() {
        let file = GeoFile::open("tenspec-data/tenspec-tess-reg.geo").unwrap();
        let mut geom = Geometry::from(file);
        geom.clear(GeoElemKind::PhysicalSurface);
        let stags: Vec<u64> = geom.tags(GeoElemKind::Surface).map(|x| *x).collect();
        for stag in stags {
            geom.correct_surface_flatness(stag).unwrap();
        }
        let mut file = OccFile::create("tenspec-data/tenspec.geo").unwrap();
        file.write_geometry(&geom).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    script: String,
}

impl Mesh {
    pub fn new(cl: &str, output: &str) -> Self {
        let Config{ dims, .. } = Config::deserialize_from_file();
        let script = [
            format!("var_l1 = {};", dims.l1),
            format!("var_l2 = {};", dims.l2),
            format!("var_le = {};", dims.le),
            format!("var_r1 = {};", dims.r1),
            format!("var_r2 = {};", dims.r2),
            format!("var_r3 = {};", dims.r3),
            format!("var_cl = {};", cl),
            include_str!("../script/script-before-ext.geo").to_owned(),
            include_str!("../script/tenspec-ext.geo").to_owned(),
            include_str!("../script/script-after-ext.geo").to_owned(),
            format!("Save \"../{}\";", output),
        ].join("\n");
        Self { script }
    }

    pub fn run(&self) {
        fs::create_dir_all("tenspec-data").unwrap();
        fs::write("tenspec-data/script.geo", &self.script).unwrap();
        let args = vec!["tenspec-data/script.geo", "-"];
        Command::new("gmsh").args(args)
                            .stdout(Stdio::inherit())
                            .stderr(Stdio::inherit())
                            .output()
                            .unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct SpecDims {
    pub l1: f64, 
    pub l2: f64, 
    pub le: f64,
    pub r1: f64, 
    pub r2: f64, 
    pub r3: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub dims: SpecDims,
    pub n: String,
}

impl Config {
    pub fn serialize_to_file(&self) {
        let ser = serde_json::to_string(self).unwrap();
        fs::create_dir_all("tenspec-data").unwrap();
        fs::write("tenspec-data/config.json", ser).unwrap();
    }

    pub fn deserialize_from_file() -> Self {
        let de = fs::read_to_string("tenspec-data/config.json").unwrap();
        serde_json::from_str(&de).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tess() {
        let dims = SpecDims{
            l1: 25.0, 
            l2: 8.0, 
            le: 1.0,
            r1: 6.0, 
            r2: 2.0, 
            r3: 6.0,
        };
        let n = 20.to_string();
        Tess::new(Config{ dims, n }).run();
    }

    #[test]
    fn reg() {
        Reg::new()
            .fmax("20")
            .sel("3")
            .mloop("5")
            .run();
    }

    #[test]
    fn mesh() {
        let cl = 3.0.to_string();
        let output = "tenspec.msh";
        Mesh::new(&cl, output).run();
    }
}
