#![feature(test)]

extern crate test;
extern crate bbcode;

#[cfg(test)]
mod tests {
    use bbcode::BBCode;
    use test::Bencher;

    #[bench]
    fn bench_table_full(b: &mut Bencher) {
        b.iter(|| {
            r"[table]
              [tr]
                [th]Name[/th]
                [th]Date[/th]
              [/tr]
              [tr]
                [td]Test[/td]
                [td]Yesterday[/td]
              [/tr]
[/table]"
                .as_html()
        })
    }

    #[bench]
    fn bench_table_empty(b: &mut Bencher) {
        b.iter(|| r"[table][/table]".as_html())
    }

    #[bench]
    fn bench_code(b: &mut Bencher) {
        b.iter(|| {
            r"[code][table]
              [tr]
                [th]Name[/th]
                [th]Date[/th]
              [/tr]
              [tr]
                [td]Test[/td]
                [td]Yesterday[/td]
              [/tr]
[/table][/code]"
                .as_html()
        })
    }
}
