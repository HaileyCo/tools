use chrono::{DateTime, Utc};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum SRU {
    Info(InfoHeader, InfoContact),
    Blanketter(Vec<Blankett>),
}

impl Display for SRU {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            SRU::Info(header, contact) => fmt.write_str(&format!("{}{}", header, contact)),
            SRU::Blanketter(_) => fmt.write_str(""),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InfoHeader {
    // #PRODUKT: this field is always "SRU"
    pub product: String,

    // #MEDIAID
    pub media_id: Option<String>,

    // #SKAPAD
    pub created_at: Option<DateTime<Utc>>,

    // #PROGRAM
    pub program: Option<String>,

    // #FILNAME
    pub filename: String,
}

impl Display for InfoHeader {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fmt.write_str("#DATABESKRIVNING_START\n")?;
        fmt.write_str("#PRODUKT SRU\n")?;
        if let Some(media_id) = &self.media_id {
            fmt.write_str(&format!("#MEDIAID {}\n", media_id))?;
        }
        if let Some(created_at) = &self.created_at {
            fmt.write_str(&format!("#SKAPAD {}\n", created_at.format("%Y%m%d %H%M%S")))?;
        }
        if let Some(program) = &self.program {
            fmt.write_str(&format!("#PROGRAM {}\n", program))?;
        }
        fmt.write_str(&format!("#FILNAMN {}\n", self.filename))?;
        fmt.write_str("#DATABESKRIVNING_SLUT\n")
    }
}

#[derive(Debug, Clone)]
pub struct InfoContact {
    // #ADRESS
    pub address: Option<String>,

    // #KONTACT
    pub contact_person: Option<String>,

    // #AVDELNING
    pub department: Option<String>,

    // #EMAIL
    pub email: Option<String>,

    // #FAX
    pub fax_number: Option<String>,

    // #NAMN
    pub name: String,

    // #ORGN
    pub organization_number: String,

    // #TELEFON
    pub phone_number: Option<String>,

    // #POSTNR
    pub zip_area: String,

    // #POSTORT
    pub zip_code: u32,
}

impl Display for InfoContact {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fmt.write_str("#MEDIELEV_START\n")?;
        fmt.write_str(&format!("#ORGNR {}\n", self.organization_number))?;
        fmt.write_str(&format!("#NAMN {}\n", self.name))?;
        if let Some(address) = &self.address {
            fmt.write_str(&format!("#ADRESS {}\n", address))?;
        }
        fmt.write_str(&format!("#POSTNR {}\n", self.zip_code))?;
        fmt.write_str(&format!("#POSTORT {}\n", self.zip_area))?;
        if let Some(department) = &self.department {
            fmt.write_str(&format!("#AVDELNING {}\n", department))?;
        }
        if let Some(contact_person) = &self.contact_person {
            fmt.write_str(&format!("#KONTAKT {}\n", contact_person))?;
        }
        if let Some(email) = &self.email {
            fmt.write_str(&format!("#EMAIL {}\n", email))?;
        }
        if let Some(phone_number) = &self.phone_number {
            fmt.write_str(&format!("#TELEFON {}\n", phone_number))?;
        }
        if let Some(fax_number) = &self.fax_number {
            fmt.write_str(&format!("#FAX {}\n", fax_number))?;
        }
        fmt.write_str("#MEDIELEV_SLUT\n")
    }
}

#[derive(Debug, Clone)]
pub struct Blankett {
    // #BLANKETT
    document_id: String,
    document_appendix: Option<String>,
    filing_year: u8,
    filing_period: u8,

    // #IDENTITET
    organization_number: String,
    created_at: DateTime<Utc>,

    // #NAMN
    name: String,

    // #UPPGIFT
    fields: Vec<BlankettField>,

    // #SYSTEMINFO
    system_info: Option<String>,
}

/*
 * NOTE: this field is a little wonky since it relies on the spreadsheets
 * included in `/docs` to be validated. We'll have to figure how to automate
 * this.
 */
#[derive(Debug, Clone)]
pub struct BlankettField {
    field_code: u16,
    field_value: String,
}
