use spooky_colors;
use spooky_canvas::raw::canvas::CanvasRaw;

fn main() {
    let c = CanvasRaw::from_png("input/lola_relaxed.png");
    c.save_as_png("output/out-felipe_loaf.png");

    let cn = c.scaled_nearest(96, 36);
    cn.save_as_png("output/cn-felipe_loaf.png");

    let cb = c.scaled_bilinear(96, 128);
    cb.save_as_png("output/cb-felipe_loaf.png");

    let cn_c = cn.scaled_nearest(480, 640);
    cn_c.save_as_png("output/cn_c-felipe_loaf.png");

    let cb_c = cb.scaled_bilinear(480, 640);
    cb_c.save_as_png("output/cb_c-felipe_loaf.png");

    println!("|--------|10-------|20-------|30-------|40-------|50-------|60-------|70-------|80-------|90-------|100------|110------|120------|130------|140------|150------|160------|170------|180------|190------|200------");

    println!("{}", cn.to_img_string());

}