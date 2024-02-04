use darkay::prelude::*;
use peroxide::fuga::*;

#[allow(non_snake_case)]
#[test]
fn P_surv_test() {
    let alp = ALP(10f64, 1e-15);
    let t_e = TCMB;
    let t_vec = logspace(TCMB.log10(), 16, 200, 10);
    let E = 100f64;

    let p_surv = t_vec.fmap(|t| alp.P_surv(Photon, t, t_e));
    let p_surv_tv = t_vec.fmap(|t| alp.P_surv_tv(Photon, t, t_e, E));

    let mut df = DataFrame::new(vec![]);
    df.push("t", Series::new(t_vec));
    df.push("p_surv", Series::new(p_surv));
    df.push("p_surv_tv", Series::new(p_surv_tv));

    df.print();

    df.write_parquet("tests/data/tv_basic.parquet", CompressionOptions::Uncompressed).unwrap();
}
