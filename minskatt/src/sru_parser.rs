use crate::sru;
use anyhow::anyhow;

lalrpop_mod!(sru_grammar);

pub fn parse(s: String) -> Result<sru::SRU, anyhow::Error> {
    sru_grammar::SRUParser::new()
        .parse(&s)
        .map_err(|parser_error| anyhow!("{:?}", parser_error))
}

#[cfg(test)]
mod tests {
    lalrpop_mod!(sru_grammar);

    use crate::sru_parser::*;

    #[test]
    fn parse_info() {
        let res = parse(
            r"#DATABESKRIVNING_START
#PRODUKT SRU
#MEDIAID DISK_12
#SKAPAD 20130428 174557
#PROGRAM SRUDEKLARATION 1.4
#FILNAMN blanketter.sru
#DATABESKRIVNING_SLUT
#MEDIELEV_START
#ORGNR 191111111111
#NAMN Databokföraren
#ADRESS BOX 159
#POSTNR 12345
#POSTORT SKATTSTAD
#AVDELNING Ekonomi
#KONTAKT KARL KARLSSON
#EMAIL kk@Databokföraren
#TELEFON 08-2121212
#FAX 08-1212121
#MEDIELEV_SLUT
        "
            .to_string(),
        )
        .unwrap();
        assert_eq!(format!("{:?}", res),
            "Info(InfoHeader { product: \"SRU\", media_id: Some(\"DISK_12\"), created_at: Some(2013-04-28T17:45:57Z), program: Some(\"SRUDEKLARATION 1.4\"), filename: \"blanketter.sru\" }, InfoContact { address: Some(\"BOX 159\"), contact_person: Some(\"KARL KARLSSON\"), department: Some(\"Ekonomi\"), email: Some(\"kk@Databokföraren\"), fax_number: Some(\"08-1212121\"), name: \"Databokföraren\", organization_number: \"191111111111\", phone_number: Some(\"08-2121212\"), zip_area: \"SKATTSTAD\", zip_code: 12345 })");
    }
    #[test]
    fn parse_blanketter() {
        let res = parse(
            // NOTE @malimnnnikk find a blanketter file to add as an example test here
            r"
#BLANKETT N7-2020P1
#IDENTITET 199704110100 20200911 123456
#NAMN Malin Männikkö 
#SYSTEMINFO markedasdone 20200908 
#UPPGIFT 7011 010102019
#UPPGIFT 7012 31122019
#BLANKETTSLUT
#FIL_SLUT
        "
            .to_string(),
        )
        .unwrap();
        assert_eq!(format!("{:?}", res),
            "Info(InfoHeader { product: \"SRU\", media_id: Some(\"DISK_12\"), created_at: Some(2013-04-28T17:45:57Z), program: Some(\"SRUDEKLARATION 1.4\"), filename: \"blanketter.sru\" }, InfoContact { address: Some(\"BOX 159\"), contact_person: Some(\"KARL KARLSSON\"), department: Some(\"Ekonomi\"), email: Some(\"kk@Databokföraren\"), fax_number: Some(\"08-1212121\"), name: \"Databokföraren\", organization_number: \"191111111111\", phone_number: Some(\"08-2121212\"), zip_area: \"SKATTSTAD\", zip_code: 12345 })");
    }
}
