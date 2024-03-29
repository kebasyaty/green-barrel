// REGULAR EXPRESSIONS
// #################################################################################################

#[cfg(test)]
mod tests {
    use regex::{Regex, RegexBuilder};

    #[test]
    fn regex_validate_password() {
        let re = Regex::new(r"^[a-zA-Z0-9@#$%^&+=*!~)(]{8,256}$").unwrap();
        // invalids
        assert!(!re.is_match("1234567"));
        assert!(!re.is_match(&"`".repeat(8)));
        assert!(!re.is_match(&"№".repeat(8)));
        assert!(!re.is_match(&" ".repeat(8)));
        assert!(!re.is_match(&"-".repeat(8)));
        assert!(!re.is_match(&"_".repeat(8)));
        assert!(!re.is_match(&":".repeat(8)));
        assert!(!re.is_match(&"'".repeat(8)));
        assert!(!re.is_match(&"\"".repeat(8)));
        assert!(!re.is_match(&",".repeat(8)));
        assert!(!re.is_match(&".".repeat(8)));
        assert!(!re.is_match(&"<".repeat(8)));
        assert!(!re.is_match(&">".repeat(8)));
        assert!(!re.is_match(&"?".repeat(8)));
        assert!(!re.is_match(&"/".repeat(8)));
        assert!(!re.is_match(&"  ".repeat(8)));
        assert!(!re.is_match(""));
        assert!(!re.is_match(&"0".repeat(257)));
        // valids
        assert!(re.is_match(&"@#$%^&+=*!~)("));
        assert!(re.is_match(&"0123456789"));
        assert!(re.is_match(&"abcdefghijklmnopqrstuvwxyz"));
        assert!(re.is_match(&"ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
        assert!(re.is_match(&"zeDKs9LtfrB7Xm2"));
        assert!(re.is_match(&"0".repeat(256)));
    }

    #[test]
    fn regex_validate_phone() {
        let re = Regex::new(r"^\+?[0-9]{8,15}$").unwrap();
        // invalids
        assert!(!re.is_match("789"));
        assert!(!re.is_match("1-1-1"));
        assert!(!re.is_match("+982"));
        assert!(!re.is_match("1222333444455555"));
        assert!(!re.is_match("+1222333444455555"));
        // valids
        assert!(re.is_match("+12223334444"));
        assert!(re.is_match("+56667778888"));
        assert!(re.is_match("56667778888"));
        assert!(re.is_match("17184441122"));
        assert!(re.is_match("7184441122"));
        assert!(re.is_match("18005551234"));
        assert!(re.is_match("1231231231"));
        assert!(re.is_match("+991234567890"));
        assert!(re.is_match("+923001234567"));
        assert!(re.is_match("1234567890"));
        assert!(re.is_match("+447222555555"));
        assert!(re.is_match("0123456789"));
        assert!(re.is_match("064423933023"));
        assert!(re.is_match("09364211235"));
        assert!(re.is_match("89076543"));
        assert!(re.is_match("010123456781234"));
        assert!(re.is_match("008618311006933"));
        assert!(re.is_match("+8617888829981"));
        assert!(re.is_match("19119255642"));
        assert!(re.is_match("919578965389"));
        assert!(re.is_match("121231234"));
        assert!(re.is_match("015121231234"));
        assert!(re.is_match("0732105432"));
        assert!(re.is_match("1300333444"));
        assert!(re.is_match("+31235256677"));
        assert!(re.is_match("+310235256677"));
        assert!(re.is_match("0235256677"));
        assert!(re.is_match("+46812345678"));
        assert!(re.is_match("0812345678"));
        assert!(re.is_match("012345678"));
        assert!(re.is_match("09754845789"));
        assert!(re.is_match("9775876662"));
        assert!(re.is_match("+919456211568"));
        assert!(re.is_match("919857842356"));
        assert!(re.is_match("010123456781234"));
        assert!(re.is_match("0216254914479"));
        assert!(re.is_match("+490222139938113"));
        assert!(re.is_match("+330123456789"));
    }

    #[test]
    fn regex_validate_dated_path() {
        let re =
            Regex::new(r"(?:(?:/|\\)\d{4}(?:/|\\)\d{2}(?:/|\\)\d{2}\-barrel(?:/|\\))").unwrap();
        // invalids
        assert!(!re.is_match(""));
        assert!(!re.is_match("-barrel"));
        assert!(!re.is_match(r#"\-barrel\"#));
        assert!(!re.is_match("\\\\"));
        assert!(!re.is_match(r#"//"#));
        assert!(!re.is_match(r#"////"#));
        assert!(!re.is_match("0000-00-00"));
        assert!(!re.is_match("0000-00-00-barrel"));
        assert!(!re.is_match("000-00-00"));
        assert!(!re.is_match("00-00-00"));
        assert!(!re.is_match("0-0-0"));
        assert!(!re.is_match("/0000-00-00"));
        assert!(!re.is_match("\\0000-00-00"));
        assert!(!re.is_match("0000-00-00/"));
        assert!(!re.is_match("0000-00-00\\"));
        assert!(!re.is_match("/0000-00-00/"));
        assert!(!re.is_match("\\0000-00-00\\"));
        assert!(!re.is_match("0000-00"));
        assert!(!re.is_match("-00-00"));
        assert!(!re.is_match("-00"));
        assert!(!re.is_match("-00-"));
        assert!(!re.is_match("/0000-00/"));
        assert!(!re.is_match("\\0000-00\\"));
        assert!(!re.is_match("/-00-00/"));
        assert!(!re.is_match("\\-00-00\\"));
        assert!(!re.is_match("/-00/"));
        assert!(!re.is_match("\\-00\\"));
        assert!(!re.is_match("/-00-/"));
        assert!(!re.is_match("\\-00-\\"));
        assert!(!re.is_match(r#"\0000-00-00\"#));
        assert!(!re.is_match("\\0000-00-00\\"));
        assert!(!re.is_match(r#"\0000-00-00-barrel"#));
        assert!(!re.is_match("\\0000-00-00-barrel"));
        assert!(!re.is_match(r#"0000-00-00-barrel\"#));
        assert!(!re.is_match(r#"\0000-00\"#));
        assert!(!re.is_match(r#"\-00-00\"#));
        assert!(!re.is_match(r#"\-00\"#));
        assert!(!re.is_match(r#"\-00-\"#));
        assert!(!re.is_match(r#"\0000-00-barrel\"#));
        assert!(!re.is_match(r#"\-00-00-barrel\"#));
        assert!(!re.is_match(r#"\-00-barrel\"#));
        // valids
        assert!(re.is_match("/0000/00/00-barrel/"));
        assert!(re.is_match("\\0000\\00\\00-barrel\\"));
        assert!(re.is_match("./2022/10/27-barrel/"));
        assert!(re.is_match(".\\2022\\10\\27-barrel\\"));
        assert!(re.is_match("/media/0000/00/00-barrel/"));
        assert!(re.is_match("\\media\\0000\\00\\00-barrel\\"));
        assert!(re.is_match("./media/2022/10/27-barrel/"));
        assert!(re.is_match(".\\media\\2022\\10/27-barrel\\"));
        assert!(re.is_match("/media/files/0000/00/00-barrel/"));
        assert!(re.is_match("\\media\\files\\0000\\00\\00-barrel\\"));
        assert!(re.is_match("./media/images/2022/10/27-barrel/"));
        assert!(re.is_match(".\\media\\images\\2022\\10\\27-barrel\\"));
        assert!(re.is_match("/media/files/0000/00/00-barrel/123e4567-e89b-12d3-a456-426655440000"));
        assert!(re
            .is_match("\\media\\files\\0000\\00\\00-barrel\\123e4567-e89b-12d3-a456-426655440000"));
        assert!(re.is_match("./media/files/0000/00/00-barrel/123e4567-e89b-12d3-a456-426655440000"));
        assert!(re.is_match(".\\files\\0000\\00\\00-barrel\\123e4567-e89b-12d3-a456-426655440000"));
        assert!(
            re.is_match(r#"/media/images/2022/10/27-barrel/123e4567-e89b-12d3-a456-426655440000/"#)
        );
        assert!(re.is_match(
            "\\media\\images\\2022\\10\\27-barrel\\123e4567-e89b-12d3-a456-426655440000\\"
        ));
        assert!(re
            .is_match(r#"./media/images/2022/10/27-barrel/123e4567-e89b-12d3-a456-426655440000/"#));
        assert!(re.is_match(
            ".\\media\\images\\2022\\10\\27-barrel\\123e4567-e89b-12d3-a456-426655440000\\"
        ));
    }

    #[test]
    fn regex_validate_color_code() {
        let re = RegexBuilder::new(
            r"^(?:#|0x)(?:[a-f0-9]{3}|[a-f0-9]{6}|[a-f0-9]{8})\b|(?:rgb|hsl)a?\([^\)]*\)$",
        )
        .case_insensitive(true)
        .build()
        .unwrap();
        // invalids
        assert!(!re.is_match("#f2ewq"));
        assert!(!re.is_match(""));
        // valids
        assert!(re.is_match("#fff"));
        assert!(re.is_match("#f2f2f2"));
        assert!(re.is_match("#F2F2F2"));
        assert!(re.is_match("#00000000"));
        assert!(re.is_match("rgb(255,0,24)"));
        assert!(re.is_match("rgb(255, 0, 24)"));
        assert!(re.is_match("rgba(255, 0, 24, .5)"));
        assert!(re.is_match("rgba(#fff, .5)"));
        assert!(re.is_match("rgba(#fff,.5)"));
        assert!(re.is_match("rgba(#FFF, .5)"));
        assert!(re.is_match("hsl(120, 100%, 50%)"));
        assert!(re.is_match("hsl(120,100%,50%)"));
        assert!(re.is_match("hsla(170, 23%, 25%, 0.2)"));
        assert!(re.is_match("hsla(170,23%,25%,0.2)"));
        assert!(re.is_match("0x00ffff"));
        assert!(re.is_match("0x00FFFF"));
    }

    #[test]
    fn regex_replace_color() {
        let re = RegexBuilder::new(
            r"(?P<color>(?:#|0x)(?:[a-f0-9]{3}|[a-f0-9]{6}|[a-f0-9]{8})\b|(?:rgb|hsl)a?\([^\)]*\))",
        )
        .case_insensitive(true)
        .build()
        .unwrap();

        // invalids
        let before = "Lorem ipsum dolor #f2ewq sit amet.";
        let after = re.replace_all(before, r#"<div style="background-color:$color;"></div>"#);
        assert_ne!(
            after,
            r#"Lorem ipsum dolor <div style="background-color:#fff;"></div> sit amet."#
        );

        // valids
        let before = "Lorem ipsum dolor sit amet.";
        let after = re.replace_all(before, r#"<div style="background-color:$color;"></div>"#);
        assert_eq!(after, r#"Lorem ipsum dolor sit amet."#);
        //
        let samples: Vec<&str> = vec![
            "#fff",
            "#f2f2f2",
            "#F2F2F2",
            "#00000000",
            "rgb(255,0,24)",
            "rgb(255, 0, 24)",
            "rgba(255, 0, 24, .5)",
            "rgba(#fff, .5)",
            "rgba(#fff,.5)",
            "rgba(#FFF, .5)",
            "hsl(120, 100%, 50%)",
            "hsl(120,100%,50%)",
            "hsla(170, 23%, 25%, 0.2)",
            "hsla(170,23%,25%,0.2)",
            "0x00ffff",
            "0x00FFFF",
        ];
        for sample in samples {
            // 1
            let before = format!("Lorem ipsum dolor {} sit amet.", sample);
            let after = re.replace_all(
                before.as_str(),
                r#"<div style="background-color:$color;"></div>"#,
            );
            assert_eq!(
                after,
                format!(
                    r#"Lorem ipsum dolor <div style="background-color:{};"></div> sit amet."#,
                    sample
                )
            );
            // 2
            let before = format!("Lorem ipsum {} dolor {} sit amet.", sample, sample);
            let after = re.replace_all(
                before.as_str(),
                r#"<div style="background-color:$color;"></div>"#,
            );
            assert_eq!(
                after,
                format!(
                    r#"Lorem ipsum <div style="background-color:{};"></div> dolor <div style="background-color:{};"></div> sit amet."#,
                    sample, sample
                )
            );
        }
    }

    #[test]
    fn regex_validate_time() {
        let re = RegexBuilder::new(r"^(?:[01]\d|2[0-3]):[0-5]\d$")
            .build()
            .unwrap();
        // invalids
        assert!(!re.is_match("00:00:00"));
        assert!(!re.is_match("0:00"));
        assert!(!re.is_match("00:0"));
        assert!(!re.is_match("0:0"));
        assert!(!re.is_match("0:"));
        assert!(!re.is_match(":0"));
        assert!(!re.is_match(":"));
        assert!(!re.is_match("0"));
        assert!(!re.is_match(""));
        assert!(!re.is_match("24:00"));
        assert!(!re.is_match("23:60"));
        assert!(!re.is_match("-1:00"));
        assert!(!re.is_match("00:-1"));
        assert!(!re.is_match(""));
        // valids
        assert!(re.is_match("00:00"));
        assert!(re.is_match("23:59"));
    }

    #[test]
    fn regex_validate_datetime() {
        let re = RegexBuilder::new(
            r"^(?:[1-9]\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)T(?:[01]\d|2[0-3]):[0-5]\d$"
        )
        .build()
        .unwrap();
        // invalids
        assert!(!re.is_match("0000-00-00T00:00"));
        assert!(!re.is_match("0000-00-00T00:00Z"));
        assert!(!re.is_match("0000-01-01T00:00"));
        assert!(!re.is_match("1900-01-01T00:00:00"));
        assert!(!re.is_match("1900-00-00T00:00"));
        assert!(!re.is_match("1900-13-01T00:00"));
        assert!(!re.is_match("1900-01-32T00:00"));
        assert!(!re.is_match("1900-01-01T24:00"));
        assert!(!re.is_match("1900-01-01T00:60"));
        assert!(!re.is_match("197-01-01T00:00"));
        assert!(!re.is_match("1900-1-01T00:00"));
        assert!(!re.is_match("1900-01-1T00:00"));
        assert!(!re.is_match("1900-01-01T0:00"));
        assert!(!re.is_match("1900-01-01T00:0"));
        assert!(!re.is_match("19000-01-01T00:00"));
        assert!(!re.is_match("1900-010-01T00:00"));
        assert!(!re.is_match("1900-01-010T00:00"));
        assert!(!re.is_match("1900-01-01T000:00"));
        assert!(!re.is_match("1900-01-01T00:000"));
        assert!(!re.is_match("1900/01/01T00:00"));
        assert!(!re.is_match("1900.01.01T00:00"));
        assert!(!re.is_match("1900-01-01 00:00"));
        assert!(!re.is_match("01011900"));
        assert!(!re.is_match("01/01/1900"));
        assert!(!re.is_match("01.01.1900"));
        assert!(!re.is_match("1900-01-01"));
        assert!(!re.is_match("1900-01-01 00:00"));
        assert!(!re.is_match("1900-01-01T00:00Z"));
        assert!(!re.is_match("1901-02-29T00:00"));
        assert!(!re.is_match("1995-02-29T00:00"));
        assert!(!re.is_match("1975-02-29T00:00"));
        assert!(!re.is_match("1951-02-29T00:00"));
        assert!(!re.is_match("1949-02-29T00:00"));
        assert!(!re.is_match("1942-02-29T00:00"));
        assert!(!re.is_match("1923-02-29T00:00"));
        assert!(!re.is_match("1921-02-29T00:00"));
        assert!(!re.is_match("1917-02-29T00:00"));
        assert!(!re.is_match("1913-02-29T00:00"));
        assert!(!re.is_match("1909-02-29T00:00"));
        assert!(!re.is_match("2002-02-29T00:00"));
        assert!(!re.is_match("2005-02-29T00:00"));
        assert!(!re.is_match("2009-02-29T00:00"));
        assert!(!re.is_match("2010-02-29T00:00"));
        assert!(!re.is_match("2011-02-29T00:00"));
        assert!(!re.is_match("2019-02-29T00:00"));
        assert!(!re.is_match("2023-02-29T00:00"));
        assert!(!re.is_match("1900-04-31T00:00"));
        assert!(!re.is_match("1900-06-31T00:00"));
        assert!(!re.is_match("1900-09-31T00:00"));
        assert!(!re.is_match("1900-11-31T00:00"));
        assert!(!re.is_match(""));
        // valids
        assert!(re.is_match("1900-01-31T00:00"));
        assert!(re.is_match("1904-02-29T00:00"));
        assert!(re.is_match("1996-02-29T00:00"));
        assert!(re.is_match("1972-02-29T00:00"));
        assert!(re.is_match("1952-02-29T00:00"));
        assert!(re.is_match("1948-02-29T00:00"));
        assert!(re.is_match("1940-02-29T00:00"));
        assert!(re.is_match("1924-02-29T00:00"));
        assert!(re.is_match("1920-02-29T00:00"));
        assert!(re.is_match("1916-02-29T00:00"));
        assert!(re.is_match("1912-02-29T00:00"));
        assert!(re.is_match("1908-02-29T00:00"));
        assert!(re.is_match("2000-02-29T00:00"));
        assert!(re.is_match("2004-02-29T00:00"));
        assert!(re.is_match("2008-02-29T00:00"));
        assert!(re.is_match("2012-02-29T00:00"));
        assert!(re.is_match("2016-02-29T00:00"));
        assert!(re.is_match("2020-02-29T00:00"));
        assert!(re.is_match("2024-02-29T00:00"));
        assert!(re.is_match("1900-03-31T00:00"));
        assert!(re.is_match("1900-04-30T00:00"));
        assert!(re.is_match("1900-05-31T00:00"));
        assert!(re.is_match("1900-06-30T00:00"));
        assert!(re.is_match("1900-07-31T00:00"));
        assert!(re.is_match("1900-08-31T00:00"));
        assert!(re.is_match("1900-09-30T00:00"));
        assert!(re.is_match("1900-10-31T00:00"));
        assert!(re.is_match("1900-11-30T00:00"));
        assert!(re.is_match("1900-12-31T00:00"));
        assert!(re.is_match("1000-01-01T00:00"));
        assert!(re.is_match("1900-01-01T00:00"));
        assert!(re.is_match("9999-12-31T23:59"));
        assert!(re.is_match("2020-10-16T12:52"));
    }

    #[test]
    fn regex_validate_date() {
        let re = RegexBuilder::new(
            r"^(?:[1-9]\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)$"
        )
            .build()
            .unwrap();
        // invalids
        assert!(!re.is_match("0000-00-00"));
        assert!(!re.is_match("1900-00-00"));
        assert!(!re.is_match("1900-13-01"));
        assert!(!re.is_match("1900-01-32"));
        assert!(!re.is_match("197-01-01"));
        assert!(!re.is_match("1900-1-01"));
        assert!(!re.is_match("1900-01-1"));
        assert!(!re.is_match("19000-01-01"));
        assert!(!re.is_match("1900-010-01"));
        assert!(!re.is_match("1900-01-010"));
        assert!(!re.is_match("1900/01/01"));
        assert!(!re.is_match("1900.01.01"));
        assert!(!re.is_match("01011900"));
        assert!(!re.is_match("01/01/1900"));
        assert!(!re.is_match("01.01.1900"));
        assert!(!re.is_match("1900-01-01 00:00"));
        assert!(!re.is_match("1900-01-01T00:00"));
        assert!(!re.is_match("1900-01-01 00:00"));
        assert!(!re.is_match("1900-01-01T00:00"));
        assert!(!re.is_match("1900-01-01T00:00:00Z"));
        assert!(!re.is_match("9999-12-31T23:59:59"));
        assert!(!re.is_match("1900-01-01T00:00"));
        assert!(!re.is_match("1901-02-29"));
        assert!(!re.is_match("2002-02-29"));
        assert!(!re.is_match("2005-02-29"));
        assert!(!re.is_match("2009-02-29"));
        assert!(!re.is_match("2010-02-29"));
        assert!(!re.is_match("2011-02-29"));
        assert!(!re.is_match("2019-02-29"));
        assert!(!re.is_match("2023-02-29"));
        assert!(!re.is_match("1995-02-29"));
        assert!(!re.is_match("1975-02-29"));
        assert!(!re.is_match("1951-02-29"));
        assert!(!re.is_match("1949-02-29"));
        assert!(!re.is_match("1942-02-29"));
        assert!(!re.is_match("1923-02-29"));
        assert!(!re.is_match("1921-02-29"));
        assert!(!re.is_match("1917-02-29"));
        assert!(!re.is_match("1913-02-29"));
        assert!(!re.is_match("1909-02-29"));
        assert!(!re.is_match("1900-04-31"));
        assert!(!re.is_match("1900-06-31"));
        assert!(!re.is_match("1900-09-31"));
        assert!(!re.is_match("1900-11-31"));
        assert!(!re.is_match(""));
        // valids
        assert!(re.is_match("1900-01-31"));
        assert!(re.is_match("1904-02-29"));
        assert!(re.is_match("1996-02-29"));
        assert!(re.is_match("1972-02-29"));
        assert!(re.is_match("1952-02-29"));
        assert!(re.is_match("1948-02-29"));
        assert!(re.is_match("1940-02-29"));
        assert!(re.is_match("1924-02-29"));
        assert!(re.is_match("1920-02-29"));
        assert!(re.is_match("1916-02-29"));
        assert!(re.is_match("1912-02-29"));
        assert!(re.is_match("1908-02-29"));
        assert!(re.is_match("2000-02-29"));
        assert!(re.is_match("2004-02-29"));
        assert!(re.is_match("2008-02-29"));
        assert!(re.is_match("2012-02-29"));
        assert!(re.is_match("2016-02-29"));
        assert!(re.is_match("2020-02-29"));
        assert!(re.is_match("2024-02-29"));
        assert!(re.is_match("1900-03-31"));
        assert!(re.is_match("1900-04-30"));
        assert!(re.is_match("1900-05-31"));
        assert!(re.is_match("1900-06-30"));
        assert!(re.is_match("1900-07-31"));
        assert!(re.is_match("1900-08-31"));
        assert!(re.is_match("1900-09-30"));
        assert!(re.is_match("1900-10-31"));
        assert!(re.is_match("1900-11-30"));
        assert!(re.is_match("1900-12-31"));
        assert!(re.is_match("1000-01-01"));
        assert!(re.is_match("1900-01-01"));
        assert!(re.is_match("9999-12-31"));
        assert!(re.is_match("2020-10-15"));
    }
}
