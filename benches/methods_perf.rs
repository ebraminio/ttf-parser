use ttf_parser as ttf;

fn from_data(bencher: &mut bencher::Bencher) {
    let font_data = std::fs::read("fonts/SourceSansPro-Regular.ttf").unwrap();
    bencher.iter(|| {
        bencher::black_box(ttf::Font::from_data(&font_data, 0).unwrap());
    })
}

fn outline_glyph_8_from_glyf(bencher: &mut bencher::Bencher) {
    let font_data = std::fs::read("fonts/SourceSansPro-Regular.ttf").unwrap();
    let font = ttf::Font::from_data(&font_data, 0).unwrap();
    bencher.iter(|| {
        font.outline_glyph(ttf::GlyphId(8), &mut Builder(0))
    })
}

fn outline_glyph_276_from_glyf(bencher: &mut bencher::Bencher) {
    let font_data = std::fs::read("fonts/SourceSansPro-Regular.ttf").unwrap();
    let font = ttf::Font::from_data(&font_data, 0).unwrap();
    let mut b = Builder(0);
    bencher.iter(|| {
        font.outline_glyph(ttf::GlyphId(276), &mut b)
    })
}

fn outline_glyph_8_from_cff(bencher: &mut bencher::Bencher) {
    let font_data = std::fs::read("fonts/SourceSansPro-Regular.otf").unwrap();
    let font = ttf::Font::from_data(&font_data, 0).unwrap();
    bencher.iter(|| {
        font.outline_glyph(ttf::GlyphId(8), &mut Builder(0))
    })
}

fn outline_glyph_276_from_cff(bencher: &mut bencher::Bencher) {
    let font_data = std::fs::read("fonts/SourceSansPro-Regular.otf").unwrap();
    let font = ttf::Font::from_data(&font_data, 0).unwrap();
    bencher.iter(|| {
        font.outline_glyph(ttf::GlyphId(276), &mut Builder(0))
    })
}

fn family_name(bencher: &mut bencher::Bencher) {
    let font_data = std::fs::read("fonts/SourceSansPro-Regular.ttf").unwrap();
    let font = ttf::Font::from_data(&font_data, 0).unwrap();
    bencher.iter(|| {
        bencher::black_box(font.family_name());
    })
}

struct Builder(usize);

impl ttf_parser::OutlineBuilder for Builder {
    #[inline]
    fn move_to(&mut self, _: f32, _: f32) {
        self.0 += 1;
    }

    #[inline]
    fn line_to(&mut self, _: f32, _: f32) {
        self.0 += 1;
    }

    #[inline]
    fn quad_to(&mut self, _: f32, _: f32, _: f32, _: f32) {
        self.0 += 2;
    }

    #[inline]
    fn curve_to(&mut self, _: f32, _: f32, _: f32, _: f32, _: f32, _: f32) {
        self.0 += 3;
    }

    #[inline]
    fn close(&mut self) {
        self.0 += 1;
    }
}

bencher::benchmark_group!(perf,
    from_data,
    outline_glyph_8_from_glyf,
    outline_glyph_276_from_glyf,
    outline_glyph_8_from_cff,
    outline_glyph_276_from_cff,
    family_name
);
bencher::benchmark_main!(perf);
